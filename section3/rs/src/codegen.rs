extern crate llvm_sys;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::{CStr, CString};

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::{
    LLVM_InitializeAllAsmParsers, LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetInfos,
    LLVM_InitializeAllTargetMCs, LLVM_InitializeAllTargets,
};
use llvm_sys::target_machine::*;

use super::parser::{Expr, Function, Program, Type};

fn cstr(s: &str) -> Cow<CStr> {
    Cow::from(CString::new(s).expect("works"))
}

struct LLVM {
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
    named_values: HashMap<String, LLVMValueRef>,
}

impl LLVM {
    fn new() -> LLVM {
        let name = cstr("my cool jit");
        unsafe {
            let ctx = LLVMContextCreate();
            let builder = LLVMCreateBuilder();
            let module = LLVMModuleCreateWithNameInContext(name.as_ptr(), ctx);
            LLVM {
                ctx,
                builder,
                module,
                named_values: HashMap::new(),
            }
        }
    }
}

impl Expr {
    fn codegen(&self, llvm: &mut LLVM) -> Result<LLVMValueRef, String> {
        match self {
            Expr::Int { value } => {
                let ty = unsafe { LLVMInt32TypeInContext(llvm.ctx) };
                let val = unsafe { LLVMConstInt(ty, *value as u64, 0) };
                Ok(val)
            }
        }
    }
}

impl Function {
    fn codegen(&self, llvm: &mut LLVM) -> Result<LLVMValueRef, String> {
        let ret_type = match self.ret_type {
            Type::Int => unsafe { LLVMInt32TypeInContext(llvm.ctx) },
            Type::Void => unsafe { LLVMVoidTypeInContext(llvm.ctx) },
        };
        let mut arg_types = Vec::new();
        for arg in &self.args {
            let ty = match arg.ty {
                Type::Int => unsafe { LLVMInt32TypeInContext(llvm.ctx) },
                Type::Void => unsafe { LLVMVoidTypeInContext(llvm.ctx) },
            };
            arg_types.push(ty);
        }
        let fn_type = unsafe {
            LLVMFunctionType(ret_type, arg_types.as_mut_ptr(), arg_types.len() as u32, 0)
        };
        let name = cstr(&self.name);
        let fn_value = unsafe { LLVMAddFunction(llvm.module, name.as_ptr(), fn_type) };
        let bb =
            unsafe { LLVMAppendBasicBlockInContext(llvm.ctx, fn_value, cstr("entry").as_ptr()) };
        unsafe { LLVMPositionBuilderAtEnd(llvm.builder, bb) };
        for (i, arg) in self.args.iter().enumerate() {
            let name = cstr(&arg.name);
            let val = unsafe { LLVMGetParam(fn_value, i as u32) };
            unsafe { LLVMSetValueName2(val, name.as_ptr(), arg.name.len()) };
            llvm.named_values.insert(arg.name.clone(), val);
        }
        let ret_val = self.ret_expr.codegen(llvm)?;
        unsafe { LLVMBuildRet(llvm.builder, ret_val) };
        Ok(fn_value)
    }
}

impl Program {
    fn codegen(&self, llvm: &mut LLVM) -> Result<LLVMValueRef, String> {
        let mut ir: Result<*mut llvm_sys::LLVMValue, String> =
            Err("No functions in program".to_string());
        for func in &self.functions {
            ir = func.codegen(llvm);
        }
        ir
    }
}

pub fn codegen(program: Program) {
    let mut llvm = LLVM::new();
    unsafe {
        let ir = program.codegen(&mut llvm).unwrap();
        LLVMDumpValue(ir);

        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmParsers();
        LLVM_InitializeAllAsmPrinters();

        let target_triple = LLVMCreateMessage(cstr("armv7-unknown-linux-gnueabi").as_ptr());
        let mut err_string = std::mem::MaybeUninit::uninit();
        let mut target = std::ptr::null_mut();
        let ok = llvm_sys::target_machine::LLVMGetTargetFromTriple(
            target_triple,
            &mut target,
            err_string.as_mut_ptr(),
        );
        if ok > 0 {
            println!("Error: {:?}", CStr::from_ptr(err_string.assume_init()));
            return;
        }
        println!("Target: {:?}", target);

        let cpu = LLVMCreateMessage(cstr("generic").as_ptr());
        let features = LLVMCreateMessage(cstr("").as_ptr());
        let target_machine = LLVMCreateTargetMachine(
            target,
            target_triple,
            cpu,
            features,
            LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
            LLVMRelocMode::LLVMRelocDefault,
            LLVMCodeModel::LLVMCodeModelDefault,
        );
        println!("Target machine: {:?}", target_machine);

        let filename = LLVMCreateMessage(cstr("a.out").as_ptr());
        err_string = std::mem::MaybeUninit::uninit();
        let ok = LLVMTargetMachineEmitToFile(
            target_machine,
            llvm.module,
            filename,
            LLVMCodeGenFileType::LLVMAssemblyFile,
            err_string.as_mut_ptr(),
        );
        if ok > 0 {
            println!("Error: {:?}", CStr::from_ptr(err_string.assume_init()));
        }
        println!("Done");
    }
}

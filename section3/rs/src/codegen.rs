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
use llvm_sys::LLVMIntPredicate::*;

use super::constants::*;
use super::parser::{Expr, Function, Program, Type};

fn cstr(s: &str) -> Cow<CStr> {
    Cow::from(CString::new(s).expect("works"))
}

struct Scoped {
    ty: LLVMTypeRef,
    val: LLVMValueRef,
}

struct LLVM {
    ctx: LLVMContextRef,
    builder: LLVMBuilderRef,
    module: LLVMModuleRef,
    locals: HashMap<String, Scoped>,
    func: LLVMValueRef,
    ret_block: LLVMBasicBlockRef,
    ret_val: LLVMValueRef,
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
                locals: HashMap::new(),
                func: std::ptr::null_mut(),
                ret_block: std::ptr::null_mut(),
                ret_val: std::ptr::null_mut(),
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
            Expr::BinOp { lhs, rhs, op } => {
                let lhsval = lhs.codegen(llvm)?;
                let rhsval = rhs.codegen(llvm)?;
                match op {
                    Op::Add => {
                        let val = unsafe {
                            LLVMBuildAdd(llvm.builder, lhsval, rhsval, cstr("addtmp").as_ptr())
                        };
                        Ok(val)
                    }
                    Op::Sub => {
                        let val = unsafe {
                            LLVMBuildSub(llvm.builder, lhsval, rhsval, cstr("subtmp").as_ptr())
                        };
                        Ok(val)
                    }
                    Op::Mul => {
                        let val = unsafe {
                            LLVMBuildMul(llvm.builder, lhsval, rhsval, cstr("multmp").as_ptr())
                        };
                        Ok(val)
                    }
                    Op::Div => {
                        let val = unsafe {
                            LLVMBuildSDiv(llvm.builder, lhsval, rhsval, cstr("divtmp").as_ptr())
                        };
                        Ok(val)
                    }
                    Op::Le => {
                        let val = unsafe {
                            LLVMBuildICmp(
                                llvm.builder,
                                LLVMIntSLE,
                                lhsval,
                                rhsval,
                                cstr("cmptmp").as_ptr(),
                            )
                        };
                        Ok(val)
                    }
                    Op::Ge => {
                        let val = unsafe {
                            LLVMBuildICmp(
                                llvm.builder,
                                LLVMIntSGE,
                                lhsval,
                                rhsval,
                                cstr("cmptmp").as_ptr(),
                            )
                        };
                        Ok(val)
                    }
                    Op::Lt => {
                        let val = unsafe {
                            LLVMBuildICmp(
                                llvm.builder,
                                LLVMIntSLT,
                                lhsval,
                                rhsval,
                                cstr("cmptmp").as_ptr(),
                            )
                        };
                        Ok(val)
                    }
                    Op::Gt => {
                        let val = unsafe {
                            LLVMBuildICmp(
                                llvm.builder,
                                LLVMIntSGT,
                                lhsval,
                                rhsval,
                                cstr("cmptmp").as_ptr(),
                            )
                        };
                        Ok(val)
                    }
                    Op::Eq => {
                        let val = unsafe {
                            LLVMBuildICmp(
                                llvm.builder,
                                LLVMIntEQ,
                                lhsval,
                                rhsval,
                                cstr("cmptmp").as_ptr(),
                            )
                        };
                        Ok(val)
                    }
                    Op::Ne => {
                        let val = unsafe {
                            LLVMBuildICmp(
                                llvm.builder,
                                LLVMIntNE,
                                lhsval,
                                rhsval,
                                cstr("cmptmp").as_ptr(),
                            )
                        };
                        Ok(val)
                    }
                    Op::Assign => {
                        let val = unsafe { LLVMBuildStore(llvm.builder, lhsval, rhsval) };
                        Ok(val)
                    }
                    Op::And => {
                        let val = unsafe {
                            LLVMBuildAnd(llvm.builder, lhsval, rhsval, cstr("andtmp").as_ptr())
                        };
                        Ok(val)
                    }
                    Op::Or => {
                        let val = unsafe {
                            LLVMBuildOr(llvm.builder, lhsval, rhsval, cstr("ortmp").as_ptr())
                        };
                        Ok(val)
                    }
                }
            }
            Expr::Return { expr } => {
                unsafe { LLVMBuildStore(llvm.builder, expr.codegen(llvm)?, llvm.ret_val) };
                Ok(std::ptr::null_mut())
            }
            Expr::If {
                cond,
                then,
                otherwise,
            } => {
                let then_bb = unsafe {
                    LLVMAppendBasicBlockInContext(llvm.ctx, llvm.func, cstr("then").as_ptr())
                };
                let else_bb = unsafe {
                    LLVMAppendBasicBlockInContext(llvm.ctx, llvm.func, cstr("else").as_ptr())
                };
                let merge_bb = unsafe {
                    LLVMAppendBasicBlockInContext(llvm.ctx, llvm.func, cstr("merge").as_ptr())
                };

                let cond_val = cond.codegen(llvm)?;
                unsafe {
                    LLVMBuildCondBr(llvm.builder, cond_val, then_bb, else_bb);
                    LLVMPositionBuilderAtEnd(llvm.builder, then_bb);
                }

                let mut is_ret = false;
                for expr in then {
                    expr.codegen(llvm)?;
                    match expr {
                        Expr::Return { .. } => {
                            is_ret = true;
                            break;
                        }
                        _ => (),
                    }
                }
                let br_block = if is_ret { llvm.ret_block } else { merge_bb };
                unsafe {
                    LLVMBuildBr(llvm.builder, br_block);
                    LLVMPositionBuilderAtEnd(llvm.builder, else_bb);
                }

                let mut is_ret = false;
                for expr in otherwise {
                    expr.codegen(llvm)?;
                    match expr {
                        Expr::Return { .. } => {
                            is_ret = true;
                            break;
                        }
                        _ => (),
                    }
                }
                let br_block = if is_ret { llvm.ret_block } else { merge_bb };
                unsafe {
                    LLVMBuildBr(llvm.builder, br_block);
                    LLVMPositionBuilderAtEnd(llvm.builder, merge_bb);
                }

                Ok(std::ptr::null_mut())
            }
            Expr::Decl { ty, name, init } => {
                let ty = match ty {
                    Type::Int => unsafe { LLVMInt32TypeInContext(llvm.ctx) },
                    Type::Void => unsafe { LLVMVoidTypeInContext(llvm.ctx) },
                };
                let val = unsafe { LLVMBuildAlloca(llvm.builder, ty, cstr(name).as_ptr()) };
                llvm.locals.insert(name.clone(), Scoped { val, ty });
                match init {
                    Some(init) => {
                        let init_val = init.codegen(llvm)?;
                        unsafe { LLVMBuildStore(llvm.builder, init_val, val) };
                    }
                    None => {}
                }
                Ok(val)
            }
            Expr::Var { name } => {
                let scoped = llvm.locals.get(name).unwrap();
                let val = unsafe {
                    LLVMBuildLoad2(llvm.builder, scoped.ty, scoped.val, cstr(name).as_ptr())
                };
                Ok(val)
            }
            Expr::Assign { name, rhs } => {
                let val = rhs.codegen(llvm)?;
                unsafe { LLVMBuildStore(llvm.builder, val, llvm.locals[name].val) };
                Ok(val)
            }
            Expr::Deref { addr } => {
                let addr_val = addr.codegen(llvm)?;
                let ptr_val = unsafe {
                    LLVMBuildIntToPtr(
                        llvm.builder,
                        addr_val,
                        LLVMPointerType(LLVMInt32TypeInContext(llvm.ctx), 0),
                        cstr("deref").as_ptr(),
                    )
                };
                let val = unsafe {
                    LLVMBuildLoad2(
                        llvm.builder,
                        LLVMInt32TypeInContext(llvm.ctx),
                        ptr_val,
                        cstr("deref").as_ptr(),
                    )
                };
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
        llvm.func = fn_value;

        let bb =
            unsafe { LLVMAppendBasicBlockInContext(llvm.ctx, fn_value, cstr("entry").as_ptr()) };
        unsafe { LLVMPositionBuilderAtEnd(llvm.builder, bb) };
        llvm.ret_val = unsafe { LLVMBuildAlloca(llvm.builder, ret_type, cstr("ret").as_ptr()) };
        unsafe { LLVMBuildStore(llvm.builder, LLVMConstInt(ret_type, 0, 0), llvm.ret_val) };

        llvm.ret_block =
            unsafe { LLVMAppendBasicBlockInContext(llvm.ctx, fn_value, cstr("retblock").as_ptr()) };
        unsafe { LLVMPositionBuilderAtEnd(llvm.builder, bb) };

        for (i, arg) in self.args.iter().enumerate() {
            let name = cstr(&arg.name);
            let val = unsafe { LLVMGetParam(fn_value, i as u32) };
            unsafe { LLVMSetValueName2(val, name.as_ptr(), arg.name.len()) };
            llvm.locals.insert(
                arg.name.clone(),
                Scoped {
                    val,
                    ty: arg_types[i],
                },
            );
        }

        for expr in &self.exprs {
            expr.codegen(llvm)?;
        }
        unsafe { LLVMBuildBr(llvm.builder, llvm.ret_block) };
        unsafe { LLVMPositionBuilderAtEnd(llvm.builder, llvm.ret_block) };
        Ok(unsafe { LLVMBuildRet(llvm.builder, llvm.ret_val) })
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

pub fn codegen(program: &Program, path: &str) {
    let mut llvm = LLVM::new();
    unsafe {
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmParsers();
        LLVM_InitializeAllAsmPrinters();

        program.codegen(&mut llvm).unwrap();
        let target_triple = LLVMCreateMessage(cstr("armv7s-apple-ios").as_ptr());
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

        let cpu = LLVMCreateMessage(cstr("").as_ptr());
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

        let filename = LLVMCreateMessage(cstr(path).as_ptr());
        err_string = std::mem::MaybeUninit::uninit();
        LLVMDumpModule(llvm.module);
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

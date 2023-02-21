use super::constants::*;
use backtrace::Backtrace;
use quickcheck::{Arbitrary, Gen, QuickCheck};
use rand::distributions::{Alphanumeric, DistString};

pub trait Deparse {
    fn deparse(&self) -> String;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Int,
    Void,
}

impl Deparse for Type {
    fn deparse(&self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Void => "void".to_string(),
        }
    }
}

fn readable_string(g: &mut Gen) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), g.size())
}

#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    pub ty: Type,
    pub name: String,
}

impl Deparse for Arg {
    fn deparse(&self) -> String {
        format!("{} {}", self.ty.deparse(), self.name)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Return {
        expr: Box<Expr>,
    },
    Int {
        value: u32,
    },
    BinOp {
        lhs: Box<Expr>,
        rhs: Box<Expr>,
        op: Op,
    },
    If {
        cond: Box<Expr>,
        then: Vec<Expr>,
        otherwise: Vec<Expr>,
    },
    Var {
        ty: Type,
        name: String,
    },
    Decl {
        ty: Type,
        name: String,
        init: Option<Box<Expr>>,
    },
    Assign {
        name: String,
        rhs: Box<Expr>,
    },
}

impl Deparse for Expr {
    fn deparse(&self) -> String {
        match self {
            Expr::Int { value } => value.to_string(),
            Expr::BinOp { lhs, rhs, op } => {
                format!("({} {} {})", lhs.deparse(), op, rhs.deparse())
            }
            Expr::Return { expr } => format!("return {}", expr.deparse()),
            Expr::If {
                cond,
                then,
                otherwise,
            } => {
                let then_str = then
                    .iter()
                    .map(|e| e.deparse())
                    .collect::<Vec<_>>()
                    .join(";\n");
                let otherwise_str = otherwise
                    .iter()
                    .map(|e| e.deparse())
                    .collect::<Vec<_>>()
                    .join(";\n");
                format!(
                    "if ({}) {{\n{};\n}} else {{\n{};\n}}",
                    cond.deparse(),
                    then_str,
                    otherwise_str,
                )
            }
            Expr::Decl {
                ty,
                name,
                init: Some(init),
            } => format!("{} {} = {}", ty.deparse(), name, init.deparse()),
            Expr::Decl {
                ty,
                name,
                init: None,
            } => format!("{} {}", ty.deparse(), name),
            Expr::Assign { name, rhs } => {
                format!("{} = {}", name, rhs.deparse())
            }
            Expr::Var { ty: _, name } => format!("{}", name),
        }
    }
}

impl Arbitrary for Expr {
    fn arbitrary(g: &mut Gen) -> Self {
        if u32::arbitrary(g) % 5 == 0 {
            let op = Op::arbitrary(g);
            let lhs = Box::new(Expr::arbitrary(g));
            let rhs = Box::new(Expr::arbitrary(g));
            return Expr::BinOp { lhs, rhs, op };
        }
        let value = u32::arbitrary(g);
        Expr::Int { value }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub ret_type: Type,
    pub exprs: Vec<Expr>,
    pub name: String,
    pub args: Vec<Arg>,
}

impl Deparse for Function {
    fn deparse(&self) -> String {
        let args = self
            .args
            .iter()
            .map(|a| a.deparse())
            .collect::<Vec<_>>()
            .join(", ");
        let exprs = self
            .exprs
            .iter()
            .map(|e| e.deparse())
            .collect::<Vec<_>>()
            .join(";\n");
        format!(
            "{} {}({}) {{\n{} ;\n}}",
            self.ret_type.deparse(),
            self.name,
            args,
            exprs
        )
    }
}

impl<'a> Arbitrary for Function {
    fn arbitrary(g: &mut Gen) -> Self {
        let ret_type = Type::Void;
        let name = readable_string(g);
        let mut exprs = Vec::new();
        for _ in 0..g.size() {
            exprs.push(Expr::arbitrary(g));
        }
        let mut args = Vec::new();
        for _ in 0..g.size() {
            args.push(Arg {
                ty: Type::Int,
                name: readable_string(g),
            });
        }
        Function {
            ret_type,
            exprs,
            name,
            args,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}

impl Deparse for Program {
    fn deparse(&self) -> String {
        self.functions
            .iter()
            .map(|f| f.deparse())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Arbitrary for Program {
    fn arbitrary(g: &mut Gen) -> Self {
        let mut functions = Vec::new();
        for _ in 0..g.size() {
            functions.push(Function::arbitrary(g));
        }
        Program { functions }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct State<'a> {
    pub code: &'a str,
    pub index: usize,
}

impl<'a> State<'a> {
    fn rest(&self) -> Option<&'a str> {
        self.code.get(self.index..)
    }
}

pub fn head(state: State) -> Option<char> {
    state.rest()?.chars().next()
}

pub fn tail(state: State) -> State {
    let add = match head(state) {
        Some(c) => c.len_utf8(),
        None => 0,
    };
    State {
        code: state.code,
        index: state.index + add,
    }
}

pub type Answer<'a, A> = Result<(State<'a>, A), (String, Backtrace)>;
pub type Parser<'a, A> = Box<dyn Fn(State<'a>) -> Answer<'a, A>>;

pub fn print_state(state: State) -> &str {
    state.code.get(state.index..).unwrap_or("")
}

pub fn expected<'a, A>(state: State<'a>, name: &str, size: usize) -> Answer<'a, A> {
    let end = state.index + size;
    let err = state.code.get(state.index..end).unwrap_or("");
    let rest = state.code.get(end..).unwrap_or("");
    // Err(format!(
    //     "Expected `{}`:\n\x1b[31m\x1b[01m{}\x1b[0m{}",
    //     name, err, rest
    // ))
    let bt = Backtrace::new();
    Err((format!("Expected `{}`: {}{}", name, err, rest), bt))
}

pub fn skip_comment(mut state: State) -> Answer<bool> {
    const COMMENT: &str = "//";
    if let Some(rest) = state.rest() {
        if let Some(line) = rest.lines().next() {
            if line.starts_with(COMMENT) {
                state.index += line.len();
                return Ok((state, true));
            }
        }
    }
    Ok((state, false))
}

pub fn skip_spaces(mut state: State) -> Answer<bool> {
    if let Some(rest) = state.rest() {
        let add: usize = rest
            .chars()
            .take_while(|a| a.is_whitespace())
            .map(|a| a.len_utf8())
            .sum();
        state.index += add;
        if add > 0 {
            return Ok((state, true));
        }
    }
    Ok((state, false))
}

pub fn skip(mut state: State) -> Answer<bool> {
    let (new_state, mut comment) = skip_comment(state)?;
    state = new_state;
    let (new_state, mut spaces) = skip_spaces(state)?;
    state = new_state;
    if comment || spaces {
        loop {
            let (new_state, new_comment) = skip_comment(state)?;
            state = new_state;
            comment = new_comment;
            let (new_state, new_spaces) = skip_spaces(state)?;
            state = new_state;
            spaces = new_spaces;
            if !comment && !spaces {
                return Ok((state, true));
            }
        }
    }
    Ok((state, false))
}

pub fn text<'a>(state: State<'a>, pat: &str) -> Answer<'a, bool> {
    let (state, _) = skip(state)?;
    if let Some(rest) = state.rest() {
        if rest.starts_with(pat) {
            let state = State {
                code: state.code,
                index: state.index + pat.len(),
            };
            return Ok((state, true));
        }
    }
    Ok((state, false))
}

pub fn consume<'a>(state: State<'a>, pat: &'a str) -> Answer<'a, &'a str> {
    let (state, matched) = text(state, pat)?;
    if matched {
        Ok((state, pat))
    } else {
        expected(state, pat, pat.len())
    }
}

pub fn optional_grammar<'a, A: 'a>(
    choices: &[Parser<'a, Option<A>>],
    state: State<'a>,
) -> Answer<'a, Option<A>> {
    for choice in choices {
        let (state, result) = choice(state)?;
        if result.is_some() {
            return Ok((state, result));
        }
    }
    Ok((state, None))
}

pub fn grammar<'a, A: 'a>(
    name: &'static str,
    choices: &[Parser<'a, Option<A>>],
    state: State<'a>,
) -> Answer<'a, A> {
    let (state, result) = optional_grammar(choices, state)?;
    if let Some(result) = result {
        Ok((state, result))
    } else {
        expected(state, name, 1)
    }
}

fn enum_consumer<'a, A>(state: State<'a>, pat: &'static str, val: A) -> Answer<'a, Option<A>> {
    let (state, matched) = text(state, pat)?;
    Ok((state, if matched { Some(val) } else { None }))
}

fn parse_type(state: State) -> Answer<Type> {
    grammar(
        "type",
        &[
            Box::new(|state| enum_consumer(state, "int", Type::Int)),
            Box::new(|state| enum_consumer(state, "void", Type::Void)),
        ],
        state,
    )
}

fn is_letter(chr: char) -> bool {
    chr.is_ascii_alphanumeric() || chr == '_'
}

fn is_number(chr: char) -> bool {
    chr.is_numeric()
}

pub fn name(state: State) -> Answer<String> {
    let mut name: String = String::new();
    let (mut state, _) = skip(state)?;
    while let Some(got) = head(state) {
        if is_letter(got) {
            name.push(got);
            state = tail(state);
        } else {
            break;
        }
    }
    Ok((state, name))
}

pub fn int_here(state: State) -> Answer<String> {
    let mut name: String = String::new();
    let (mut state, _) = skip(state)?;
    while let Some(got) = head(state) {
        if is_number(got) {
            name.push(got);
            state = tail(state);
        } else {
            break;
        }
    }
    Ok((state, name))
}

fn parse_int(state: State) -> Answer<Expr> {
    let (state, is_hex) = text(state, "0x")?;
    if is_hex {
        let (state, src) = int_here(state)?;
        match u32::from_str_radix(&src, 16) {
            Ok(value) => Ok((state, Expr::Int { value })),
            Err(_) => expected(state, "hexadecimal number", src.len()),
        }
    } else {
        let (state, src) = int_here(state)?;
        match u32::from_str_radix(&src, 10) {
            Ok(value) => Ok((state, Expr::Int { value })),
            Err(_) => expected(state, "base10 number", src.len()),
        }
    }
}

fn parse_factor(state: State) -> Answer<Expr> {
    let (state, is_paren) = text(state, "(")?;
    if is_paren {
        let (state, expr) = parse_expr(state)?;
        let (state, _) = consume(state, ")")?;
        Ok((state, expr))
    } else {
        parse_int(state)
    }
}

fn parse_term(state: State) -> Answer<Expr> {
    let (state, lhs) = parse_factor(state)?;
    let (state, op_option) = optional_grammar(
        &[
            Box::new(|state| enum_consumer(state, "/", Op::Div)),
            Box::new(|state| enum_consumer(state, "*", Op::Mul)),
        ],
        state,
    )?;
    if let Some(op) = op_option {
        let (state, rhs) = parse_term(state)?;
        return Ok((
            state,
            Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        ));
    }
    Ok((state, lhs))
}

fn parse_additive_expr(state: State) -> Answer<Expr> {
    let (state, lhs) = parse_term(state)?;
    let (state, op_option) = optional_grammar(
        &[
            Box::new(|state| enum_consumer(state, "-", Op::Sub)),
            Box::new(|state| enum_consumer(state, "+", Op::Add)),
        ],
        state,
    )?;
    if let Some(op) = op_option {
        let (state, rhs) = parse_expr(state)?;
        return Ok((
            state,
            Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        ));
    }
    Ok((state, lhs))
}

fn parse_relational_expr(state: State) -> Answer<Expr> {
    let (state, lhs) = parse_additive_expr(state)?;
    let (state, op_option) = optional_grammar(
        &[
            Box::new(|state| enum_consumer(state, "<=", Op::Le)),
            Box::new(|state| enum_consumer(state, ">=", Op::Ge)),
            Box::new(|state| enum_consumer(state, "<", Op::Lt)),
            Box::new(|state| enum_consumer(state, ">", Op::Gt)),
        ],
        state,
    )?;
    if let Some(op) = op_option {
        let (state, rhs) = parse_expr(state)?;
        return Ok((
            state,
            Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        ));
    }
    Ok((state, lhs))
}

fn parse_assignment_expr(state: State) -> Answer<Expr> {
    let (state, lhs) = parse_relational_expr(state)?;
    let (state, op_option) = optional_grammar(
        &[
            Box::new(|state| enum_consumer(state, "=", Op::Assign)),
            Box::new(|state| enum_consumer(state, "+=", Op::AddAssign)),
            Box::new(|state| enum_consumer(state, "-=", Op::SubAssign)),
            Box::new(|state| enum_consumer(state, "*=", Op::MulAssign)),
            Box::new(|state| enum_consumer(state, "/=", Op::DivAssign)),
        ],
        state,
    )?;
    if let Some(op) = op_option {
        let (state, rhs) = parse_expr(state)?;
        return Ok((
            state,
            Expr::BinOp {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        ));
    }
    Ok((state, lhs))
}

fn parse_expr(state: State) -> Answer<Expr> {
    parse_assignment_expr(state)
}

fn parse_compound_statement(state: State) -> Answer<Vec<Expr>> {
    let mut exprs = Vec::new();
    let (mut state, _) = consume(state, "{")?;
    loop {
        let (new_state, got) = text(state, "}")?;
        if got {
            return Ok((new_state, exprs));
        }
        let (state2, expr) = parse_statement(new_state)?;
        exprs.push(expr);
        state = state2;
    }
}

fn try_parser<'a>(parser: fn(State) -> Answer<Expr>, state: State<'a>) -> Answer<'a, Option<Expr>> {
    match parser(state) {
        Ok((state, expr)) => Ok((state, Some(expr))),
        Err(_) => Ok((state, None)),
    }
}

fn parse_declaration_statement(state: State) -> Answer<Expr> {
    let (state, ty) = parse_type(state)?;
    let (state, identifier) = name(state)?;
    let (state, has_init) = text(state, "=")?;
    if !has_init {
        let (state, _) = consume(state, ";")?;
        return Ok((
            state,
            Expr::Decl {
                ty,
                name: identifier,
                init: None,
            },
        ));
    }
    let (state, expr) = parse_expr(state)?;
    let (state, _) = consume(state, ";")?;
    Ok((
        state,
        Expr::Decl {
            ty,
            name: identifier,
            init: Some(Box::new(expr)),
        },
    ))
}

fn parse_selection_statement(state: State) -> Answer<Expr> {
    let (state, _) = consume(state, "if")?;
    let (state, _) = consume(state, "(")?;
    let (state, expr) = parse_expr(state)?;
    let (state, _) = consume(state, ")")?;
    let (state, then) = grammar(
        "body of if",
        &[
            Box::new(|state| {
                let (state, body) = parse_compound_statement(state)?;
                Ok((state, Some(body)))
            }),
            Box::new(|state| {
                let (state, expr) = parse_statement(state)?;
                Ok((state, Some(vec![expr])))
            }),
        ],
        state,
    )?;
    let (state, else_option) = optional_grammar(
        &[
            Box::new(|state| {
                let (state, _) = consume(state, "else")?;
                let (state, body) = parse_compound_statement(state)?;
                Ok((state, Some(body)))
            }),
            Box::new(|state| {
                let (state, _) = consume(state, "else")?;
                let (state, expr) = parse_statement(state)?;
                Ok((state, Some(vec![expr])))
            }),
        ],
        state,
    )?;
    return Ok((
        state,
        Expr::If {
            cond: Box::new(expr),
            then,
            otherwise: else_option.unwrap_or(vec![]),
        },
    ));
}

fn parse_return_statement(state: State) -> Answer<Expr> {
    let (state, ret) = text(state, "return")?;
    let (state, expr) = parse_expr(state)?;
    let (state, _) = consume(state, ";")?;
    if ret {
        return Ok((
            state,
            Expr::Return {
                expr: Box::new(expr),
            },
        ));
    }
    Ok((state, expr))
}

fn parse_assign_statement(state: State) -> Answer<Expr> {
    let (state, identifier) = name(state)?;
    let (state, _) = consume(state, "=")?;
    let (state, expr) = parse_expr(state)?;
    let (state, _) = consume(state, ";")?;
    Ok((
        state,
        Expr::Assign {
            name: identifier,
            rhs: Box::new(expr),
        },
    ))
}

fn parse_statement(state: State) -> Answer<Expr> {
    grammar(
        "type",
        &[
            Box::new(|state| try_parser(parse_return_statement, state)),
            Box::new(|state| try_parser(parse_assign_statement, state)),
            Box::new(|state| try_parser(parse_declaration_statement, state)),
            Box::new(|state| try_parser(parse_selection_statement, state)),
        ],
        state,
    )
}

fn parse_paramlist<'a, 'b>(
    args: &'b mut Vec<Arg>,
    pat: &str,
    state: State<'a>,
) -> Answer<'a, &'b Vec<Arg>> {
    let (state, matched) = text(state, pat)?;
    if matched {
        return Ok((state, args));
    }
    let (state, ty) = parse_type(state)?;
    let (state, name) = name(state)?;
    let (state, _) = text(state, ",")?;
    args.push(Arg { name, ty });
    parse_paramlist(args, pat, state)
}

fn parse_function(state: State) -> Answer<Function> {
    let (state, ret_type) = parse_type(state)?;
    let (state, name) = name(state)?;
    let (state, _) = consume(state, "(")?;
    let mut args = Vec::new();
    let (state, _) = parse_paramlist(&mut args, ")", state)?;
    let (state, exprs) = parse_compound_statement(state)?;
    let function = Function {
        ret_type,
        exprs,
        name,
        args,
    };
    Ok((state, function))
}

fn parse_top_level(state: State) -> Answer<Program> {
    let mut state = state;
    let mut functions: Vec<Function> = Vec::new();
    loop {
        let (new_state, _) = skip(state)?;
        if new_state.rest().is_none() || new_state.rest().unwrap().len() == 0 {
            break;
        }
        let (new_state, function) = parse_function(new_state)?;
        functions.push(function);
        state = new_state;
    }
    Ok((state, Program { functions }))
}

// parse a string of C code
pub fn parse(code: &str) -> Result<Program, String> {
    println!("\n\nparsing\n{}", code);
    match parse_top_level(State { code, index: 0 }) {
        Ok((_, value)) => Ok(value),
        Err((msg, bt)) => {
            println!("{:?}", bt);
            Err(msg)
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use crate::codegen::codegen;

    use super::*;

    #[test]
    fn test_skip_whitespace() {
        assert_eq!(
            skip(State {
                code: "\n",
                index: 0
            })
            .unwrap()
            .0
            .index,
            1
        );
        assert_eq!(
            skip(State {
                code: "\n\n \t\n ",
                index: 1
            })
            .unwrap()
            .0
            .index,
            6
        );
    }

    fn codegen_code(code: &str, program: &Program) {
        let hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::Hasher;
            let mut hasher = DefaultHasher::new();
            code.hash(&mut hasher);
            hasher.finish()
        };
        let file_path = format!("out/{}.out", hash.to_string());
        println!("writing to {}", file_path);
        codegen(program, &file_path);
    }

    fn test_main1(code: &str, ret_type: Type, exprs: Vec<Expr>) {
        let program = parse(code).unwrap();
        let function = &program.functions[0];
        assert_eq!(program.functions.len(), 1);
        assert_eq!(function.name, "main");
        assert_eq!(function.ret_type, ret_type);
        assert_eq!(function.exprs, exprs);
        codegen_code(code, &program)
    }

    #[test]
    fn test_parse1() {
        let code = "int main() { return 0; }";
        let ret_type = Type::Int;
        let ret_expr = Expr::Return {
            expr: Box::new(Expr::Int { value: 0 }),
        };
        let exprs = vec![ret_expr];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_void1() {
        let code = "void main() { 42 + 24; }";
        let ret_type = Type::Void;
        let exprs = vec![Expr::BinOp {
            op: Op::Add,
            lhs: Box::new(Expr::Int { value: 42 }),
            rhs: Box::new(Expr::Int { value: 24 }),
        }];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_statements1() {
        let code = "int main() { 2 + 3; return 0; }";
        let ret_type = Type::Int;
        let exprs = vec![
            Expr::BinOp {
                op: Op::Add,
                lhs: Box::new(Expr::Int { value: 2 }),
                rhs: Box::new(Expr::Int { value: 3 }),
            },
            Expr::Return {
                expr: Box::new(Expr::Int { value: 0 }),
            },
        ];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_conditional1() {
        let code = "int main() { if (1) { return 0; } else { return 1; } }";
        let ret_type = Type::Int;
        let exprs = vec![Expr::If {
            cond: Box::new(Expr::Int { value: 1 }),
            then: vec![Expr::Return {
                expr: Box::new(Expr::Int { value: 0 }),
            }],
            otherwise: vec![Expr::Return {
                expr: Box::new(Expr::Int { value: 1 }),
            }],
        }];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_conditional2() {
        let code = "int main() { if (2 > 1) { return 0; } else { return 1; } }";
        let ret_type = Type::Int;
        let exprs = vec![Expr::If {
            cond: Box::new(Expr::BinOp {
                op: Op::Gt,
                lhs: Box::new(Expr::Int { value: 2 }),
                rhs: Box::new(Expr::Int { value: 1 }),
            }),
            then: vec![Expr::Return {
                expr: Box::new(Expr::Int { value: 0 }),
            }],
            otherwise: vec![Expr::Return {
                expr: Box::new(Expr::Int { value: 1 }),
            }],
        }];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_parse2() {
        let code = "int main() { return 3 + 2; }";
        let ret_type = Type::Int;
        let expr = Expr::BinOp {
            op: Op::Add,
            lhs: Box::new(Expr::Int { value: 3 }),
            rhs: Box::new(Expr::Int { value: 2 }),
        };
        let exprs = vec![Expr::Return {
            expr: Box::new(expr),
        }];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_parse3() {
        let code = "void main(int ls, int js) { (0 + 3738978009); 2714820978 ; }";
        let ret_type = Type::Void;
        let exprs = vec![
            Expr::BinOp {
                op: Op::Add,
                lhs: Box::new(Expr::Int { value: 0 }),
                rhs: Box::new(Expr::Int { value: 3738978009 }),
            },
            Expr::Int { value: 2714820978 },
        ];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_assign1() {
        let code = "int main() { int a; int b; a = 2; b = 3 + 2; return 1 + 2; }";
        let ret_type = Type::Int;
        let exprs = vec![
            Expr::Decl {
                ty: Type::Int,
                name: "a".to_string(),
                init: None,
            },
            Expr::Decl {
                ty: Type::Int,
                name: "b".to_string(),
                init: None,
            },
            Expr::Assign {
                name: "a".to_string(),
                rhs: Box::new(Expr::Int { value: 2 }),
            },
            Expr::Assign {
                name: "b".to_string(),
                rhs: Box::new(Expr::BinOp {
                    op: Op::Add,
                    lhs: Box::new(Expr::Int { value: 3 }),
                    rhs: Box::new(Expr::Int { value: 2 }),
                }),
            },
            Expr::Return {
                expr: Box::new(Expr::BinOp {
                    op: Op::Add,
                    lhs: Box::new(Expr::Int { value: 1 }),
                    rhs: Box::new(Expr::Int { value: 2 }),
                }),
            },
        ];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_decl1() {
        let code = "int main() { int a; }";
        let ret_type = Type::Int;
        let exprs = vec![Expr::Decl {
            ty: Type::Int,
            name: "a".to_string(),
            init: None,
        }];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_decl2() {
        let code = "int main() { int a = 2; int b = 3 + 2; return 1 + 2; }";
        let ret_type = Type::Int;
        let exprs = vec![
            Expr::Decl {
                ty: Type::Int,
                name: "a".to_string(),
                init: Some(Box::new(Expr::Int { value: 2 })),
            },
            Expr::Decl {
                ty: Type::Int,
                name: "b".to_string(),
                init: Some(Box::new(Expr::BinOp {
                    op: Op::Add,
                    lhs: Box::new(Expr::Int { value: 3 }),
                    rhs: Box::new(Expr::Int { value: 2 }),
                })),
            },
            Expr::Return {
                expr: Box::new(Expr::BinOp {
                    op: Op::Add,
                    lhs: Box::new(Expr::Int { value: 1 }),
                    rhs: Box::new(Expr::Int { value: 2 }),
                }),
            },
        ];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_parse_mixed_binop1() {
        let code = "int main() { return 3 + 2 + (5 - 7 * 10); }";
        let ret_type = Type::Int;
        let expr = Expr::BinOp {
            op: Op::Add,
            lhs: Box::new(Expr::Int { value: 3 }),
            rhs: Box::new(Expr::BinOp {
                op: Op::Add,
                lhs: Box::new(Expr::Int { value: 2 }),
                rhs: Box::new(Expr::BinOp {
                    op: Op::Sub,
                    lhs: Box::new(Expr::Int { value: 5 }),
                    rhs: Box::new(Expr::BinOp {
                        op: Op::Mul,
                        lhs: Box::new(Expr::Int { value: 7 }),
                        rhs: Box::new(Expr::Int { value: 10 }),
                    }),
                }),
            }),
        };
        let exprs = vec![Expr::Return {
            expr: Box::new(expr),
        }];
        test_main1(code, ret_type, exprs);
    }

    #[test]
    fn test_prop1() {
        fn prop1(program: Program) -> bool {
            let code = program.deparse();
            let parsed = parse(&code).unwrap();
            codegen_code(&code, &program);
            parsed == program
        }
        QuickCheck::new()
            .gen(Gen::new(2))
            .tests(10)
            .quickcheck(prop1 as fn(Program) -> bool)
    }
}

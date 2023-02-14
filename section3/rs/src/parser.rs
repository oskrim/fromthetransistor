use super::constants::*;
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
    Int {
        value: u32,
    },
    BinOp {
        left: Box<Expr>,
        right: Box<Expr>,
        op: Op,
    },
}

impl Deparse for Expr {
    fn deparse(&self) -> String {
        match self {
            Expr::Int { value } => value.to_string(),
            Expr::BinOp { left, right, op } => {
                format!("({} {} {})", left.deparse(), op, right.deparse())
            }
        }
    }
}

impl Arbitrary for Expr {
    fn arbitrary(g: &mut Gen) -> Self {
        Expr::Int {
            value: u32::arbitrary(g),
        }
        // let op = Op::arbitrary(g);
        // let left = Box::new(Expr::arbitrary(g));
        // let right = Box::new(Expr::arbitrary(g));
        // Expr::BinOp { left, right, op }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub ret_type: Type,
    pub ret_expr: Expr,
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
        format!(
            "{} {}({}) {{ return {}; }}",
            self.ret_type.deparse(),
            self.name,
            args,
            self.ret_expr.deparse()
        )
    }
}

impl<'a> Arbitrary for Function {
    fn arbitrary(g: &mut Gen) -> Self {
        let ret_type = Type::Void;
        let ret_expr = Expr::arbitrary(g);
        let name = readable_string(g);
        let mut args = Vec::new();
        for _ in 0..g.size() {
            args.push(Arg {
                ty: Type::Int,
                name: readable_string(g),
            });
        }
        Function {
            ret_type,
            ret_expr,
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
            .join("\n\n")
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

pub type Answer<'a, A> = Result<(State<'a>, A), String>;
pub type Parser<'a, A> = Box<dyn Fn(State<'a>) -> Answer<'a, A>>;

pub fn expected<'a, A>(name: &str, size: usize, state: State<'a>) -> Answer<'a, A> {
    let end = state.index + size;
    let err = state.code.get(state.index..end).unwrap_or("");
    let rest = state.code.get(end..).unwrap_or("");
    // Err(format!(
    //     "Expected `{}`:\n\x1b[31m\x1b[01m{}\x1b[0m{}",
    //     name, err, rest
    // ))
    Err(format!("Expected `{}`: {}{}", name, err, rest))
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

pub fn text_here<'a>(pat: &str, state: State<'a>) -> Answer<'a, bool> {
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

pub fn text<'a>(pat: &str, state: State<'a>) -> Answer<'a, bool> {
    let (state, _) = skip(state)?;
    let (state, matched) = text_here(pat, state)?;
    Ok((state, matched))
}

pub fn consume<'a>(pat: &'a str, state: State<'a>) -> Answer<'a, &'a str> {
    let (state, matched) = text(pat, state)?;
    if matched {
        Ok((state, pat))
    } else {
        expected(pat, pat.len(), state)
    }
}

pub fn grammar<'a, A: 'a>(
    name: &'static str,
    choices: &[Parser<'a, Option<A>>],
    state: State<'a>,
) -> Answer<'a, A> {
    for choice in choices {
        let (state, result) = choice(state)?;
        if let Some(value) = result {
            return Ok((state, value));
        }
    }
    expected(name, 1, state)
}

fn type_consumer<'a>(pat: &'static str, ty: Type, state: State<'a>) -> Answer<'a, Option<Type>> {
    let (state, matched) = text(pat, state)?;
    Ok((state, if matched { Some(ty) } else { None }))
}

fn parse_type(state: State) -> Answer<Type> {
    grammar(
        "type",
        &[
            Box::new(|state| type_consumer("int", Type::Int, state)),
            Box::new(|state| type_consumer("void", Type::Void, state)),
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

pub fn name_here(state: State) -> Answer<String> {
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
    let (state, is_hex) = text("0x", state)?;
    if is_hex {
        let (state, src) = int_here(state)?;
        match u32::from_str_radix(&src, 16) {
            Ok(value) => Ok((state, Expr::Int { value })),
            Err(_) => expected("hexadecimal number", src.len(), state),
        }
    } else {
        let (state, src) = int_here(state)?;
        match u32::from_str_radix(&src, 10) {
            Ok(value) => Ok((state, Expr::Int { value })),
            Err(_) => expected("hexadecimal number", src.len(), state),
        }
    }
}

fn parse_binop<'a>(state: State<'a>, pat: &'a str) -> Answer<'a, Option<Expr>> {
    let (state, matched) = text(pat, state)?;
    if matched {
        let (state, expr2) = parse_expr(state)?;
        Ok((state, Some(expr2)))
    } else {
        Ok((state, None))
    }
}

fn get_op(op: &str) -> Op {
    match op {
        "+" => Op::Add,
        "-" => Op::Sub,
        _ => panic!("Unknown operator {}", op),
    }
}

fn parse_expr(state: State) -> Answer<Expr> {
    let (state, left) = parse_int(state)?;
    for op in ["+", "-"].iter() {
        let (state, expr) = parse_binop(state, op)?;
        if let Some(right) = expr {
            return Ok((
                state,
                Expr::BinOp {
                    op: get_op(op),
                    left: Box::new(left),
                    right: Box::new(right),
                },
            ));
        }
    }
    Ok((state, left))
}

fn parse_return_statement(state: State) -> Answer<Expr> {
    let (state, _) = consume("return", state)?;
    let (state, expr) = parse_expr(state)?;
    let (state, _) = consume(";", state)?;
    Ok((state, expr))
}

fn parse_paramlist<'a, 'b>(
    args: &'b mut Vec<Arg>,
    pat: &str,
    state: State<'a>,
) -> Answer<'a, &'b Vec<Arg>> {
    let (state, matched) = text(pat, state)?;
    if matched {
        return Ok((state, args));
    }
    let (state, ty) = parse_type(state)?;
    let (state, name) = name_here(state)?;
    args.push(Arg { name, ty });
    parse_paramlist(args, pat, state)
}

fn parse_function(state: State) -> Answer<Function> {
    let (state, ret_type) = parse_type(state)?;
    let (state, name) = name_here(state)?;
    let (state, _) = consume("(", state)?;
    let mut args = Vec::new();
    let (state, _) = parse_paramlist(&mut args, ")", state)?;
    let (state, _) = consume("{", state)?;
    let (state, ret_expr) = parse_return_statement(state)?;
    let (state, _) = consume("}", state)?;
    let function = Function {
        ret_type,
        ret_expr,
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
    match parse_top_level(State { code, index: 0 }) {
        Ok((_, value)) => Ok(value),
        Err(msg) => Err(msg),
    }
}

// tests
#[cfg(test)]
mod tests {
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

    fn test_main1(code: &str, ret_type: Type, ret_expr: Expr) {
        let program = parse(code).unwrap();
        let function = &program.functions[0];
        assert_eq!(program.functions.len(), 1);
        assert_eq!(function.name, "main");
        assert_eq!(function.ret_type, ret_type);
        assert_eq!(function.ret_expr, ret_expr);
    }

    #[test]
    fn test_parse1() {
        let code = "int main() { return 0; }";
        let ret_type = Type::Int;
        let ret_expr = Expr::Int { value: 0 };
        test_main1(code, ret_type, ret_expr);
    }

    #[test]
    fn test_parse2() {
        let code = "int main() { return 3 + 2; }";
        let ret_type = Type::Int;
        let ret_expr = Expr::BinOp {
            op: Op::Add,
            left: Box::new(Expr::Int { value: 3 }),
            right: Box::new(Expr::Int { value: 2 }),
        };
        test_main1(code, ret_type, ret_expr);
    }

    #[test]
    fn test_prop1() {
        fn prop1(program: Program) -> bool {
            let code = program.deparse();
            let parsed = parse(&code).unwrap();
            parsed == program
        }
        QuickCheck::new()
            .gen(Gen::new(1))
            .quickcheck(prop1 as fn(Program) -> bool)
    }
}

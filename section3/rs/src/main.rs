// import env and File
use std::fs;
use std::env;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
  Int,
  Void,
}

#[derive(Clone, Debug)]
pub struct Arg {
  pub ty: Type,
  pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
  Int { value: u32 },
}

#[derive(Clone, Debug)]
pub struct Function {
  pub ret_type: Type,
  pub ret_expr: Expr,
  pub name: String,
  pub args: Vec<Arg>,
}

#[derive(Clone, Debug)]
pub struct Program {
  pub functions: Vec<Function>,
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
  State { code: state.code, index: state.index + add }
}

pub type Answer<'a, A> = Result<(State<'a>, A), String>;
pub type Parser<'a, A> = Box<dyn Fn(State<'a>) -> Answer<'a, A>>;

pub fn expected<'a, A>(name: &str, size: usize, state: State<'a>) -> Answer<'a, A> {
  let end = state.index + size;
  let err = state.code.get(state.index..end).unwrap_or("");
  let rest = state.code.get(end..).unwrap_or("");
  Err(format!("Expected `{}`:\n\x1b[31m\x1b[01m{}\x1b[0m{}", name, err, rest))
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
    let add: usize = rest.chars().take_while(|a| a.is_whitespace()).map(|a| a.len_utf8()).sum();
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
      let state = State { code: state.code, index: state.index + pat.len() };
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

fn parse_type (state: State) -> Answer<Type> {
  grammar ("type", &[
    Box::new(|state| { type_consumer("int", Type::Int, state) }),
    Box::new(|state| { type_consumer("void", Type::Void, state) }),
  ], state)
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
      Err(_) => expected("hexadecimal number",src.len(), state),
    }
  }
}

fn parse_expr(state: State) -> Answer<Expr> {
  let (state, expr) = parse_int(state)?;
  Ok((state, expr))
}

fn parse_return_statement(state: State) -> Answer<Expr> {
  let (state, _) = consume("return", state)?;
  let (state, expr) = parse_expr(state)?;
  let (state, _) = consume(";", state)?;
  Ok((state, expr))
}

fn parse_function(state: State) -> Answer<Function> {
  let (state, ret_type) = parse_type(state)?;
  let (state, name) = name_here(state)?;
  let (state, _) = consume("(", state)?; // TODO: parse args
  let (state, _) = consume(")", state)?;
  let (state, _) = consume("{", state)?;
  let (state, ret_expr) = parse_return_statement(state)?;
  let (state, _) = consume("}", state)?;
  let function = Function { ret_type, ret_expr, name, args: Vec::new() };
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
fn parse(code: &str) -> Result<Program, String>  {
  match parse_top_level(State { code, index: 0 }) {
    Ok((_, value)) => Ok(value),
    Err(msg) => Err(msg),
  }
}

// read a file from argv[1]
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
      println!("Usage: {} filename", args[0]);
      return;
  }

  let code = fs::read_to_string(&args[1]).expect("Unable to read file");
  match parse(&code) {
    Ok(program) => println!("{:#?}", program),
    Err(msg) => println!("{}", msg),
  }
}

// tests
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_skip_whitespace() {
    assert_eq!(skip(State { code: "\n", index: 0 }).unwrap().0.index, 1);
    assert_eq!(skip(State { code: "\n\n \t\n ", index: 1 }).unwrap().0.index, 6);
  }

  #[test]
  fn test_parse() {
    let code = "int main() { return 0; }";
    let program = parse(code).unwrap();
    assert_eq!(program.functions.len(), 1);
    let function = &program.functions[0];
    assert_eq!(function.name, "main");
    assert_eq!(function.ret_type, Type::Int);
    assert_eq!(function.ret_expr, Expr::Int { value: 0 });
  }
}

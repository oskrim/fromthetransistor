mod parser;

use std::fs;
use std::env;

// read a file from argv[1]
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
      println!("Usage: {} filename", args[0]);
      return;
  }

  let code = fs::read_to_string(&args[1]).expect("Unable to read file");
  match parser::parse(&code) {
    Ok(program) => println!("{:#?}", program),
    Err(msg) => println!("{}", msg),
  }
}

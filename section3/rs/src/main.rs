mod codegen;
mod parser;

use std::env;
use std::fs;

// read a file from argv[1]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} filename", args[0]);
        return;
    }

    let code = fs::read_to_string(&args[1]).expect("Unable to read file");
    match parser::parse(&code) {
        Ok(program) => {
            println!("Parsed program: {:#?}", program);
            println!("Generated:");
            codegen::codegen(program);
        }
        Err(msg) => println!("{}", msg),
    }
}

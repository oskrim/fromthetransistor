mod codegen;
mod constants;
mod parser;

use std::env;
use std::fs;
use std::io;

// read a file from argv[1]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: {} <filename> or stdin", args[0]);
        return;
    }

    let code = if args.len() == 2 {
        fs::read_to_string(&args[1]).expect("Unable to read file")
    } else {
        let mut code = String::new();
        io::stdin()
            .read_line(&mut code)
            .expect("Unable to read stdin");
        code
    };

    match parser::parse(&code) {
        Ok(program) => {
            println!("Parsed program: {:#?}", program);
            println!("Generated:");
            codegen::codegen(program);
        }
        Err(msg) => println!("{}", msg),
    }
}

#[cfg(test)]
extern crate quickcheck;

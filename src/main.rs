pub mod token_type;
pub mod token;
pub mod scanner;
pub mod parser;
pub mod ast;
mod interpreter;

use std::{env, io::Error, path::Path};


use interpreter::interpret;
use parser::Parser;
use rustlox::read_file;



use crate::scanner::Scanner;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let contents = read_file(&path)?;
    let mut scanner = Scanner::new(contents);
    scanner.scan_tokens();

    let mut parser = Parser::from_tokens(scanner.tokens);
    let expr = parser.parse();
    
    interpret(&expr);

    Ok(())
}

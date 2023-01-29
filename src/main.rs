pub mod token_type;
pub mod token;
pub mod scanner;

use std::{env, io::Error, path::Path};

use rustlox::read_file;

use crate::scanner::Scanner;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let contents = read_file(&path)?;
    let mut scanner = Scanner::new(contents);
    scanner.scan_tokens();
    
    println!("{}", scanner);
    

    Ok(())
}

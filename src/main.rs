pub mod token_type;
pub mod token;
pub mod scanner;
pub mod parser;
pub mod ast;

use std::{env, io::Error, path::Path};


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
    println!("{expr}");

    // println!("{}", scanner);

    // let expr = Expr::Binary {
    //     left: Box::new(Expr::Unary { operator: Token::new(TokenType::Minus, 1), right: Box::new(Expr::Literal { value: ast::LiteralKind::Float(123f32) }) }),
    //     operator: Token::new(TokenType::Star, 1),
    //     right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { value: ast::LiteralKind::String("Hi".into()) }) })
    // };

    // println!("Expression: {expr}");

    Ok(())
}

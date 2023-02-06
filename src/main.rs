pub mod token_type;
pub mod token;
pub mod scanner;
mod parser;
mod ast;

use std::{env, io::Error, path::Path};

use ast::Expr;
use token::Token;
use token_type::TokenType;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let _path = Path::new(&args[1]);

    // let contents = read_file(&path)?;
    // let mut scanner = Scanner::new(contents);
    // scanner.scan_tokens();

    // println!("{}", scanner);

    let expr = Expr::Binary {
        left: Box::new(Expr::Unary { operator: Token::new(TokenType::Minus, 1), right: Box::new(Expr::Literal { value: ast::LiteralKind::Float(123f32) }) }),
        operator: Token::new(TokenType::Star, 1),
        right: Box::new(Expr::Grouping { expr: Box::new(Expr::Literal { value: ast::LiteralKind::String("Hi".into()) }) })
    };

    println!("Expression: {expr}");

    Ok(())
}

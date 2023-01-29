use core::fmt;

use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text: String = match &self.token_type {
            TokenType::Identifier(identifier) => identifier.to_owned(),
            TokenType::String(string) => string.to_owned(),
            TokenType::Float(float) => float.to_string(),
            token => format!("{:?}", token),
        };

        write!(f, "Token \"{}\" on line {}", text, self.line)
    }
}

impl Token {
    pub fn new(token_type: TokenType, line: usize) -> Token {
        Token { token_type, line, }
    }
}
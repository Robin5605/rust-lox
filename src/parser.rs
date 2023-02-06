use std::{iter::Peekable, vec::IntoIter, fmt::Display};

use crate::{token::Token, ast::{Expr, LiteralKind}, token_type::TokenType};

struct Parser<'a> {
    tokens: Peekable<IntoIter<Token>>,
    last: Option<&'a Token>
}

impl<'a> Parser<'a> {
    pub fn from_tokens(tokens: Vec<Token>) -> Parser<'a> {
        let iter = tokens.into_iter().peekable();
        Parser { tokens: iter, last: None }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparision();

        while self.match_(TokenType::BangEq) || self.match_(TokenType::EqEq) {
            let operator = self.last.unwrap().clone();
            let right = self.comparision();
            expr = Expr::Binary { left: expr.into(), operator, right: right.into() }
        };

        expr
    }

    fn comparision(&mut self) -> Expr {
        let mut expr = self.term();

        while
            self.match_(TokenType::Greater) ||
            self.match_(TokenType::GreaterEq) ||
            self.match_(TokenType::Less) ||
            self.match_(TokenType::LessEq)
        {
            let operator = self.last.unwrap().clone();
            let right = self.term();
            expr = Expr::Binary { left: expr.into(), operator, right: right.into() }
        };

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        while self.match_(TokenType::Minus) || self.match_(TokenType::Plus) {
            let operator = self.last.unwrap().clone();
            let right = self.factor();
            expr = Expr::Binary { left: expr.into(), operator, right: right.into() }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_(TokenType::Slash) || self.match_(TokenType::Star) {
            let operator = self.last.unwrap().clone();
            let right = self.unary();
            expr = Expr::Binary { left: expr.into(), operator, right: right.into() }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_(TokenType::Bang) || self.match_(TokenType::Minus) {
            let operator = self.last.unwrap().clone();
            let right = self.unary();
            Expr::Unary { operator, right: right.into() }
        } else {
            self.primary()
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
           self.tokens.next().unwrap()
        } else {
            panic!("{}", message);
        }
    }

    fn primary(&mut self) -> Expr {
        if let Some(token) = self.tokens.next() {
           match token.token_type {
               TokenType::True => Expr::Literal { value: LiteralKind::Bool(true) },
               TokenType::False => Expr::Literal { value: LiteralKind::Bool(false) },
               TokenType::Nil => Expr::Literal { value: LiteralKind::Nil },
               TokenType::Float(f) => Expr::Literal { value: LiteralKind::Float(f) },
               TokenType::String(s) => Expr::Literal { value: LiteralKind::String(s) },
               TokenType::LeftParen => {
                   let expr = self.expression();
                   self.consume(TokenType::RightParen, "Expected ')' after expression.");
                   Expr::Grouping { expr: expr.into() }
               },
               _ => todo!(),
           }
        } else {
            panic!("Couldn't parse primary at end of stream");
        }
    }

    fn match_(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.tokens.next();
            true
        } else {
            false
        }
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if let Some(peek) = self.tokens.peek() {
            peek.token_type == token_type
        } else {
            false
        }
    }
}

impl<'a> Display for Parser<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = String::new();
        for token in self.tokens.clone() {
            buffer.push_str(&format!("{token} "));
        };

        write!(f, "{buffer}")
    }
}

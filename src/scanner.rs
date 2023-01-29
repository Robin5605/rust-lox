use std::fmt::Display;

use crate::{token::Token, token_type::TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner { 
            source, 
            tokens: Vec::new(), 
            current: 0,
            start: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token();
            if let Some(t) = token {
                self.tokens.push(t);
            }
        }

        self.tokens.push(Token::new(TokenType::EOF, self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    
    fn parse_string(&mut self) -> String {
        let mut string = String::new();

        // Opening "
        self.advance();
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line = self.line.saturating_add(1);
            }

            let c = self.advance();
            string.push(c);
        }

        if self.is_at_end() {
            panic!("Unterminated string literal")
        }

        // Closing "
        self.advance();

        string
    }

    fn parse_number(&mut self) -> f32 {
        let mut num = String::new();

        while self.peek().is_digit(10) {
            num.push(self.advance());
        };

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume "."
            num.push(self.advance());

            while self.peek().is_digit(10) {
                num.push(self.advance());
            };
        };

        // Ideally this should never panic
        num.parse().unwrap()
    }

    fn parse_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while self.peek().is_alphanumeric() && !self.is_at_end() {
            let c = self.advance();
            dbg!(self.current);
            identifier.push(c);
        };

        identifier
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.peek();

        let token_type: Option<TokenType> = match c {
            '(' => {
                self.advance();
                Some(TokenType::LeftParen)
            },
            ')' => {
                self.advance();
                Some(TokenType::RightParen)
            },
            '{' => {
                self.advance();
                Some(TokenType::LeftBrace)
            },
            '}' => {
                self.advance();
                Some(TokenType::RightBrace)
            },
            ',' => {
                self.advance();
                Some(TokenType::Comma)
            },
            '.' => {
                self.advance();
                Some(TokenType::Dot)
            },
            '-' => {
                self.advance();
                Some(TokenType::Minus)
            },
            '+' => {
                self.advance();
                Some(TokenType::Plus)
            },
            ';' => {
                self.advance();
                Some(TokenType::Semicolon)
            },
            '*' => {
                self.advance();
                Some(TokenType::Star)
            },
            '\n' => { 
                self.line = self.line.saturating_add(1);
                self.advance();
                None
            },
            ' ' | '\r' | '\t' => {
                self.advance();
                None
            },
            '!' => {
                self.advance();
                Some(if self.match_('=') { TokenType::BangEq } else { TokenType::Bang })
            }
            '=' => {
                self.advance();
                Some(if self.match_('=') { TokenType::EqEq } else { TokenType::Eq })
            },
            '<' => {
                self.advance();
                Some(if self.match_('=') { TokenType::LessEq } else { TokenType::Less})
            },
            '>' => {
                self.advance();
                Some(if self.match_('=') { TokenType::GreaterEq } else { TokenType::Greater })
            },
            '/' => {
                self.advance();
                if self.match_('/') {
                    // Skip over comments
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }

                    None
                } else {
                    Some(TokenType::Slash)
                }
            }
            '"' => Some(TokenType::String(self.parse_string())),
            '0'..='9' => Some(TokenType::Float(self.parse_number())),
            'a'..='z' | 'A'..='Z' | '_' => Some(TokenType::Identifier(self.parse_identifier())),
            _ => panic!("Unrecognized token: {} on line {}", c, self.line),
        };

        match token_type {
            Some(t) => Some(Token::new(t, self.line)),
            None => None,
        }
    }

    fn _char_at(&self, index: usize) -> char {
        self.source.as_bytes()[index] as char
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self._char_at(self.current)
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self._char_at(self.current + 1)
        }
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            false
        } else {
            if !self._char_at(self.current).eq(&expected) {
                false
            } else {
                self.current = self.current.saturating_add(1);
                true
            }
        }
    }

    fn advance(&mut self) -> char {
        let curr = self.current;
        self.current = self.current.saturating_add(1);
        self._char_at(curr)
    }
}

impl  Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.tokens.iter().map(|i| format!("{}", i)).collect::<Vec<String>>().join("\n"))
    }
}
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, 
    RightParen, 
    LeftBrace, 
    RightBrace,
    Comma, 
    Dot, 
    Minus, 
    Plus, 
    Semicolon, 
    Slash, 
    Star,

    Bang,
    BangEq,
    Eq,
    EqEq,
    Greater,
    GreaterEq,
    Less,
    LessEq,

    Identifier(String),
    String(String),
    Float(f32),

    And,
    Class,
    If,
    Else,
    True,
    False,
    Fun,
    For,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            TokenType::Identifier(identifier) => format!("Identifier('{}')", identifier),
            TokenType::String(string) => format!("String('{}')", string),
            TokenType::Float(float) => format!("Float({})", float),
            TokenType::Star => "*".into(),
            TokenType::Minus => "-".into(),
            TokenType::LeftParen => "(".into(),
            TokenType::RightParen => ")".into(),
            TokenType::LeftBrace => "{".into(),
            TokenType::RightBrace => "}".into(),
            TokenType::Comma => ",".into(),
            TokenType::Dot => ".".into(),
            TokenType::Plus => "+".into(),
            TokenType::Semicolon => ";".into(),
            TokenType::Slash => "/".into(),
            TokenType::Bang => "!".into(),
            TokenType::BangEq => "!=".into(),
            TokenType::Eq => "=".into(),
            TokenType::EqEq => "==".into(),
            TokenType::Greater => ">".into(),
            TokenType::GreaterEq => ">=".into(),
            TokenType::Less => "<".into(),
            TokenType::LessEq => "<=".into(),
            TokenType::And => "&".into(),
            TokenType::Class => "CLASS".into(),
            TokenType::If => "IF".into(),
            TokenType::Else => "ELSE".into(),
            TokenType::True => "TRUE".into(),
            TokenType::False => "FALSE".into(),
            TokenType::Fun => "FUN".into(),
            TokenType::For => "FOR".into(),
            TokenType::Nil => "NIL".into(),
            TokenType::Or => "|".into(),
            TokenType::Print => "PRINT".into(),
            TokenType::Return => "RETURN".into(),
            TokenType::Super => "SUPER".into(),
            TokenType::This => "THIS".into(),
            TokenType::Var => "VAR".into(),
            TokenType::While => "WHILE".into(),
            TokenType::EOF => "EOF".into(),
        };

        write!(f, "{}", res)
    }
}

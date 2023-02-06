use std::fmt::Display;

use crate::token::Token;

#[derive(Clone)]
pub enum LiteralKind {
    String(String),
    Float(f32),
    Bool(bool),
    Nil,
}

#[derive(Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    Grouping {
        expr: Box<Expr>,
    },

    Literal {
        value: LiteralKind
    },

    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Display for LiteralKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match &self {
            LiteralKind::String(s) => format!("String('{s}')"),
            LiteralKind::Float(f) => format!("Float('{f}')"),
            LiteralKind::Bool(b) => format!("Bool('{b}')"),
            LiteralKind::Nil => format!("Nil"),

        };

        write!(f, "{}", result)
    }
}

fn parenthesize(name: String, exprs: Vec<&Box<Expr>>) -> String {
    let mut builder = String::new();

    builder.push('(');
    builder.push_str(&name);
    for expr in exprs {
        builder.push(' ');
        builder.push_str(&format!("{}", expr));
    }
    builder.push(')');

    builder
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = match &self {
            Expr::Binary { left, operator, right } => parenthesize(operator.to_string(), vec![left, right]),
            Expr::Grouping { expr } => parenthesize("group".into(), vec![expr]),
            Expr::Literal { value } => value.to_string(),
            Expr::Unary { operator, right } => parenthesize(operator.to_string(), vec![right]),
        };
        write!(f, "{}", result)
    }
}

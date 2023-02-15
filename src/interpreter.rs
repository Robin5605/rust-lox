use crate::{ast::{Expr, LiteralKind}, token::Token, token_type::TokenType};

#[derive(Debug)]
enum Value {
    String(String),
    Float(f32),
    Bool(bool),
    Nil,
}

fn evaluate_grouping(expr: &Expr) -> Value {
    evaluate_expression(&expr)
}

fn evaluate_unary(operator: &Token, right: &Expr) -> Value {
    let right = evaluate_expression(right);
    match &operator.token_type {
        TokenType::Minus => match right {
            Value::Float(f) => Value::Float(f),
            value => panic!("- not supported for {:?}", value),
        },
        TokenType::Bang => Value::Bool(!evaluate_truthy(&right)),
        token_type => panic!("({} {:?}) not supported", token_type, right),
    }
}

fn evaluate_binary(left: &Expr, operator: &Token, right: &Expr) -> Value {
    let left = evaluate_expression(left);
    let right = evaluate_expression(right);
    
    match (left, right) {
        (Value::String(left_value), Value::String(right_value)) => match &operator.token_type {
            TokenType::Plus => Value::String(format!("{left_value} {right_value}")),
            token_type => panic!("Operator {} not supported for {} and {}", token_type, left_value, right_value)
        },

        (Value::String(left_value), Value::Float(right_value)) => match &operator.token_type {
            TokenType::Star => Value::String(left_value.repeat(right_value as usize)),
            token_type => panic!("Operator {} not supported for {} and {}", token_type, left_value, right_value)
        },

        (Value::Float(left_value), Value::String(right_value)) => match &operator.token_type {
            TokenType::Star => Value::String(right_value.repeat(left_value as usize)),
            token_type => panic!("Operator {} not supported for {} and {}", token_type, left_value, right_value)
        }

        (Value::Float(left_value), Value::Float(right_value)) => match &operator.token_type {
                TokenType::Plus => Value::Float(left_value + right_value),
                TokenType::Minus => Value::Float(left_value - right_value),
                TokenType::Star => Value::Float(left_value * right_value),
                TokenType::Slash => Value::Float(left_value / right_value),
                TokenType::EqEq => Value::Bool(left_value == right_value),
                TokenType::BangEq => Value::Bool(left_value != right_value),
                TokenType::Greater => Value::Bool(left_value > right_value),
                TokenType::GreaterEq => Value::Bool(left_value >= right_value),
                TokenType::Less => Value::Bool(left_value < right_value),
                TokenType::LessEq => Value::Bool(left_value <= right_value),
                token_type => panic!("Operator {} not supported for {} and {}", token_type, left_value, right_value),
            },
        (left_value, Value::Nil) => match &operator.token_type {
            token_type => panic!("Operator {} not supported for {:?} and Nil", token_type, left_value)
        },
        (Value::Nil, right_value) => match &operator.token_type {
            token_type => panic!("Operator {} not supported for Nil and {:?}", token_type, right_value)
        },
        (left_value, right_value) => match &operator.token_type {
            token_type => panic!("Operator {} not supported for {:?} and {:?}", token_type, left_value, right_value),
        }
    }
}

fn evaluate_truthy(value: &Value) -> bool {
    match value {
        Value::Bool(bool) => *bool,
        Value::Nil => false,
        _ => true,
    }
}

fn evaluate_literal(literal: &LiteralKind) -> Value {
    match literal {
        LiteralKind::String(v) => Value::String(v.clone()),
        LiteralKind::Float(v) => Value::Float(v.clone()),
        LiteralKind::Bool(v) => Value::Bool(v.clone()),
        LiteralKind::Nil => Value::Nil,
    }
}

fn evaluate_expression(expr: &Expr) -> Value {
    match expr {
        Expr::Binary { left, operator, right } => evaluate_binary(&left, &operator, &right),
        Expr::Grouping { expr } => evaluate_grouping(&expr),
        Expr::Literal { value } => evaluate_literal(value),
        Expr::Unary { operator, right } => evaluate_unary(operator, right),
    }
}

pub fn interpret(expr: &Expr) {
    let value = evaluate_expression(&expr);
    println!("{:?}", value)
}
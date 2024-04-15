use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenize::Token;

#[derive(Debug)]
pub enum Expression {
    Add(Add),
    Mul(Mul),
    Number(i32),
}

#[derive(Debug)]
pub struct Add {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug)]
pub struct Mul {
    left: Box<Expression>,
    right: Box<Expression>,
}

pub type PeekToken<'a> = Peekable<Iter<'a, Token>>;

pub fn parse(tokens: &mut PeekToken) -> Expression {
    return parse_add(tokens);
}

pub fn parse_add(tokens: &mut PeekToken) -> Expression {
    let mut expression = parse_mul(tokens);
    while let Some(&&ref t) = tokens.peek() {
        match t {
            Token::Add => {
                expression = parse_add_tail(expression, tokens);
            }
            _ => {
                return expression;
            }
        }
    }
    return expression;
}

pub fn parse_add_tail(pre_exp: Expression, tokens: &mut PeekToken) -> Expression {
    tokens.next();
    return Expression::Add(Add {
        left: Box::new(pre_exp),
        right: Box::new(parse_mul(tokens)),
    });
}

pub fn parse_mul(tokens: &mut PeekToken) -> Expression {
    let mut expression = match tokens.next().unwrap() {
        Token::Number(n) => {
            Expression::Number(*n)
        }
        _ => { todo!() }
    };

    while let Some(&&ref t) = tokens.peek() {
        match t {
            Token::Mul => {
                expression = parse_mul_tail(expression, tokens);
            }
            _ => {
                return expression;
            }
        }
    };
    return expression;
}

pub fn parse_mul_tail(pre_exp: Expression, tokens: &mut PeekToken) -> Expression {
    tokens.next();
    match tokens.next().unwrap() {
        Token::Number(n) => {
            return Expression::Mul(Mul {
                left: Box::new(pre_exp),
                right: Box::new(Expression::Number(*n)),
            }, );
        }
        _ => {
            todo!();
        }
    };
}

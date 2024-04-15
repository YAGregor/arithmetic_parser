use std::cmp::PartialEq;
use std::iter::Peekable;
use std::slice::Iter;
use crate::tokenize::Token;

#[derive(Debug)]
pub enum Expression {
    Add(Add),
    Mul(Mul),
    Div(Div),
    Sub(Sub),
    Number(i32),
    Power(Power),
}

#[derive(Debug)]
pub struct Power {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug)]
pub struct Add {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug)]
pub struct Sub {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug)]
pub struct Mul {
    left: Box<Expression>,
    right: Box<Expression>,
}

#[derive(Debug)]
pub struct Div {
    left: Box<Expression>,
    right: Box<Expression>,
}

pub type PeekToken<'a> = Peekable<Iter<'a, Token>>;

pub fn parse(tokens: &mut PeekToken) -> Expression {
    return parse_add(tokens);
}

fn is_add_level(token: &Token) -> bool {
    return token == &Token::Add || token == &Token::Sub;
}

pub fn parse_add(tokens: &mut PeekToken) -> Expression {
    let mut expression = parse_mul(tokens);
    while let Some(&&ref t) = tokens.peek() {
        expression = if is_add_level(t) {
            parse_add_tail(expression, tokens)
        } else {
            return expression;
        };
    };
    return expression;
}

pub fn parse_add_tail(pre_exp: Expression, tokens: &mut PeekToken) -> Expression {
    let t = tokens.next().unwrap();
    return if t == &Token::Add {
        Expression::Add(Add {
            left: Box::new(pre_exp),
            right: Box::new(parse_mul(tokens)),
        })
    } else {
        Expression::Sub(Sub {
            left: Box::new(pre_exp),
            right: Box::new(parse_mul(tokens)),
        })
    };
}

fn is_mul_level(token: &Token) -> bool {
    return token == &Token::Mul || token == &Token::Div;
}

pub fn parse_mul(tokens: &mut PeekToken) -> Expression {
    let mut expression = parse_power(tokens);

    while let Some(&&ref t) = tokens.peek() {
        if is_mul_level(t) {
            expression = parse_mul_tail(expression, tokens);
        } else {
            return expression;
        }
    };
    return expression;
}

pub fn parse_mul_tail(pre_exp: Expression, tokens: &mut PeekToken) -> Expression {
    let exp_op = tokens.next().unwrap();
    let right = parse_power(tokens);
    return if (*exp_op == Token::Mul) {
        Expression::Mul(Mul {
            left: Box::new(pre_exp),
            right: Box::new(right),
        })
    } else {
        Expression::Div(Div {
            left: Box::new(pre_exp),
            right: Box::new(right),
        })
    };
}


pub fn parse_power(tokens: &mut PeekToken) -> Expression {
    let expression = parse_atom(tokens);
    return if let Some(&&ref t) = tokens.peek() {
        if (*t == Token::Power) {
            parse_power_tail(expression, tokens)
        } else {
            expression
        }
    } else {
        expression
    };
}

pub fn parse_power_tail(pre_exp: Expression, tokens: &mut PeekToken) -> Expression {
    tokens.next();
    let right = parse_power(tokens);
    return Expression::Power(Power {
        left: Box::new(pre_exp),
        right: Box::new(right),
    });
}

fn parse_atom(tokens: &mut PeekToken) -> Expression {
    return match tokens.peek().unwrap() {
        Token::Number(n) => {
            tokens.next();
            Expression::Number(*n)
        }
        Token::LParen => {
            tokens.next();
            let res = parse(tokens);
            tokens.next();
            res
        }
        _ => {
            todo!()
        }
    };
}

use crate::parse::parse;
use crate::tokenize::{Token, tokenize};

mod tokenize;
mod parse;

fn main() {
    let test_string = String::from("1 * 2 + 3");
    let tokens = tokenize(test_string);
    println!("{:?}", tokens);
    let mut peek_token = tokens.iter().peekable();
    let expression_tree = parse(&mut peek_token);
    println!("{:?}", expression_tree);
}

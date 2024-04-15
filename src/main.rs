use crate::tokenize::{Token, tokenize};

mod tokenize;
mod parse;

fn main() {
    let test_string = String::from("1 + 2 * 3");
    println!("{:?}", tokenize(test_string));
}

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Add,
    Sub,
    Mul,
    Div,
    Fact,
    Number(i32),
}

pub fn tokenize(source: String) -> Vec<Token> {
    let mut res = Vec::<Token>::new();
    let mut numbers = Vec::<char>::new();
    fn clear_numbers(numbers: &mut Vec<char>, res: &mut Vec<Token>) {
        if !numbers.is_empty() {
            let number_string = numbers.iter().collect::<String>();
            let number = number_string.parse::<i32>().unwrap();
            numbers.clear();
            res.push(Token::Number(number));
        }
    }
    for char in source.chars() {
        match char {
            '(' => {
                clear_numbers(&mut numbers, &mut res);
                res.push(Token::LParen);
            }
            ')' => {
                clear_numbers(&mut numbers, &mut res);
                res.push(Token::RParen);
            }
            '+' => {
                clear_numbers(&mut numbers, &mut res);
                res.push(Token::Add);
            }
            '-' => {
                clear_numbers(&mut numbers, &mut res);
                res.push(Token::Sub);
            }
            '*' => {
                clear_numbers(&mut numbers, &mut res);
                res.push(Token::Mul);
            }
            '^' => {
                clear_numbers(&mut numbers, &mut res);
                res.push(Token::Fact);
            }
            '/' => {
                clear_numbers(&mut numbers, &mut res);
                res.push(Token::Div);
            }
            ' ' => {}
            _ => {
                numbers.push(char);
            }
        }
    };
    clear_numbers(&mut numbers, &mut res);
    return res;
}


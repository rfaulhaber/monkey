mod token;
mod word_iter;

use word_iter::WordIter;

pub use token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    word_iter: WordIter<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input,
            word_iter: WordIter::new(input),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.word_iter.next() {
            Some(s) => Some(match s.as_str() {
                "let" => Token::Let,
                "fn" => Token::Fn,
                "return" => Token::Return,
                "true" => Token::True,
                "false" => Token::False,
                "if" => Token::If,
                "else" => Token::Else,
                "{" => Token::Lbrace,
                "}" => Token::Rbrace,
                "(" => Token::Lparen,
                ")" => Token::Rparen,
                "+" => Token::Plus,
                ";" => Token::Semicolon,
                "," => Token::Comma,
                "=" => Token::Assign,
                "-" => Token::Minus,
                "!" => Token::Bang,
                "*" => Token::Asterisk,
                "/" => Token::Slash,
                "<" => Token::Lt,
                ">" => Token::Gt,
                "!=" => Token::Neq,
                "==" => Token::Eq,
                s if parse_number(s).is_some() => Token::Int(parse_number(s).unwrap()), // TODO do better
                s => Token::Identifier(s.to_string()),
            }),
            None => None,
        }
    }
}

fn parse_number(s: &str) -> Option<i64> {
    match s.parse::<i64>().map(|i| i) {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn basic_symbol_lexing() {
        let input = "=+(){},;";

        let output: Vec<Token> = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Lbrace,
            Token::Rbrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let result: Vec<Token> = Lexer::new(input).into_iter().collect();

        assert_eq!(output, result);
    }

    #[test]
    fn iterator_implementation() {
        let input = "let five = 5;";

        let mut lexer = Lexer::new(input);

        assert_eq!(Some(Token::Let), lexer.next());
    }

    #[test]
    fn basic_code_lexing() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);

if (5 < 10) {
   return true;
} else {
   return false;
}

10 == 10;
10 != 9;";

        let output: Vec<Token> = vec![
            Token::Let,
            Token::Identifier("five".into()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".into()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".into()),
            Token::Assign,
            Token::Fn,
            Token::Lparen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::Rparen,
            Token::Lbrace,
            Token::Identifier("x".into()),
            Token::Plus,
            Token::Identifier("y".into()),
            Token::Semicolon,
            Token::Rbrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("add".into()),
            Token::Lparen,
            Token::Identifier("five".into()),
            Token::Comma,
            Token::Identifier("ten".into()),
            Token::Rparen,
            Token::Semicolon,
            Token::If,
            Token::Lparen,
            Token::Int(5),
            Token::Lt,
            Token::Int(10),
            Token::Rparen,
            Token::Lbrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::Rbrace,
            Token::Else,
            Token::Lbrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::Rbrace,
            Token::Int(10),
            Token::Eq,
            Token::Int(10),
            Token::Semicolon,
            Token::Int(10),
            Token::Neq,
            Token::Int(9),
            Token::Semicolon,
        ];

        let result: Vec<Token> = Lexer::new(input).into_iter().collect();

        assert_eq!(output, result);
    }
}

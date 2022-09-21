pub enum Token {
    Illegal,
    EOF,
    Identifier(String),
    Int(i64),
    Assign,
    Plus,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    pos: i64,
    read_pos: i64,
    character: Option<char>,
}

impl Lexer {
    pub fn from_input(input: String) -> Lexer {
        Lexer {
            input,
            pos: 0,
            read_pos: 0,
            character: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_lexing() {
        let input = "=+(){},;";

        let output: Vec<(Token, &str)> = vec![
            (Token::Assign, "="),
            (Token::Plus, "+"),
            (Token::Lparen, "("),
            (Token::Rparen, ")"),
            (Token::Lbrace, "{"),
            (Token::Rbrace, "}"),
            (Token::Comma, ","),
            (Token::Semicolon, ";"),
        ];

}

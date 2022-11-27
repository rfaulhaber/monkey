use std::fmt::Display;

// TODO custom attribute macro that recognizes each symbol from a string
// TODO custom attribute macro that implements both display and can parse from string
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(String),
    EOF,
    Identifier(String),
    Int(i64),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Fn,
    Let,
    Eq,
    Neq,
    If,
    Else,
    Return,
    True,
    False,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal(_) => todo!(),
            Token::EOF => todo!(),
            Token::Identifier(i) => write!(f, "{}", i),
            Token::Int(i) => write!(f, "{}", i),
            Token::Assign => write!(f, "="),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Bang => write!(f, "!"),
            Token::Asterisk => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::Lt => write!(f, "<"),
            Token::Gt => write!(f, ">"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Lparen => write!(f, "("),
            Token::Rparen => write!(f, ")"),
            Token::Lbrace => write!(f, "["),
            Token::Rbrace => write!(f, "]"),
            Token::Fn => write!(f, "fn"),
            Token::Let => write!(f, "let"),
            Token::Eq => write!(f, "="),
            Token::Neq => write!(f, "!="),
            Token::If => write!(f, "if"),
            Token::Else => write!(f, "else"),
            Token::Return => write!(f, "return"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
        }
    }
}

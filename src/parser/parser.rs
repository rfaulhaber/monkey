use super::ast::{Expr, Identifier, Program, Stmt};
use crate::lexer::{Lexer, Token};
use thiserror::Error;

macro_rules! assert_next_token {
    ($lexer:expr, $tok:path) => {
        match $lexer.next() {
            Some($tok) => Ok(()),
            Some(x) => Err(ParseError::UnexpectedToken(x.to_string())),
            _ => Err(ParseError::UnexpectedEndOfInput),
        }
    };
}

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    program: Program,
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
    #[error("unexpected token: expected {expected:?}, found {found:?}")]
    UnexpectedTokenFound { expected: String, found: String },
    #[error("unexpected end of input")]
    UnexpectedEndOfInput,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser {
            lexer,
            program: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Result<Program, ParseError> {
        while let Some(token) = self.lexer.next() {
            match token {
                Token::Let => self.parse_let()?,
                _ => todo!(),
            }
        }

        Ok(self.program)
    }

    fn parse_let(&mut self) -> Result<(), ParseError> {
        let ident = self.parse_ident()?;

        assert_next_token!(self.lexer, Token::Eq)?;

        let expr = self.parse_expr()?;

        self.program.push(Stmt::Let {
            name: ident,
            expr: Box::new(expr),
        });

        Ok(())
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        // match self.lexer.next() {
        //     Some(Token::Int(n)) =>
        // }

        todo!();
    }

    fn parse_ident(&mut self) -> Result<Identifier, ParseError> {
        match self.lexer.next() {
            Some(Token::Identifier(ident)) => Ok(Identifier { value: ident }),
            Some(u) => {
                return Err(ParseError::UnexpectedTokenFound {
                    expected: "identifier".to_owned(),
                    found: u.to_string(),
                })
            }
            _ => return Err(ParseError::UnexpectedEndOfInput),
        }
    }
}

impl<'a> From<Lexer<'a>> for Parser<'a> {
    fn from(l: Lexer<'a>) -> Self {
        Parser::new(l)
    }
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(s: &'a str) -> Self {
        Parser::from(Lexer::from(s))
    }
}

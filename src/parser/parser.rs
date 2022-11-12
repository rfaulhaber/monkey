use crate::lexer::Lexer;

use super::{ast::Statement, program::Program};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer) -> Parser {
        Parser { lexer }
    }

    pub fn parse<S: Statement>(&mut self) -> Program<S> {
        unimplemented!();
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

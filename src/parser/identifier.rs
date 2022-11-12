use crate::lexer::Token;

pub struct Identifier<'a> {
    token: Token,
    value: &'a str,
}

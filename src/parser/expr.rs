use crate::lexer::Token;

use super::{
    ast::{Expression, Node, Statement},
    identifier::Identifier,
};

pub struct Let<'a, E: Expression> {
    token: Token,
    name: Option<Identifier<'a>>,
    value: E,
}

impl<'a, E: Expression> Node for Let<'a, E> {
    fn literal(&self) -> &str {
        todo!()
    }
}

impl<'a, E: Expression> Statement for Let<'a, E> {
    fn statement_node(&self) {
        todo!()
    }
}

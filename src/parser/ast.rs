// TODO track tokens, source info

pub type Program = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let { name: Identifier, expr: Box<Expr> },
    Return(Box<Expr>),
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Identifier(Identifier),
    Integer(i64),
}

// TODO should be single tuple struct?
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub(super) value: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum OperatorPrecedence {
    Lowest,
    Equals,
    LessOrGreater,
    Sum,
    Product,
    Prefix,
    Call
}

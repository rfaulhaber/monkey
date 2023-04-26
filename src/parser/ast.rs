// TODO track tokens, source info

pub type Program = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let { name: Identifier, expr: Box<Expr> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(Identifier),
    Integer(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub(super) value: String,
}

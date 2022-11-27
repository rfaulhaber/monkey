// TODO track tokens, source info

pub type Program = Vec<Stmt>;

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: Identifier, expr: Box<Expr> },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(Identifier),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub(super) value: String,
}

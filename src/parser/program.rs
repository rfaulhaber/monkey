use super::ast::Statement;

pub struct Program<S: Statement> {
    pub(super) statements: Vec<S>,
}

impl<S: Statement> Program<S> {
    pub fn literal(&self) -> &str {
        if self.statements.len() > 0 {
            self.statements.get(0).unwrap().literal()
        } else {
            ""
        }
    }
}

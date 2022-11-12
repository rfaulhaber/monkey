mod ast;
mod expr;
mod identifier;
mod parser;
mod program;

#[cfg(test)]
mod test {
    use crate::parser::ast::Statement;

    use super::{parser::Parser, *};
    use pretty_assertions::assert_eq;

    fn parse_let_statements() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;";

        let parser = Parser::from(input);
        let program = parser.parse::<dyn Statement>();

        assert_eq!(3, program.statements.len());
    }
}

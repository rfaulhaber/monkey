mod ast;
mod parser;

#[cfg(test)]
mod test {
    use crate::parser::ast::Program;

    use super::{parser::Parser, *};
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_let_statements() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;";

        let program = Parser::from(input).parse();
        assert!(
            matches!(program, Ok(_)),
            "Expected program, got: {:?}",
            program
        );
    }
}

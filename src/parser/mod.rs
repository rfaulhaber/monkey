mod ast;
mod parser;

#[cfg(test)]
mod test {
    use crate::parser::{
        ast::{Expr, Identifier, Stmt},
        parser::ParseError,
    };

    use super::parser::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_let_statements() {
        let input = "let x = 5;
let y = 10;
let foobar = 838383;";

        let program_op = Parser::from(input).parse();
        assert!(
            matches!(program_op, Ok(_)),
            "Expected program, got: {:?}",
            program_op
        );

        let program = program_op.unwrap();

        assert_eq!(program.len(), 3);

        let expected_result = vec![
            Stmt::Let {
                expr: Box::new(Expr::Integer(5)),
                name: Identifier {
                    value: String::from("x"),
                },
            },
            Stmt::Let {
                expr: Box::new(Expr::Integer(10)),
                name: Identifier {
                    value: String::from("y"),
                },
            },
            Stmt::Let {
                expr: Box::new(Expr::Integer(838383)),
                name: Identifier {
                    value: String::from("foobar"),
                },
            },
        ];

        assert_eq!(program, expected_result);
    }

    #[test]
    fn parse_let_statement_errors() {
        let input = "let x 5;
let = 10
let 838383";

        let program_op = Parser::from(input).parse();

        assert!(matches!(program_op, Err(_)));
    }
}

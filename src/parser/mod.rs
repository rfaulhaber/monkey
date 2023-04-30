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

    #[test]
    fn parse_return_statements() {
        let input = "return 5;
return 10;
return 993322;";

        let program_op = Parser::from(input).parse();

        let expected = vec![
            Stmt::Return(Box::new(Expr::Integer(5))),
            Stmt::Return(Box::new(Expr::Integer(10))),
            Stmt::Return(Box::new(Expr::Integer(993322))),
        ];

        assert_eq!(expected, program_op.unwrap());
    }

    #[test]
    fn parse_identifier_expr_stmt() {
        let input = "foobar;";

        let expected = vec![
            Stmt::Expr(Box::new(Expr::Identifier(Identifier { value: "foobar".into() })))
        ];

        let program = Parser::from(input).parse().unwrap();

        assert_eq!(expected, program);
    }

    #[test]
    fn parse_integer_expr_stmt() {
        let input = "5;";

        let expected = vec![
            Stmt::Expr(Box::new(Expr::Integer(5))),
        ];

        let program = Parser::from(input).parse().unwrap();

        assert_eq!(expected, program);
    }
}

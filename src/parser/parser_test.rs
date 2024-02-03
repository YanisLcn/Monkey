#[cfg(test)]
pub mod parser_test {
    use std::iter::zip;

    use crate::{
        ast::ast::{
            Expression, Identifier, InfixExpr, LetStatement, PrefixExpr, ReturnStatement, Statement,
        },
        lexer::lexer::Lexer,
        parser::parser::Parser,
        token::token::Token,
    };

    #[test]
    fn test_let_statement() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";
        let expected_statements = vec![
            Statement::LetStatement(LetStatement {
                name: Identifier {
                    value: "x".to_string(),
                },
                value: Expression::Integer(5),
            }),
            Statement::LetStatement(LetStatement {
                name: Identifier {
                    value: "y".to_string(),
                },
                value: Expression::Integer(10),
            }),
            Statement::LetStatement(LetStatement {
                name: Identifier {
                    value: "foobar".to_string(),
                },
                value: Expression::Integer(838383),
            }),
        ];

        test_parsing_statements(input, 0, expected_statements)
    }

    #[test]
    fn test_let_statement_missing_token() {
        let input = "
let x 5;
let = 10;
let 838383;
";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert_eq!(parser.errors().len(), 3);
        check_parse_errors(&parser);
        assert_eq!(program.statements.len(), 0);
    }

    #[test]
    fn test_return_statement() {
        let input = "
return 5;
return 10;
return 838383;
";
        let expected_statements = vec![
            Statement::ReturnStatement(ReturnStatement {
                value: Expression::Integer(5),
            }),
            Statement::ReturnStatement(ReturnStatement {
                value: Expression::Integer(10),
            }),
            Statement::ReturnStatement(ReturnStatement {
                value: Expression::Integer(838383),
            }),
        ];
        test_parsing_statements(input, 0, expected_statements)
    }

    #[test]
    fn test_let_statement_display() {
        let input = "let  myvar  =  anothervar;";
        let expected = "let myvar = anothervar;";
        test_parsing_display_format(input, expected);
    }

    #[test]
    fn test_ident_expr() {
        let input = "foobar;";

        let expected_statements = vec![Statement::ExpressionStatement(Expression::Identifier(
            Identifier {
                value: "foobar".to_string(),
            },
        ))];

        test_parsing_statements(input, 0, expected_statements)
    }

    #[test]
    fn test_integer() {
        let input = "5;";

        let expected_statements = vec![Statement::ExpressionStatement(Expression::Integer(5))];

        test_parsing_statements(input, 0, expected_statements)
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let input = "!5; -15;";

        let expected_statements = vec![
            Statement::ExpressionStatement(Expression::Prefix(PrefixExpr::new(
                Token::BANG,
                Expression::Integer(5),
            ))),
            Statement::ExpressionStatement(Expression::Prefix(PrefixExpr::new(
                Token::SUB,
                Expression::Integer(15),
            ))),
        ];

        test_parsing_statements(input, 0, expected_statements);
    }

    #[test]
    fn test_parsing_infix_expression() {
        let input = "1 + 3; 3 - 4; 12 * 12; 1 / 2; 6 > 5; 6 < 5; 1 == 2; 0 != 0;";

        let expected_statements = vec![
            build_infix_expr_statement(Token::PLUS, Expression::Integer(1), Expression::Integer(3)),
            build_infix_expr_statement(Token::SUB, Expression::Integer(3), Expression::Integer(4)),
            build_infix_expr_statement(
                Token::MUL,
                Expression::Integer(12),
                Expression::Integer(12),
            ),
            build_infix_expr_statement(Token::DIV, Expression::Integer(1), Expression::Integer(2)),
            build_infix_expr_statement(Token::GT, Expression::Integer(6), Expression::Integer(5)),
            build_infix_expr_statement(Token::LT, Expression::Integer(6), Expression::Integer(5)),
            build_infix_expr_statement(Token::EQ, Expression::Integer(1), Expression::Integer(2)),
            build_infix_expr_statement(Token::NE, Expression::Integer(0), Expression::Integer(0)),
        ];

        test_parsing_statements(input, 0, expected_statements);
    }

    #[test]
    fn test_bool() {
        let input = "true; false; let foobar = true; let barfoo = false;";

        let expected_statements = vec![
            Statement::ExpressionStatement(Expression::Bool(true)),
            Statement::ExpressionStatement(Expression::Bool(false)),
            Statement::LetStatement(LetStatement {
                name: Identifier {
                    value: "foobar".to_string(),
                },
                value: Expression::Bool(true),
            }),
            Statement::LetStatement(LetStatement {
                name: Identifier {
                    value: "barfoo".to_string(),
                },
                value: Expression::Bool(false),
            }),
        ];

        test_parsing_statements(input, 0, expected_statements)
    }

    #[test]
    fn test_parsing_operator_precedence_display() {
        let input_expect = vec![
            ("-a * b", "((-a) * b);"),
            ("!-a", "(!(-a));"),
            ("a + b + c", "((a + b) + c);"),
            ("a + b - c", "((a + b) - c);"),
            ("a * b * c", "((a * b) * c);"),
            ("a * b / c", "((a * b) / c);"),
            ("a + b / c", "(a + (b / c));"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f);"),
            ("3 + 4; -5 * 5", "(3 + 4);((-5) * 5);"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4));"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4));"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)));",
            ),
            ("3 > 5 == false", "((3 > 5) == false);"),
            ("true == 3 < 5", "(true == (3 < 5));"),
        ];

        input_expect
            .iter()
            .for_each(|(i, e)| test_parsing_display_format(i, e));
    }

    fn build_infix_expr_statement(token: Token, left: Expression, right: Expression) -> Statement {
        Statement::ExpressionStatement(Expression::Infix(InfixExpr::new(token, left, right)))
    }

    fn test_parsing_statements(input: &str, errors: usize, expected_statements: Vec<Statement>) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);
        assert_eq!(parser.errors().len(), errors);
        assert_eq!(program.statements.len(), expected_statements.len());

        zip(program.statements, expected_statements)
            .into_iter()
            .for_each(|(stmt, expect_stmt)| {
                assert_eq!(stmt, expect_stmt);
            });
    }

    fn test_parsing_display_format(input: &str, expected_format: &str) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(format!("{program}"), expected_format);
    }

    fn check_parse_errors(parser: &Parser) {
        parser
            .errors()
            .iter()
            .for_each(|error| println!("{}", error));
    }
}

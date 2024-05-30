#[cfg(test)]
pub mod parser_test {
    use crate::{
        ast::ast::{
            Arrays, CallExpression, Expression, FnExpression, Identifier, IfExpression, InfixExpr,
            LetStatement, PrefixExpr, ReturnStatement, Statement,
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
    fn test_string() {
        let input = "\"hello world\"";

        let expected_statements = vec![Statement::ExpressionStatement(Expression::String(
            "hello world".to_string(),
        ))];

        test_parsing_statements(input, 0, expected_statements)
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let input = "!5; -15; !false";

        let expected_statements = vec![
            Expression::Prefix(PrefixExpr::new(Token::BANG, Expression::Integer(5))),
            Expression::Prefix(PrefixExpr::new(Token::SUB, Expression::Integer(15))),
            Expression::Prefix(PrefixExpr::new(Token::BANG, Expression::Bool(false))),
        ]
        .iter()
        .map(|expr| build_stmt_from_expr(expr.clone()))
        .collect();

        test_parsing_statements(input, 0, expected_statements);
    }

    fn build_int_int_infix(token: Token, a: i32, b: i32) -> Expression {
        build_infix_expr(token, Expression::Integer(a), Expression::Integer(b))
    }

    fn build_ident_ident_infix(token: Token, a: &str, b: &str) -> Expression {
        build_infix_expr(token, build_ident_expr(a), build_ident_expr(b))
    }

    fn build_bool_bool_infix(token: Token, a: bool, b: bool) -> Expression {
        build_infix_expr(token, Expression::Bool(a), Expression::Bool(b))
    }

    fn build_stmt_from_expr(expr: Expression) -> Statement {
        Statement::ExpressionStatement(expr)
    }

    #[test]
    fn test_parsing_infix_expression() {
        let input = "1 + 3; 3 - 4; 12 * 12; 1 / 2; 6 > 5; 6 < 5; 1 == 2; 0 != 0; true == true; true != false; false == false;";

        let expected_statements = vec![
            build_int_int_infix(Token::PLUS, 1, 3),
            build_int_int_infix(Token::SUB, 3, 4),
            build_int_int_infix(Token::MUL, 12, 12),
            build_int_int_infix(Token::DIV, 1, 2),
            build_int_int_infix(Token::GT, 6, 5),
            build_int_int_infix(Token::LT, 6, 5),
            build_int_int_infix(Token::EQ, 1, 2),
            build_int_int_infix(Token::NE, 0, 0),
            build_bool_bool_infix(Token::EQ, true, true),
            build_bool_bool_infix(Token::NE, true, false),
            build_bool_bool_infix(Token::EQ, false, false),
        ]
        .iter()
        .map(|expr| build_stmt_from_expr(expr.clone()))
        .collect();

        test_parsing_statements(input, 0, expected_statements);
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";

        let expected_statements = vec![build_stmt_from_expr(build_if_expr(
            build_ident_ident_infix(Token::LT, "x", "y"),
            vec![build_stmt_from_expr(build_ident_expr("x"))],
            None,
        ))];

        test_parsing_statements(input, 0, expected_statements)
    }
    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";

        let expected_statements = vec![build_stmt_from_expr(build_if_expr(
            build_ident_ident_infix(Token::LT, "x", "y"),
            vec![build_stmt_from_expr(build_ident_expr("x"))],
            Some(vec![build_stmt_from_expr(build_ident_expr("y"))]),
        ))];

        test_parsing_statements(input, 0, expected_statements)
    }

    #[test]
    fn test_function_literals() {
        let input = "fn(x, y) { x + y; }";

        let expected_statements = vec![build_stmt_from_expr(build_fn_expr(
            vec!["x", "y"],
            vec![build_stmt_from_expr(build_infix_expr(
                Token::PLUS,
                build_ident_expr("x"),
                build_ident_expr("y"),
            ))],
        ))];

        test_parsing_statements(input, 0, expected_statements);
    }

    fn vec_str_to_ident(v: Vec<&str>) -> Vec<Identifier> {
        v.iter()
            .map(|x| Identifier {
                value: x.to_string(),
            })
            .collect()
    }

    #[test]
    fn test_function_parameters() {
        check_function_parameters("fn () {}", 0, vec_str_to_ident(vec![]));
        check_function_parameters("fn (x) {}", 0, vec_str_to_ident(vec!["x"]));
        check_function_parameters("fn (x, y, z) {}", 0, vec_str_to_ident(vec!["x", "y", "z"]));
    }

    #[test]
    fn test_call_expression() {
        let input = "add(1, 2 + 3, 4 * 5);";

        let expected = vec![build_stmt_from_expr(Expression::CallExpression(
            CallExpression {
                function: Box::new(build_ident_expr("add")),
                arguments: vec![
                    Expression::Integer(1),
                    build_int_int_infix(Token::PLUS, 2, 3),
                    build_int_int_infix(Token::MUL, 4, 5),
                ],
            },
        ))];

        test_parsing_statements(input, 0, expected);
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
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4);"),
            ("(5 + 5) * 2", "((5 + 5) * 2);"),
            ("2 / (5 + 5)", "(2 / (5 + 5));"),
            ("-(5 + 5)", "(-(5 + 5));"),
            ("!(true == true)", "(!(true == true));"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d);"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)));",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g));",
            ),
        ];

        input_expect
            .iter()
            .for_each(|(i, e)| test_parsing_display_format(i, e));
    }

    #[test]
    fn parse_arrays() {
        let input = "[1, 2 * 2, 3 + 3]";

        let expected = vec![build_stmt_from_expr(Expression::Arrays(Arrays {
            elements: vec![
                Expression::Integer(1),
                Expression::Infix(InfixExpr {
                    left_expr: Box::new(Expression::Integer(2)),
                    operator: Token::MUL,
                    right_expr: Box::new(Expression::Integer(2)),
                }),
                Expression::Infix(InfixExpr {
                    left_expr: Box::new(Expression::Integer(3)),
                    operator: Token::PLUS,
                    right_expr: Box::new(Expression::Integer(3)),
                }),
            ],
        }))];

        test_parsing_statements(input, 0, expected);
    }

    fn build_ident_expr(name: &str) -> Expression {
        Expression::Identifier(Identifier {
            value: name.to_string(),
        })
    }

    fn build_infix_expr(token: Token, left: Expression, right: Expression) -> Expression {
        Expression::Infix(InfixExpr::new(token, left, right))
    }

    fn build_if_expr(
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Option<Vec<Statement>>,
    ) -> Expression {
        Expression::IfExpression(IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn build_fn_expr(par: Vec<&str>, body: Vec<Statement>) -> Expression {
        Expression::FnExpression(FnExpression {
            parameters: par
                .iter()
                .map(|&s| Identifier {
                    value: s.to_string(),
                })
                .collect::<Vec<Identifier>>(),
            body,
        })
    }

    fn test_parsing_statements(input: &str, errors: usize, expected_statements: Vec<Statement>) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);
        assert_eq!(parser.errors().len(), errors);
        assert_eq!(program.statements.len(), expected_statements.len());
        assert_eq!(program.statements, expected_statements);
    }

    fn check_function_parameters(input: &str, errors: usize, expected_parameters: Vec<Identifier>) {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);
        assert_eq!(parser.errors().len(), errors);
        assert!(program.statements.len() > 0);

        if let Statement::ExpressionStatement(Expression::FnExpression(func)) =
            program.statements.first().unwrap()
        {
            assert_eq!(func.parameters, expected_parameters);
        }
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

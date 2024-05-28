#[cfg(test)]
pub mod evaluator_test {
    use crate::{
        evaluator::evaluator::Evaluator, lexer::lexer::Lexer, object::object::Object,
        parser::parser::Parser,
    };

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let mut evaluator = Evaluator::new();
        evaluator.eval(parser.parse_program())
    }

    fn eval_integer_object(obj: Object, expected_value: i32) {
        assert_eq!(obj, Object::INTEGER(expected_value));
    }

    fn eval_boolean_object(obj: Object, expected_value: bool) {
        assert_eq!(obj, Object::BOOLEAN(expected_value));
    }

    #[test]
    fn eval_integer_expression() {
        let input_expctdvalue = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("0", 0),
            ("-0", 0),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_integer_object(test_eval(i), *v));
    }

    #[test]
    fn eval_boolean_expression() {
        let input_expctdvalue = vec![
            ("false", false),
            ("true", true),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_boolean_object(test_eval(i), *v));
    }

    #[test]
    fn eval_bang_operator() {
        let input_expctdvalue = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!0", true),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
            ("!!0", false),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_boolean_object(test_eval(i), *v));
    }

    #[test]
    fn eval_string() {
        let input_expctdvalue = vec![("\"hello world\"", "hello world")];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| assert_eq!(test_eval(i), Object::STRING(v.to_string())));
    }

    #[test]
    fn eval_string_concatenation() {
        let input_expctdvalue = vec![("\"hello\" + \" \" + \"world\"", "hello world")];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| assert_eq!(test_eval(i), Object::STRING(v.to_string())));
    }

    #[test]
    fn eval_if_expression() {
        let input_expctdvalue = vec![
            ("if (true) { 10 }", Object::INTEGER(10)),
            ("if (false) { 10 }", Object::NULL),
            ("if (1) { 10 }", Object::INTEGER(10)),
            ("if (1 < 2) { 10 }", Object::INTEGER(10)),
            ("if (1 > 2) { 10 }", Object::NULL),
            ("if (1 > 2) { 10 } else { 20 }", Object::INTEGER(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::INTEGER(10)),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| assert_eq!(test_eval(i), *v));
    }

    #[test]
    fn eval_return_statement() {
        let input_expctdvalue = vec![
            ("9; 8;", 8),
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 6;", 10),
        ];

        input_expctdvalue
            .iter()
            .map(|(i, v)| (test_eval(i), v))
            .map(|(i, v)| {
                (
                    match i {
                        Object::RETURN(r) => *r,
                        obj => obj,
                    },
                    v,
                )
            })
            .for_each(|(i, v)| eval_integer_object(i, *v));
    }

    #[test]
    fn eval_error_handling() {
        let input_expctdvalue = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { if (10 > 1) { return true + false; } return 1; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
            ("\"hello\" - \"world\"", "unknown operator: STRING - STRING"),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| assert_eq!(test_eval(i), Object::ERROR(v.to_string())));
    }

    #[test]
    fn eval_let_statement() {
        let input_expctdvalue = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_integer_object(test_eval(i), *v))
    }

    #[test]
    fn eval_let_function_expression() {
        let input_expctdvalue = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
            ("fn(x) { x; }(5)", 5),
        ];

        input_expctdvalue
            .iter()
            .for_each(|(i, v)| eval_integer_object(test_eval(i), *v));
    }

    #[test]
    fn eval_function_expression() {
        let input_expctdvalues = vec![("fn(x) { x + 2; };", 1, vec!["x"], "(x + 2);")];

        input_expctdvalues.iter().for_each(|(f, np, p, b)| {
            assert!(eval_function_support(
                test_eval(f),
                *np,
                p.iter().map(|s| s.to_string()).collect(),
                b.to_string(),
            )
            .is_ok())
        });
    }

    fn eval_function_support(
        obj: Object,
        num_param: usize,
        param: Vec<String>,
        body: String,
    ) -> Result<(), &'static str> {
        match obj {
            Object::FUNCTION(f) => {
                assert_eq!(f.parameters.len(), num_param);
                assert_eq!(
                    f.parameters
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<String>>(),
                    param
                );
                assert_eq!(
                    f.body
                        .iter()
                        .map(|b| b.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    body
                );
                Ok(())
            }
            _ => Err("Should be a function"),
        }
    }

    #[test]
    fn eval_closure() {
        let input = "let newAdder = fn(x) { fn(y) { x + y }; };
let addTwo = newAdder(2);
addTwo(2);";

        eval_integer_object(test_eval(input), 4);
    }
}

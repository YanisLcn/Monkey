#[cfg(test)]
pub mod evaluator_test {
    use crate::{
        evaluator::evaluator::eval, lexer::lexer::Lexer, object::object::Object,
        parser::parser::Parser,
    };

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        eval(parser.parse_program())
    }

    fn eval_integer_object(obj: Object, expected_value: i32) {
        assert_eq!(obj, Object::INTEGER(expected_value));
    }

    fn eval_boolean_object(obj: Object, expected_value: bool) {
        assert_eq!(obj, Object::BOOLEAN(expected_value));
    }

    #[test]
    fn eval_integer_expression() {
        let input_exptcdvalue = vec![
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

        input_exptcdvalue
            .iter()
            .for_each(|(i, v)| eval_integer_object(test_eval(i), *v));
    }

    #[test]
    fn eval_boolean_expression() {
        let input_exptcdvalue = vec![("false", false), ("true", true)];

        input_exptcdvalue
            .iter()
            .for_each(|(i, v)| eval_boolean_object(test_eval(i), *v));
    }

    #[test]
    fn eval_bang_operator() {
        let input_exptcdvalue = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!0", true),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
            ("!!0", false),
        ];

        input_exptcdvalue
            .iter()
            .for_each(|(i, v)| eval_boolean_object(test_eval(i), *v));
    }
}

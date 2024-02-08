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

    #[test]
    fn eval_integer_expression() {
        let input_exptcdvalue = vec![("5", 5), ("10", 10)];

        input_exptcdvalue
            .iter()
            .for_each(|(i, v)| eval_integer_object(test_eval(i), *v));
    }
}

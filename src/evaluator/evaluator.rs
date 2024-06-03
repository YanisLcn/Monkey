use std::{cell::RefCell, rc::Rc, string::String, usize};

use crate::{
    ast::ast::{
        Expression::{self, *},
        Identifier, IfExpression, Program,
        Statement::{self, *},
    },
    object::{
        builtin::BuiltinFunction,
        env::Environment,
        object::{Function, Object},
    },
    token::token::Token,
};

const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);
const NULL: Object = Object::NULL;

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn eval(&mut self, node: Program) -> Object {
        match self.eval_statement_vec(node.statements) {
            Object::RETURN(r) => *r,
            o => o,
        }
    }

    pub fn eval_statement_vec(&mut self, nodes: Vec<Statement>) -> Object {
        let mut last = NULL;
        for stmt in nodes.iter() {
            let evaluated = self.eval_statement(stmt.clone());
            match evaluated {
                Object::RETURN(_) => {
                    return evaluated;
                }
                Object::ERROR(e) => {
                    return Object::ERROR(e);
                }
                _ => (),
            }
            last = evaluated;
        }
        last
    }

    fn eval_statement(&mut self, node: Statement) -> Object {
        match node {
            LetStatement(let_statement) => {
                let evaluated = self.eval_expression(let_statement.value);
                match evaluated {
                    Object::ERROR(e) => return Object::ERROR(e),
                    _ => (),
                };
                self.env
                    .borrow_mut()
                    .set(let_statement.name.value, evaluated.clone());
                evaluated
            }
            ReturnStatement(return_statement) => {
                Object::RETURN(Box::new(self.eval_expression(return_statement.value)))
            }
            ExpressionStatement(expression_statement) => self.eval_expression(expression_statement),
        }
    }

    fn eval_expression(&mut self, node: Expression) -> Object {
        match node {
            Identifier(i) => self.eval_identifier(i),
            Integer(i) => Object::INTEGER(i),
            Bool(b) => self.native_bool_to_object(b),
            String(s) => Object::STRING(s),
            Prefix(p) => match self.eval_expression(*p.expr) {
                Object::ERROR(e) => Object::ERROR(e),
                obj => self.eval_prefix_expression(p.operator, obj),
            },
            Infix(i) => {
                let left_expr = self.eval_expression(*i.left_expr);
                if let Object::ERROR(_) = left_expr {
                    return left_expr;
                };

                let right_expr = self.eval_expression(*i.right_expr);
                if let Object::ERROR(_) = right_expr {
                    return right_expr;
                };

                self.eval_infix_expression(i.operator, left_expr, right_expr)
            }
            IfExpression(if_expr) => self.eval_if_expression(if_expr),
            FnExpression(fun) => Object::FUNCTION(Function {
                parameters: fun.parameters,
                body: fun.body,
                env: Rc::clone(&self.env),
            }),
            CallExpression(c) => {
                let evaluated = self.eval_expression(*c.function);
                if self.is_error(&evaluated) {
                    return evaluated;
                }

                let args = self.eval_arguments(c.arguments);

                if args.len() == 1 && self.is_error(&args.first().unwrap()) {
                    return args.first().unwrap().clone();
                }

                self.apply_function(&evaluated, args)
            }
            Arrays(a) => {
                let elements = self.eval_arguments(a.elements);

                if elements.len() == 1 && self.is_error(&elements.first().unwrap()) {
                    return elements.first().unwrap().clone();
                }

                return Object::ARRAY(elements);
            }
            Indexed(i) => {
                let left = self.eval_expression(*i.left_expr);
                if self.is_error(&left) {
                    return left;
                }
                let index = self.eval_expression(*i.index);
                if self.is_error(&index) {
                    return index;
                }
                return self.eval_index_expression(left, index);
            }
        }
    }

    fn eval_arguments(&mut self, args: Vec<Expression>) -> Vec<Object> {
        args.iter()
            .map(|e| self.eval_expression(e.clone()))
            .collect()
    }

    fn apply_function(&mut self, func: &Object, args: Vec<Object>) -> Object {
        match func {
            Object::FUNCTION(f) => {
                let extended_env = self.extended_func_env(&f, args);
                let old_env = Rc::clone(&self.env);
                self.env = Rc::new(RefCell::new(extended_env));
                let evaluated = self.eval_statement_vec(f.clone().body);
                self.env = old_env;
                self.unwrap_return_value(evaluated)
            }
            Object::BUILTIN(builtin) => builtin.call(args),
            _ => Object::ERROR(format!("not a function : {}", func.get_type())),
        }
    }

    fn extended_func_env(&mut self, func: &Function, args: Vec<Object>) -> Environment {
        let mut env = Environment::new_enclosed(Rc::clone(&func.env));
        func.parameters
            .iter()
            .zip(args.iter())
            .for_each(|(ident, arg)| env.set(ident.value.clone(), arg.clone()));
        env
    }

    fn unwrap_return_value(&mut self, obj: Object) -> Object {
        match obj {
            Object::RETURN(r) => *r,
            o => o,
        }
    }

    fn eval_prefix_expression(&mut self, operator: Token, object: Object) -> Object {
        match operator {
            Token::BANG => self.eval_bang_expression(object),
            Token::SUB => self.eval_minus_expression(object),
            _ => Object::ERROR(format!(
                "unknown operator: {}{}",
                operator,
                object.get_type()
            )),
        }
    }

    fn eval_infix_expression(
        &mut self,
        operator: Token,
        object_left: Object,
        object_right: Object,
    ) -> Object {
        match (object_left, object_right) {
            (Object::INTEGER(a), Object::INTEGER(b)) => {
                self.eval_integer_infix_expression(operator, a, b)
            }
            (Object::BOOLEAN(a), Object::BOOLEAN(b)) => {
                self.eval_boolean_infix_expression(operator, a, b)
            }
            (Object::STRING(a), Object::STRING(b)) => {
                self.eval_string_infix_expression(operator, a, b)
            }
            (s, t) if !PartialEq::eq(&s, &t) => Object::ERROR(format!(
                "type mismatch: {} {} {}",
                s.get_type(),
                operator,
                t.get_type()
            )),
            (left, right) => Object::ERROR(format!(
                "unknown operator: {} {} {}",
                left.get_type(),
                operator,
                right.get_type()
            )),
        }
    }

    fn eval_boolean_infix_expression(&mut self, operator: Token, a: bool, b: bool) -> Object {
        match operator {
            Token::EQ => Object::BOOLEAN(a == b),
            Token::NE => Object::BOOLEAN(a != b),
            _ => Object::ERROR(format!("unknown operator: BOOLEAN {operator} BOOLEAN")),
        }
    }

    fn eval_integer_infix_expression(&mut self, operator: Token, a: i32, b: i32) -> Object {
        match operator {
            Token::PLUS => Object::INTEGER(a + b),
            Token::SUB => Object::INTEGER(a - b),
            Token::MUL => Object::INTEGER(a * b),
            Token::DIV => Object::INTEGER(a / b),
            Token::EQ => self.native_bool_to_object(a == b),
            Token::NE => self.native_bool_to_object(a != b),
            Token::GT => self.native_bool_to_object(a > b),
            Token::LT => self.native_bool_to_object(a < b),
            _ => Object::ERROR(format!("unknown operator: INTEGER {} INTEGER", operator)),
        }
    }

    fn eval_string_infix_expression(&mut self, operator: Token, a: String, b: String) -> Object {
        match operator {
            Token::PLUS => Object::STRING(a + &b),
            _ => Object::ERROR(format!("unknown operator: STRING {} STRING", operator)),
        }
    }

    fn eval_if_expression(&mut self, if_expr: IfExpression) -> Object {
        let condition = self.eval_expression(*if_expr.condition);

        if self.is_true(condition) == TRUE {
            self.eval_statement_vec(if_expr.consequence)
        } else if if_expr.alternative.is_some() {
            self.eval_statement_vec(if_expr.alternative.unwrap())
        } else {
            NULL
        }
    }

    fn eval_bang_expression(&mut self, object: Object) -> Object {
        match self.is_true(object) {
            FALSE => TRUE,
            _ => FALSE,
        }
    }

    fn eval_minus_expression(&mut self, object: Object) -> Object {
        match object {
            Object::INTEGER(i) => Object::INTEGER(-i),
            obj => Object::ERROR(format!("unknown operator: -{}", obj.get_type())),
        }
    }

    fn eval_identifier(&mut self, ident: Identifier) -> Object {
        match self.env.borrow().get(&ident.value) {
            Some(result) => return result,
            None => (),
        }

        match BuiltinFunction::get_builtin(&ident.value) {
            Some(result) => result,
            None => return Object::ERROR(format!("identifier not found: {}", ident.value)),
        }
    }

    fn eval_index_expression(&mut self, left: Object, index: Object) -> Object {
        match (left, index) {
            (Object::ARRAY(a), Object::INTEGER(i)) => self.eval_array_index_expression(a, i),
            (obj, _) => Object::ERROR(format!("index operator not supported for {obj}.")),
        }
    }

    fn eval_array_index_expression(&mut self, left: Vec<Object>, index: i32) -> Object {
        if index < 0 || index > (left.len() - 1).try_into().unwrap() {
            return Object::NULL;
        } else {
            left.get(index as usize).unwrap().clone()
        }
    }

    fn is_true(&mut self, object: Object) -> Object {
        match object {
            FALSE | Object::NULL | Object::INTEGER(0) => FALSE,
            _ => TRUE,
        }
    }

    fn is_error(&mut self, object: &Object) -> bool {
        match object {
            Object::ERROR(_) => true,
            _ => false,
        }
    }

    fn native_bool_to_object(&mut self, b: bool) -> Object {
        if b {
            TRUE
        } else {
            FALSE
        }
    }
}

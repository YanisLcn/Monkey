use crate::{
    ast::ast::{
        Expression::{self, *},
        Identifier, IfExpression, Program,
        Statement::{self, *},
    },
    object::{
        env::Environment,
        object::{Function, Object},
    },
    token::token::Token,
};

const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);
const NULL: Object = Object::NULL;

pub fn eval(node: Program, env: &mut Environment) -> Object {
    eval_statement_vec(node.statements, env)
}

fn eval_statement_vec(nodes: Vec<Statement>, env: &mut Environment) -> Object {
    let mut last = NULL;
    for stmt in nodes.iter() {
        let evaluated = eval_statement(stmt.clone(), env);
        match evaluated {
            Object::RETURN(r) => {
                return *r;
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

fn eval_statement(node: Statement, env: &mut Environment) -> Object {
    match node {
        LetStatement(let_statement) => {
            let evaluated = eval_expression(let_statement.value, env);
            match evaluated {
                Object::ERROR(e) => return Object::ERROR(e),
                _ => (),
            };
            env.set(let_statement.name.value, evaluated.clone());
            evaluated
        }
        ReturnStatement(return_statement) => {
            Object::RETURN(Box::new(eval_expression(return_statement.value, env)))
        }
        ExpressionStatement(expression_statement) => eval_expression(expression_statement, env),
    }
}

fn eval_expression(node: Expression, env: &mut Environment) -> Object {
    match node {
        Identifier(i) => eval_identifier(i, env),
        Integer(i) => Object::INTEGER(i),
        Bool(b) => native_bool_to_object(b),
        Prefix(p) => match eval_expression(*p.expr, env) {
            Object::ERROR(e) => Object::ERROR(e),
            obj => eval_prefix_expression(p.operator, obj),
        },
        Infix(i) => {
            let left_expr = eval_expression(*i.left_expr, env);
            if let Object::ERROR(_) = left_expr {
                return left_expr;
            };

            let right_expr = eval_expression(*i.right_expr, env);
            if let Object::ERROR(_) = right_expr {
                return right_expr;
            };

            eval_infix_expression(i.operator, left_expr, right_expr)
        }
        IfExpression(if_expr) => eval_if_expression(if_expr, env),
        FnExpression(fun) => Object::FUNCTION(Function {
            parameters: fun.parameters,
            body: fun.body,
            env: env.clone(),
        }),
        CallExpression(c) => match eval_expression(*c.function, env) {
            Object::ERROR(e) => Object::ERROR(e),
            _ => todo!(),
        },
    }
}

fn eval_prefix_expression(operator: Token, object: Object) -> Object {
    match operator {
        Token::BANG => eval_bang_expression(object),
        Token::SUB => eval_minus_expression(object),
        _ => Object::ERROR(format!(
            "unknown operator: {}{}",
            operator,
            object.get_type()
        )),
    }
}

fn eval_infix_expression(operator: Token, object_left: Object, object_right: Object) -> Object {
    match (object_left, object_right) {
        (Object::INTEGER(a), Object::INTEGER(b)) => eval_integer_infix_expression(operator, a, b),
        (Object::BOOLEAN(a), Object::BOOLEAN(b)) => eval_boolean_infix_expression(operator, a, b),
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

fn eval_boolean_infix_expression(operator: Token, a: bool, b: bool) -> Object {
    match operator {
        Token::EQ => Object::BOOLEAN(a == b),
        Token::NE => Object::BOOLEAN(a != b),
        _ => Object::ERROR(format!("unknown operator: BOOLEAN {operator} BOOLEAN")),
    }
}

fn eval_integer_infix_expression(operator: Token, a: i32, b: i32) -> Object {
    match operator {
        Token::PLUS => Object::INTEGER(a + b),
        Token::SUB => Object::INTEGER(a - b),
        Token::MUL => Object::INTEGER(a * b),
        Token::DIV => Object::INTEGER(a / b),
        Token::EQ => native_bool_to_object(a == b),
        Token::NE => native_bool_to_object(a != b),
        Token::GT => native_bool_to_object(a > b),
        Token::LT => native_bool_to_object(a < b),
        _ => Object::ERROR(format!("unknown operator: INTEGER {} INTEGER", operator)),
    }
}

fn eval_if_expression(if_expr: IfExpression, env: &mut Environment) -> Object {
    let condition = eval_expression(*if_expr.condition, env);

    if is_true(condition) == TRUE {
        eval_statement_vec(if_expr.consequence, env)
    } else if if_expr.alternative.is_some() {
        eval_statement_vec(if_expr.alternative.unwrap(), env)
    } else {
        NULL
    }
}

fn eval_bang_expression(object: Object) -> Object {
    match is_true(object) {
        FALSE => TRUE,
        _ => FALSE,
    }
}

fn eval_minus_expression(object: Object) -> Object {
    match object {
        Object::INTEGER(i) => Object::INTEGER(-i),
        obj => Object::ERROR(format!("unknown operator: -{}", obj.get_type())),
    }
}

fn eval_identifier(ident: Identifier, env: &mut Environment) -> Object {
    env.get(ident.value.clone())
        .unwrap_or(&Object::ERROR(format!(
            "identifier not found: {}",
            ident.value
        )))
        .clone()
}

fn is_true(object: Object) -> Object {
    match object {
        FALSE | Object::NULL | Object::INTEGER(0) => FALSE,
        _ => TRUE,
    }
}

fn native_bool_to_object(b: bool) -> Object {
    if b {
        TRUE
    } else {
        FALSE
    }
}

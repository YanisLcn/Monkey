use crate::{
    ast::ast::{
        Expression::{self, *},
        IfExpression, Program,
        Statement::{self, *},
    },
    object::object::Object,
    token::token::Token,
};

const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);
const NULL: Object = Object::NULL;

pub fn eval(node: Program) -> Object {
    eval_statement_vec(node.statements)
}

fn eval_statement_vec(nodes: Vec<Statement>) -> Object {
    match nodes.iter().map(|stmt| eval_statement(stmt.clone())).last() {
        Some(obj) => obj,
        None => NULL,
    }
}

fn eval_statement(node: Statement) -> Object {
    match node {
        LetStatement(_) => todo!(),
        ReturnStatement(_) => todo!(),
        ExpressionStatement(expression) => eval_expression(expression),
    }
}

fn eval_expression(node: Expression) -> Object {
    match node {
        Identifier(_) => todo!(),
        Integer(i) => Object::INTEGER(i),
        Bool(b) => native_bool_to_object(b),
        Prefix(p) => eval_prefix_expression(p.operator, eval_expression(*p.expr)),
        Infix(i) => eval_infix_expression(
            i.operator,
            eval_expression(*i.left_expr),
            eval_expression(*i.right_expr),
        ),
        IfExpression(if_expr) => eval_if_expression(if_expr),
        FnExpression(_) => todo!(),
        CallExpression(_) => todo!(),
    }
}

fn eval_prefix_expression(operator: Token, object: Object) -> Object {
    match operator {
        Token::BANG => eval_bang_expression(object),
        Token::SUB => eval_minus_expression(object),
        _ => todo!(),
    }
}

fn eval_infix_expression(operator: Token, object_left: Object, object_right: Object) -> Object {
    match (object_left, object_right) {
        (Object::INTEGER(a), Object::INTEGER(b)) => eval_integer_infix_expression(operator, a, b),
        (Object::BOOLEAN(a), Object::BOOLEAN(b)) => eval_boolean_infix_expression(operator, a, b),
        _ => NULL,
    }
}

fn eval_boolean_infix_expression(operator: Token, a: bool, b: bool) -> Object {
    match operator {
        Token::EQ => Object::BOOLEAN(a == b),
        Token::NE => Object::BOOLEAN(a != b),
        _ => NULL,
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
        _ => NULL,
    }
}

fn eval_if_expression(if_expr: IfExpression) -> Object {
    let condition = eval_expression(*if_expr.condition);

    if is_true(condition) == TRUE {
        eval_statement_vec(if_expr.consequence)
    } else if if_expr.alternative.is_some() {
        eval_statement_vec(if_expr.alternative.unwrap())
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
        _ => return NULL,
    }
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

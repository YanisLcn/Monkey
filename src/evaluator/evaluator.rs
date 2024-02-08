use crate::{
    ast::ast::{
        Expression::{self, *},
        Program,
        Statement::{self, *},
    },
    object::object::Object,
};

const TRUE: Object = Object::BOOLEAN(true);
const FALSE: Object = Object::BOOLEAN(false);
const NULL: Object = Object::NULL;

pub fn eval(node: Program) -> Object {
    match node
        .statements
        .iter()
        .map(|stmt| eval_statement(stmt.clone()))
        .last()
    {
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
        Prefix(_) => todo!(),
        Infix(_) => todo!(),
        IfExpression(_) => todo!(),
        FnExpression(_) => todo!(),
        CallExpression(_) => todo!(),
    }
}

fn native_bool_to_object(b: bool) -> Object {
    if b {
        TRUE
    } else {
        FALSE
    }
}

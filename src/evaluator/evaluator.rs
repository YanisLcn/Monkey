use crate::{
    ast::ast::{
        Expression::{self, *},
        Program,
        Statement::{self, *},
    },
    object::object::Object,
};

pub fn eval(node: Program) -> Object {
    node.statements
        .iter()
        .map(|stmt| eval_statement(stmt.clone()))
        .last()
        .unwrap()
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
        Bool(_) => todo!(),
        Prefix(_) => todo!(),
        Infix(_) => todo!(),
        IfExpression(_) => todo!(),
        FnExpression(_) => todo!(),
        CallExpression(_) => todo!(),
    }
}

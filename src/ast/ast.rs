use std::fmt::Display;

use crate::token::token::Token;

/** PROGRAM */

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
    pub fn push(&mut self, stmt: Statement) {
        self.statements.push(stmt);
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = String::new();
        self.statements
            .iter()
            .for_each(|statement| str.push_str(format!("{statement}").as_str()));
        write!(f, "{}", str)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

/** STATEMENTS */

impl LetStatement {
    pub(crate) fn new(name: Identifier, value: Identifier) -> Self {
        LetStatement {
            name,
            value: Expression::new(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReturnStatement {
    pub value: Expression,
}

impl ReturnStatement {
    pub(crate) fn new(value: Identifier) -> Self {
        ReturnStatement {
            value: Expression::new(value),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(Expression),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::LetStatement(ls) => write!(f, "let {} = {};", ls.name.value, ls.value),
            Statement::ReturnStatement(rs) => write!(f, "return {};", rs.value),
            Statement::ExpressionStatement(expr) => write!(f, "{};", expr),
        }
    }
}

/** EXPRESSIONS */

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Identifier { value }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrefixExpr {
    pub operator: Token,
    pub expr: Box<Expression>,
}

impl PrefixExpr {
    pub fn new(token: Token, expr: Expression) -> Self {
        PrefixExpr {
            operator: token,
            expr: Box::new(expr),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InfixExpr {
    pub operator: Token,
    pub left_expr: Box<Expression>,
    pub right_expr: Box<Expression>,
}

impl InfixExpr {
    pub fn new(operator: Token, left_expr: Expression, right_expr: Expression) -> Self {
        InfixExpr {
            operator,
            left_expr: Box::new(left_expr),
            right_expr: Box::new(right_expr),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Integer(i32),
    Prefix(PrefixExpr),
    Infix(InfixExpr),
}

impl Expression {
    pub fn new(ident: Identifier) -> Expression {
        Expression::Identifier(ident)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident.value),
            Expression::Integer(int) => write!(f, "{}", int),
            Expression::Prefix(prefix) => write!(f, "({}{})", prefix.operator, prefix.expr),
            Expression::Infix(infix) => write!(
                f,
                "({} {} {})",
                infix.left_expr, infix.operator, infix.right_expr
            ),
        }
    }
}

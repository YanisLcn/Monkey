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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct LetStatement {
    pub name: Identifier,
    pub value: Expression,
}

/** STATEMENTS */

impl LetStatement {
    pub(crate) fn new(name: Identifier, value: Expression) -> Self {
        LetStatement { name, value }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ReturnStatement {
    pub value: Expression,
}

impl ReturnStatement {
    pub(crate) fn new(value: Expression) -> Self {
        ReturnStatement { value }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: Vec<Statement>,
    pub alternative: Option<Vec<Statement>>,
}

impl Clone for IfExpression {
    fn clone(&self) -> Self {
        let consequence = self.consequence.iter().map(|stmt| stmt.clone()).collect();
        let alternative = if let Some(alt) = &self.alternative {
            Some(
                alt.iter()
                    .map(|stmt| stmt.clone())
                    .collect::<Vec<Statement>>(),
            )
        } else {
            None
        };
        IfExpression {
            condition: self.condition.clone(),
            consequence,
            alternative,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FnExpression {
    pub parameters: Vec<Identifier>,
    pub body: Vec<Statement>,
}

impl Clone for FnExpression {
    fn clone(&self) -> Self {
        let parameters = self.parameters.iter().map(|par| par.clone()).collect();
        let body = self.body.iter().map(|stmt| stmt.clone()).collect();

        FnExpression { parameters, body }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CallExpression {
    pub function: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl Clone for CallExpression {
    fn clone(&self) -> Self {
        let arguments = self.arguments.iter().map(|arg| arg.clone()).collect();
        CallExpression {
            function: self.function.clone(),
            arguments,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Identifier(Identifier),
    Integer(i32),
    Bool(bool),
    Prefix(PrefixExpr),
    Infix(InfixExpr),
    IfExpression(IfExpression),
    FnExpression(FnExpression),
    CallExpression(CallExpression),
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
            Expression::Bool(bool) => write!(f, "{}", bool),
            Expression::IfExpression(ifexpr) => {
                write!(f, "if {} {{ ", ifexpr.condition)?;
                for stmt in ifexpr.consequence.iter() {
                    write!(f, "{}", stmt)?
                }
                write!(f, " }}")?;

                if let Some(statements) = &ifexpr.alternative {
                    write!(f, " else {{ ")?;
                    for stmt in statements.iter() {
                        write!(f, "{}", stmt)?
                    }
                    write!(f, " }}")?;
                };

                Ok(())
            }
            Expression::FnExpression(func) => {
                let param = func
                    .parameters
                    .iter()
                    .map(|par| format!("{:?}", par))
                    .collect::<Vec<String>>()
                    .join(", ");
                let stmts = func
                    .body
                    .iter()
                    .map(|stmt| format!("{}", stmt))
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "fn ({}) {{ {} }}", param, stmts)
            }
            Expression::CallExpression(call) => {
                let args = call
                    .arguments
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({})", call.function, args)
            }
        }
    }
}

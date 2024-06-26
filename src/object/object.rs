use super::{builtin::BuiltinFunction, env::Environment};
use crate::ast::ast::{Identifier, Statement};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    INTEGER(i32),
    BOOLEAN(bool),
    STRING(String),
    NULL,
    RETURN(Box<Object>),
    ERROR(String),
    FUNCTION(Function),
    BUILTIN(BuiltinFunction),
    ARRAY(Vec<Object>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: Vec<Statement>,
    pub env: Rc<RefCell<Environment>>,
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::INTEGER(_) => "INTEGER".to_string(),
            Object::BOOLEAN(_) => "BOOLEAN".to_string(),
            Object::STRING(_) => "STRING".to_string(),
            Object::NULL => "NULL".to_string(),
            Object::RETURN(obj) => obj.get_type(),
            Object::ERROR(_) => "ERROR".to_string(),
            Object::FUNCTION(_) => "FUNCTION".to_string(),
            Object::BUILTIN(_) => "BUILTIN".to_string(),
            Object::ARRAY(_) => "ARRAY".to_string(),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::INTEGER(x) => write!(f, "{x}"),
            Object::BOOLEAN(b) => write!(f, "{b}"),
            Object::STRING(s) => write!(f, "{s}"),
            Object::NULL => write!(f, "null"),
            Object::RETURN(r) => write!(f, "return {r}"),
            Object::ERROR(s) => write!(f, "{s}"),
            Object::FUNCTION(fun) => write!(
                f,
                "fn ({:?}) {{\n {:?} }}\n",
                fun.parameters
                    .iter()
                    .map(|f| format!("{f}"))
                    .collect::<Vec<String>>()
                    .join(", "),
                fun.body
            ),
            Object::BUILTIN(_) => write!(f, "builtin"),
            Object::ARRAY(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(|f| format!("{f}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

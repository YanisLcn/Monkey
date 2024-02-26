use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    INTEGER(i32),
    BOOLEAN(bool),
    NULL,
    RETURN(Box<Object>),
    ERROR(String),
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::INTEGER(_) => "INTEGER".to_string(),
            Object::BOOLEAN(_) => "BOOLEAN".to_string(),
            Object::NULL => "NULL".to_string(),
            Object::RETURN(obj) => obj.get_type(),
            Object::ERROR(_) => "ERROR".to_string(),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::INTEGER(x) => write!(f, "{x}"),
            Object::BOOLEAN(b) => write!(f, "{b}"),
            Object::NULL => write!(f, "null"),
            Object::RETURN(r) => write!(f, "return {r}"),
            Object::ERROR(s) => write!(f, "{s}"),
        }
    }
}

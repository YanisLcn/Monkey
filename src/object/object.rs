use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    INTEGER(i32),
    BOOLEAN(bool),
    NULL,
    RETURN(Box<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::INTEGER(x) => write!(f, "{x}"),
            Object::BOOLEAN(b) => write!(f, "{b}"),
            Object::NULL => write!(f, "null"),
            Object::RETURN(r) => write!(f, "return {r}"),
        }
    }
}

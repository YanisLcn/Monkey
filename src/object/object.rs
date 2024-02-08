use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    INTEGER(i32),
    BOOLEAN(bool),
    NULL,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::INTEGER(x) => write!(f, "INTEGER({x})"),
            Object::BOOLEAN(b) => write!(f, "BOOLEAN({b})"),
            Object::NULL => write!(f, "null"),
        }
    }
}

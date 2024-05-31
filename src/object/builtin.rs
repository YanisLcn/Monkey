use super::object::Object;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltinFunction {
    LEN,
    FIRST,
    LAST,
}

impl BuiltinFunction {
    pub fn get_builtin(name: &str) -> Option<Object> {
        Self::from(name).ok()
    }

    fn from(name: &str) -> Result<Object, ()> {
        match name {
            "len" => Ok(Object::BUILTIN(Self::LEN)),
            "first" => Ok(Object::BUILTIN(Self::FIRST)),
            "last" => Ok(Object::BUILTIN(Self::LAST)),
            _ => Result::Err(()),
        }
    }

    pub fn call(&self, args: Vec<Object>) -> Object {
        match self {
            BuiltinFunction::LEN => Self::call_len(args),
            BuiltinFunction::FIRST => Self::call_first(args),
            BuiltinFunction::LAST => Self::call_last(args),
        }
    }

    fn call_len(args: Vec<Object>) -> Object {
        Self::handle_expected_number_arguments(1, args.len()).unwrap_or_else(|| match &args[0] {
            Object::STRING(s) => Object::INTEGER(s.len().try_into().unwrap()),
            Object::ARRAY(a) => Object::INTEGER(a.len().try_into().unwrap()),
            _ => Object::ERROR("Argument type not supported by `len`.".to_string()),
        })
    }

    fn call_first(args: Vec<Object>) -> Object {
        Self::handle_expected_number_arguments(1, args.len()).unwrap_or_else(|| match &args[0] {
            Object::ARRAY(a) => a.first().unwrap_or(&Object::NULL).clone(),
            _ => Object::ERROR("Argument type not supported by `first`.".to_string()),
        })
    }

    fn call_last(args: Vec<Object>) -> Object {
        Self::handle_expected_number_arguments(1, args.len()).unwrap_or_else(|| match &args[0] {
            Object::ARRAY(a) => a.last().unwrap_or(&Object::NULL).clone(),
            _ => Object::ERROR("Argument type not supported by `first`.".to_string()),
        })
    }

    fn handle_expected_number_arguments(expected: usize, received: usize) -> Option<Object> {
        match expected == received {
            true => None,
            false => Some(Object::ERROR(format!(
                "Wrong number of arguments. Expected: {expected} | Got: {received}"
            ))),
        }
    }
}

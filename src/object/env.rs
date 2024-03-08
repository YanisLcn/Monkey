use std::collections::HashMap;

use super::object::Object;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: String) -> Option<&Object> {
        self.store.get(&name)
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}

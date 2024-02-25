use std::collections::HashMap;

use super::object::Object;

struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    fn get(&self, name: String) -> Option<&Object> {
        self.store.get(&name)
    }

    fn set(&self, name: String, value: Object) {
        self.store[&name] = value;
    }
}

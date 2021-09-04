use std::collections::HashMap;
use crate::ast::Value;

pub struct Scope<'a> {
    registry: HashMap<&'a str, Value>
}

impl<'a> Scope<'a> {

    pub fn new() -> Self {
        Scope {
            registry: HashMap::new(),
        }
    }

    pub fn store(&mut self, identifier: &'a str, value: Value) {
        self.registry.insert(identifier, value);
    }

    pub fn resolve(&self, identifier: &str) -> Option<&Value> {
        self.registry.get(identifier)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_resolve() {
        let mut scope = Scope::new();
        scope.store("foo", Value::Integer(1));
        assert_eq!(scope.resolve("foo"), Some(&Value::Integer(1)));
        assert_eq!(scope.resolve("bar"), None);
    }
}
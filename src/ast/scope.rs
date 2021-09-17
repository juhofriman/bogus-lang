use crate::ast::{Value, EvaluationError};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Scope<'a> {
    parent: Option<Box<&'a Scope<'a>>>,
    registry: HashMap<String, Rc<dyn Value>>,
}

impl<'a> Scope<'a> {
    pub fn new() -> Scope<'a> {
        Scope {
            parent: None,
            registry: HashMap::new(),
        }
    }
    pub fn sub(scope: &'a Scope) -> Scope<'a> {
        Scope {
            parent: Some(Box::new(scope)),
            registry: HashMap::new(),
        }
    }
    pub fn store(&mut self, name: String, value: Rc<dyn Value>) {
        self.registry.insert(name, value);
    }
    pub fn resolve(&self, name: &String) -> Option<Rc<dyn Value>> {
        match self.registry.get(name) {
            Some(v) => Some(v.clone()),
            None => match &self.parent {
                None => None,
                Some(parent_scope) => parent_scope.resolve(name)
            },
        }
    }
    pub fn resolve_result(&self, name: &String) -> Result<Rc<dyn Value>, EvaluationError> {
        match self.resolve(name) {
            Some(value) => Ok(value),
            None => Err(EvaluationError::cant_resolve(name)),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::v_integer::IntegerValue;

    #[test]
    fn simple_scope_usage() {
        let mut scope = Scope::new();
        scope.store("foo".to_string(), IntegerValue::rc_value(1));
        does_not_resolve(&scope,"bar");
        resolves_to(&scope, "foo", IntegerValue::rc_value(1));
    }

    #[test]
    fn resolve_from_parent() {
        let mut scope = Scope::new();
        scope.store("foo".to_string(), IntegerValue::rc_value(1));

        let sub_scope = Scope::sub(&scope);

        resolves_to(&scope, "foo", IntegerValue::rc_value(1));
        resolves_to(&sub_scope, "foo", IntegerValue::rc_value(1));
    }

    #[test]
    fn resolve_from_sub_only() {
        let scope = Scope::new();

        let mut sub_scope = Scope::sub(&scope);

        sub_scope.store("foo".to_string(), IntegerValue::rc_value(1));

        does_not_resolve(&scope, "foo");
        resolves_to(&sub_scope, "foo", IntegerValue::rc_value(1));
    }

    fn resolves_to(scope: &Scope, key: &str, expected: Rc<dyn Value>) {
        let resolved = scope.resolve(&key.to_string());
        match resolved {
            Some(value) => assert_eq!(value.type_matcher(), expected.type_matcher()),
            None => panic!("Could not resolve var `{}`", key)
        }
    }

    fn does_not_resolve(scope: &Scope, key: &str) {
        let resolved = scope.resolve(&key.to_string());
        match resolved {
            Some(value) =>
                panic!("Expected `{}` not to resolve but {:?} resolved",
                       key,
                       value.type_matcher()),
            None => ()
        }
    }
}
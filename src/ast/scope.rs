use crate::ast::Value;
use std::collections::HashMap;
use std::rc::Rc;

pub struct Scope {
    registry: HashMap<String, Rc<dyn Value>>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            registry: HashMap::new(),
        }
    }
    pub fn store(&mut self, name: String, value: Rc<dyn Value>) {
        self.registry.insert(name, value);
    }
    pub fn resolve(&self, name: &String) -> Option<Rc<dyn Value>> {
        match self.registry.get(name) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }
}
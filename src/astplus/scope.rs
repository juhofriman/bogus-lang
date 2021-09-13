use crate::astplus::Value;
use std::collections::HashMap;

pub struct Scope {
    registry: HashMap<String, Box<dyn Value>>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            registry: HashMap::new(),
        }
    }
    pub fn store(&mut self, name: String, value: Box<dyn Value>) {
        self.registry.insert(name, value);
    }
    pub fn resolve(&self, name: &String) -> Option<Box<dyn Value>> {
        match self.registry.get(name) {
            Some(v) => Some(v.value_clone()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

}
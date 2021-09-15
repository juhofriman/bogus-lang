use crate::ast::{Value, TypeMatcher, Expression, EvaluationError};
use std::rc::Rc;
use crate::ast::scope::Scope;

pub struct Null;

impl Null {
    pub fn rc() -> Rc<Null> { Rc::new(Null) }
}

impl Expression for Null {
    fn evaluate(&self, _scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        Ok(Null::rc())
    }
    fn visualize(&self, level: usize) {
        println!("{} Null", "-".repeat(level));
    }
}

impl Value for Null {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Null
    }
}

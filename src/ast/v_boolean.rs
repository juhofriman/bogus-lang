use crate::ast::{Expression, Scope, Value, EvaluationError, TypeMatcher};
use std::rc::Rc;

pub struct BooleanExpression {
    value: bool,
}

impl BooleanExpression {
    pub fn rc(value: bool) -> Rc<BooleanExpression> {
        Rc::new(BooleanExpression { value })
    }
}

impl Expression for BooleanExpression {
    fn evaluate(&self, _scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        Ok(Rc::new(BooleanValue { value: self.value }))
    }

    fn visualize(&self, level: usize) {
        println!("{} Boolean({})", "-".repeat(level), self.value);
    }
}

pub struct BooleanValue {
    value: bool,
}

impl BooleanValue {
    pub fn rc(value: bool) -> Rc<BooleanValue> {
        Rc::new(BooleanValue { value })
    }
}

impl Value for BooleanValue {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Boolean(&self.value)
    }
    fn is_truthy(&self) -> bool {
        self.value == true
    }
}

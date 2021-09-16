use crate::ast::scope::Scope;
use std::rc::Rc;
use crate::ast::{Expression, Value, EvaluationError, TypeMatcher};


pub struct ReturnStatement {
    expression: Rc<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(expression: Rc<dyn Expression>) -> ReturnStatement {
        ReturnStatement {
            expression,
        }
    }
    pub fn rc(expression: Rc<dyn Expression>) -> Rc<ReturnStatement> {
        Rc::new(ReturnStatement::new(expression))
    }
}

impl Expression for ReturnStatement {
    fn is_return(&self) -> bool {
        true
    }
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        Ok(Rc::new(ReturnValue { value: self.expression.evaluate(scope)? }))
    }
    fn visualize(&self, level: usize) {
        println!("{} ReturnStatement", "-".repeat(level));
    }
}

pub struct ReturnValue {
    value: Rc<dyn Value>
}

impl Value for ReturnValue {
    fn type_matcher(&self) -> TypeMatcher {
        self.value.type_matcher()
    }
    fn is_return_value(&self) -> bool {
        true
    }
}
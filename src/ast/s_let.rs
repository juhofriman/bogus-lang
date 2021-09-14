use crate::ast::scope::Scope;
use std::rc::Rc;
use crate::ast::{Expression, Value, EvaluationError, TypeMatcher};
use crate::ast::v_void::Void;

pub struct LetStatement {
    identifier: String,
    expression: Rc<dyn Expression>,
}

impl LetStatement {
    pub fn new(identifier: String, expression: Rc<dyn Expression>) -> LetStatement {
        LetStatement {
            identifier,
            expression,
        }
    }
}

impl Expression for LetStatement {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        let value = self.expression.evaluate(scope)?;
        scope.store(self.identifier.clone(), value);
        Ok(Rc::new(Void))
    }
    fn visualize(&self, level: usize) {
        println!("{} LetStatement", "-".repeat(level));
    }
}

struct Function {
    expression: Rc<dyn Expression>,
}

impl Value for Function {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Function
    }
    fn call(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        self.expression.evaluate(scope)
    }
}
use crate::ast::scope::Scope;
use std::rc::Rc;
use crate::ast::{Expression, Value, EvaluationError};
use crate::ast::v_void::Void;

pub struct AssignStatement {
    identifier: String,
    expression: Rc<dyn Expression>,
}

impl AssignStatement {
    pub fn new(identifier: String, expression: Rc<dyn Expression>) -> AssignStatement {
        AssignStatement {
            identifier,
            expression,
        }
    }
    pub fn rc(identifier: String, expression: Rc<dyn Expression>) -> Rc<AssignStatement> {
        Rc::new(AssignStatement::new(identifier, expression))
    }
}

impl Expression for AssignStatement {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        match scope.resolve(&self.identifier) {
            Some(_) => {
                let value = self.expression.evaluate(scope)?;
                scope.store(self.identifier.clone(), value);
                Ok(Rc::new(Void))
            },
            None => Err( EvaluationError::cant_assing(&self.identifier.as_str()))
        }
    }
    fn visualize(&self, level: usize) {
        println!("{} AssignStatement ({})", "-".repeat(level), self.identifier);
        self.expression.visualize(level + 1);
    }
}
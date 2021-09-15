use crate::ast::scope::Scope;
use std::rc::Rc;
use crate::ast::{Expression, Value, EvaluationError};
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
    pub fn rc(identifier: String, expression: Rc<dyn Expression>) -> Rc<LetStatement> {
        Rc::new(LetStatement::new(identifier, expression))
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
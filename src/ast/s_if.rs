use crate::ast::scope::Scope;
use std::rc::Rc;
use crate::ast::{Expression, Value, EvaluationError};
use crate::ast::v_void::Void;

pub struct IfStatement {
    condition: Rc<dyn Expression>,
    branch: Rc<dyn Expression>,
}

impl IfStatement {
    pub fn new(condition: Rc<dyn Expression>, branch: Rc<dyn Expression>) -> IfStatement {
        IfStatement {
            condition,
            branch
        }
    }
    pub fn rc(condition: Rc<dyn Expression>, branch: Rc<dyn Expression>) -> Rc<IfStatement> {
        Rc::new(IfStatement::new(condition, branch))
    }
}

impl Expression for IfStatement {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        if self.condition.evaluate(scope)?.is_truthy() {
            return self.branch.evaluate(scope);
        }
        Ok(Rc::new(Void))
    }
    fn visualize(&self, level: usize) {
        println!("{} IfStatement", "-".repeat(level));
    }
}
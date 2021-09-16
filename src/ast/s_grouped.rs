use crate::ast::scope::Scope;
use std::rc::Rc;
use crate::ast::{Expression, Value, EvaluationError};
use crate::ast::v_void::Void;

pub struct GroupedStatement {
    expressions: Vec<Rc<dyn Expression>>,
}

impl GroupedStatement {
    pub fn new(expressions: Vec<Rc<dyn Expression>>) -> GroupedStatement {
        GroupedStatement {
            expressions,
        }
    }
    pub fn rc(expressions: Vec<Rc<dyn Expression>>) -> Rc<GroupedStatement> {
        Rc::new(GroupedStatement::new(expressions))
    }
}

impl Expression for GroupedStatement {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        for expr in &self.expressions {
            if expr.is_return() {
                return expr.evaluate(scope)
            }
            let value = expr.evaluate(scope)?;
            if value.is_return_value() {
                return Ok(value)
            }
        }
        Ok(Rc::new(Void))
    }
    fn visualize(&self, level: usize) {
        println!("{} GroupedStatement", "-".repeat(level));
        for e in &self.expressions {
            e.visualize(level + 1)
        }
    }
}
use crate::ast::{Expression, Value, EvaluationError};
use crate::ast::scope::Scope;
use std::rc::Rc;

pub struct EqualsExpression {
    left: Rc<dyn Expression>,
    right: Rc<dyn Expression>,
}

impl EqualsExpression {
    pub fn new(left: Rc<dyn Expression>, right: Rc<dyn Expression>) -> EqualsExpression {
        EqualsExpression {
            left,
            right,
        }
    }
    pub fn rc(left: Rc<dyn Expression>, right: Rc<dyn Expression>) -> Rc<EqualsExpression> {
        Rc::new(EqualsExpression::new(left, right))
    }
}

impl Expression for EqualsExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        let l_value = self.left.evaluate(scope)?;
        let r_value = self.right.evaluate(scope)?;

        l_value.apply_equals(r_value)
    }

    fn visualize(&self, level: usize) {
        println!("{} EqualsExpression", "-".repeat(level));
        println!("{} Left", "-".repeat(level + 1));
        self.left.visualize(level + 2);
        println!("{} Right", "-".repeat(level + 1));
        self.right.visualize(level + 2);
    }
}

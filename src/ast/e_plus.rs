use crate::ast::{Expression, Value, EvaluationError};
use crate::ast::scope::Scope;
use std::rc::Rc;

pub struct PlusExpression {
    left: Rc<dyn Expression>,
    right: Rc<dyn Expression>,
}

impl PlusExpression {
    pub fn new(left: Rc<dyn Expression>, right: Rc<dyn Expression>) -> PlusExpression {
        PlusExpression {
            left,
            right,
        }
    }
    pub fn rc(left: Rc<dyn Expression>, right: Rc<dyn Expression>) -> Rc<PlusExpression> {
        Rc::new(PlusExpression::new(left, right))
    }
}

impl Expression for PlusExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        let l_value = self.left.evaluate(scope)?;
        let r_value = self.right.evaluate(scope)?;

        l_value.apply_plus(r_value)
    }

    fn visualize(&self, level: usize) {
        println!("{} PlusExpression", "-".repeat(level));
        println!("{} Left", "-".repeat(level + 1));
        self.left.visualize(level + 2);
        println!("{} Right", "-".repeat(level + 1));
        self.right.visualize(level + 2);
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::evaluates_to;
    use crate::ast::v_integer::{IntegerExpression, IntegerValue};

    #[test]
    fn test_plus_expression() {
        let expr = PlusExpression::new(
            IntegerExpression::rc(1),
            IntegerExpression::rc(1),
        );
        evaluates_to(
            expr.evaluate(&mut Scope::new()),
            IntegerValue::rc_value(2)
        );

        let expr = PlusExpression::new(
            Rc::new(PlusExpression::new(
                IntegerExpression::rc(5),
                IntegerExpression::rc(5),
            )),
            Rc::new(PlusExpression::new(
                IntegerExpression::rc(10),
                IntegerExpression::rc(9),
            ))
        );
        evaluates_to(
            expr.evaluate(&mut Scope::new()),
            IntegerValue::rc_value(29)
        );
    }

}
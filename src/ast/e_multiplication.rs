use crate::ast::{Expression, Value, EvaluationError};
use crate::ast::scope::Scope;
use std::rc::Rc;

pub struct MultiplicationExpression {
    left: Rc<dyn Expression>,
    right: Rc<dyn Expression>,
}

impl MultiplicationExpression {
    pub fn new(left: Rc<dyn Expression>, right: Rc<dyn Expression>) -> MultiplicationExpression {
        MultiplicationExpression {
            left,
            right,
        }
    }
}

impl Expression for MultiplicationExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        let l_value = self.left.evaluate(scope)?;
        let r_value = self.right.evaluate(scope)?;

        l_value.apply_multiplication(r_value)
    }

    fn visualize(&self, level: usize) {
        println!("{} MultiplicationExpression", "-".repeat(level));
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
    fn test_multiplication_expression() {
        let expr = MultiplicationExpression::new(
            IntegerExpression::rc(1),
            IntegerExpression::rc(1),
        );
        evaluates_to(
            expr.evaluate(&mut Scope::new()),
            IntegerValue::rc_value(1)
        );

        let expr = MultiplicationExpression::new(
            Rc::new(MultiplicationExpression::new(
                IntegerExpression::rc(2),
                IntegerExpression::rc(5),
            )),
            Rc::new(MultiplicationExpression::new(
                IntegerExpression::rc(2),
                IntegerExpression::rc(5),
            ))
        );
        evaluates_to(
            expr.evaluate(&mut Scope::new()),
            IntegerValue::rc_value(100)
        );
    }

}
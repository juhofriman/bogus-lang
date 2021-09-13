use crate::astplus::{Expression, Value, EvaluationError};
use crate::astplus::scope::Scope;
use std::rc::Rc;

pub struct PlusExpression {
    left: Rc<dyn Expression>,
    right: Rc<dyn Expression>,
}

impl PlusExpression {
    fn new(left: Rc<dyn Expression>, right: Rc<dyn Expression>) -> PlusExpression {
        PlusExpression {
            left,
            right,
        }
    }
}

impl Expression for PlusExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        let l_value = self.left.evaluate(scope)?;
        let r_value = self.right.evaluate(scope)?;

        l_value.apply_plus(r_value)
    }
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::astplus::tests::evaluates_to;
    use crate::astplus::v_integer::{IntegerExpression, IntegerValue};

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
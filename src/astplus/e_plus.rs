use crate::astplus::{Expression, Value, EvaluationError};
use crate::astplus::scope::Scope;

pub struct PlusExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl PlusExpression {
    fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> PlusExpression {
        PlusExpression {
            left,
            right,
        }
    }
}

impl Expression for PlusExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<dyn Value>, EvaluationError> {
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
            IntegerExpression::boxed(1),
            IntegerExpression::boxed(1),
        );
        evaluates_to(
            expr.evaluate(&mut Scope::new()),
            IntegerValue::boxed_value(2)
        );

        let expr = PlusExpression::new(
            Box::new(PlusExpression::new(
                IntegerExpression::boxed(5),
                IntegerExpression::boxed(5),
            )),
            Box::new(PlusExpression::new(
                IntegerExpression::boxed(10),
                IntegerExpression::boxed(9),
            ))
        );
        evaluates_to(
            expr.evaluate(&mut Scope::new()),
            IntegerValue::boxed_value(29)
        );
    }

}
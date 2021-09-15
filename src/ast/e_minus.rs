use crate::ast::{Expression, Value, EvaluationError};
use crate::ast::scope::Scope;
use std::rc::Rc;

pub struct PrefixMinusExpression {
    expression: Rc<dyn Expression>
}

impl PrefixMinusExpression {
    pub fn new(expression: Rc<dyn Expression>) -> PrefixMinusExpression {
        PrefixMinusExpression {
            expression,
        }
    }
}

impl Expression for PrefixMinusExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        self.expression.evaluate(scope)?.apply_prefix_minus()
    }

    fn visualize(&self, level: usize) {
        println!("{} PrefixMinusExpression", "-".repeat(level));
        self.expression.visualize(level + 1);
    }
}

pub struct MinusExpression {
    left: Rc<dyn Expression>,
    right: Rc<dyn Expression>,
}

impl MinusExpression {
    pub fn new(left: Rc<dyn Expression>, right: Rc<dyn Expression>) -> MinusExpression {
        MinusExpression {
            left,
            right,
        }
    }
    pub fn rc(left: Rc<dyn Expression>, right: Rc<dyn Expression>) -> Rc<MinusExpression> {
        Rc::new(MinusExpression::new(left, right))
    }
}

impl Expression for MinusExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        let l_value = self.left.evaluate(scope)?;
        let r_value = self.right.evaluate(scope)?;

        l_value.apply_minus(r_value)
    }

    fn visualize(&self, level: usize) {
        println!("{} MinusExpression", "-".repeat(level));
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
        let expr = MinusExpression::new(
            IntegerExpression::rc(1),
            IntegerExpression::rc(1),
        );
        evaluates_to(
            expr.evaluate(&mut Scope::new()),
            IntegerValue::rc_value(0)
        );

        let expr = MinusExpression::new(
            Rc::new(MinusExpression::new(
                IntegerExpression::rc(5),
                IntegerExpression::rc(5),
            )),
            Rc::new(MinusExpression::new(
                IntegerExpression::rc(10),
                IntegerExpression::rc(9),
            ))
        );
        evaluates_to(
            expr.evaluate(&mut Scope::new()),
            IntegerValue::rc_value(-1)
        );
    }

}
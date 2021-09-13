use crate::astplus::{Expression, Scope, Value, EvaluationError, TypeMatcher};
use std::rc::Rc;

pub struct IntegerExpression {
    value: i32,
}

impl IntegerExpression {
    pub fn new(value: i32) -> IntegerExpression {
        IntegerExpression {
            value,
        }
    }
    pub fn rc(value: i32) -> Rc<IntegerExpression> {
        Rc::new(IntegerExpression {
            value,
        })
    }
}

impl Expression for IntegerExpression {
    fn evaluate(&self, _: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        Ok(IntegerValue::rc_from(self))
    }
}

pub struct IntegerValue {
    value: i32,
}

impl IntegerValue {
    pub fn rc_from(expr: &IntegerExpression) -> Rc<dyn Value> {
        Rc::new(IntegerValue { value: expr.value })
    }
    pub fn rc_value(value: i32) -> Rc<dyn Value> {
        Rc::new(IntegerValue { value })
    }
}

impl Value for IntegerValue {

    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Integer(&self.value)
    }

    fn apply_plus(&self, other: Rc<dyn Value>) -> Result<Rc<dyn Value>, EvaluationError> {
        match other.type_matcher() {
            TypeMatcher::Integer(other_value) =>
                Ok(IntegerValue::rc_value(self.value + other_value)),

            _ => Err(EvaluationError::operator_not_applicable(
                "+",
                self.type_matcher(),
                other.type_matcher()))
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::astplus::tests::evaluates_to;

    #[test]
    fn test_integer_equals() {
        assert_eq!(
            IntegerValue::rc_value(1).type_matcher(),
            IntegerValue::rc_value(1).type_matcher(),
        );
        assert_ne!(
            IntegerValue::rc_value(1).type_matcher(),
            IntegerValue::rc_value(2).type_matcher(),
        );
    }

    #[test]
    fn test_integer_evaluate() {
        let integer_expr = IntegerExpression::new(1);
        evaluates_to(integer_expr.evaluate(&mut Scope::new()),
                     IntegerValue::rc_value(1));
    }
}
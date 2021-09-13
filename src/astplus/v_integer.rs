use crate::astplus::{Expression, Scope, Value, EvaluationError, TypeMatcher};

pub struct IntegerExpression {
    value: i32,
}

impl IntegerExpression {
    pub fn new(value: i32) -> IntegerExpression {
        IntegerExpression {
            value,
        }
    }
    pub fn boxed(value: i32) -> Box<IntegerExpression> {
        Box::new(IntegerExpression {
            value,
        })
    }
}

impl Expression for IntegerExpression {
    fn evaluate(&self, _: &mut Scope) -> Result<Box<dyn Value>, EvaluationError> {
        Ok(IntegerValue::boxed_from(self))
    }
}

pub struct IntegerValue {
    value: i32,
}

impl IntegerValue {
    pub fn boxed_from(expr: &IntegerExpression) -> Box<dyn Value> {
        Box::new(IntegerValue { value: expr.value })
    }
    pub fn boxed_value(value: i32) -> Box<dyn Value> {
        Box::new(IntegerValue { value })
    }
}

impl Value for IntegerValue {
    fn value_clone(&self) -> Box<dyn Value> {
        IntegerValue::boxed_value(self.value)
    }

    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Integer(&self.value)
    }

    fn apply_plus(&self, other: Box<dyn Value>) -> Result<Box<dyn Value>, EvaluationError> {
        match other.type_matcher() {
            TypeMatcher::Integer(other_value) =>
                Ok(IntegerValue::boxed_value(self.value + other_value)),

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
            IntegerValue::boxed_value(1).type_matcher(),
            IntegerValue::boxed_value(1).type_matcher(),
        );
        assert_ne!(
            IntegerValue::boxed_value(1).type_matcher(),
            IntegerValue::boxed_value(2).type_matcher(),
        );
    }

    #[test]
    fn test_integer_evaluate() {
        let integer_expr = IntegerExpression::new(1);
        evaluates_to(integer_expr.evaluate(&mut Scope::new()),
                     IntegerValue::boxed_value(1));
    }
}
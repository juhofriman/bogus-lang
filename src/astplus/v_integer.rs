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
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Integer(&self.value)
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
use crate::ast::{OperatorApplyMatcher, Matcher, Value, EvalError};

impl OperatorApplyMatcher for Matcher<'_, i32> {
    fn name(&self) -> &'static str {
        "Integer"
    }

    fn apply_plus_with_integer(&self, other: &i32) -> Result<Value, EvalError> {
        Ok(Value::Integer(self.value + other))
    }
    fn apply_plus_with_string(&self, other: &String) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.to_string().as_str());
        new.push_str(other.as_str());
        Ok(Value::String(new))
    }
    fn apply_minus_with_integer(&self, other: &i32) -> Result<Value, EvalError> {
        Ok(Value::Integer(self.value - other))
    }
    fn apply_multiplication_with_integer(&self, other: &i32) -> Result<Value, EvalError> {
        Ok(Value::Integer(self.value * other))
    }
    fn apply_prefix_minus(&self) -> Result<Value, EvalError> {
        Ok(Value::Integer(-(*self.value)))
    }
}
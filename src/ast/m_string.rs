use crate::ast::{OperatorApplyMatcher, Matcher, Value, EvalError};

impl OperatorApplyMatcher for Matcher<'_, String> {
    fn name(&self) -> &'static str {
        "String"
    }

    fn apply_plus_with_integer(&self, other: &i32) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.as_str());
        new.push_str(other.to_string().as_str());
        Ok(Value::String(new))
    }
    fn apply_plus_with_string(&self, other: &String) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.as_str());
        new.push_str(other.as_str());
        Ok(Value::String(new))
    }
    fn apply_plus_with_null(&self) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.as_str());
        new.push_str("null");
        Ok(Value::String(new))
    }
}
use crate::ast::{OperatorApplyMatcher, Value, EvalError};

pub struct NullMatcher {}

impl OperatorApplyMatcher for NullMatcher {
    fn name(&self) -> &'static str {
        "Null"
    }

    fn apply_plus_with_string(&self, other: &String) -> Result<Value, EvalError> {
        let mut new = String::from("null");
        new.push_str(other.as_str());
        Ok(Value::String(new))
    }
}

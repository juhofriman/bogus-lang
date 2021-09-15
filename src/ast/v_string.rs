use crate::ast::{Expression, Scope, Value, EvaluationError, TypeMatcher};
use std::rc::Rc;

pub struct StringExpression {
    value: String,
}

impl StringExpression {
    pub fn new(value: String) -> StringExpression {
        StringExpression {
            value,
        }
    }
    pub fn rc(value: String) -> Rc<StringExpression> {
        Rc::new(StringExpression::new(value))
    }
}

impl Expression for StringExpression {
    fn evaluate(&self, _: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        Ok(StringValue::rc_from(self))
    }
    fn visualize(&self, level: usize) {
        println!("{} String({})", "-".repeat(level), self.value);
    }
}

pub struct StringValue {
    value: String,
}

impl StringValue {
    pub fn rc_value(value: String) -> Rc<dyn Value> {
        Rc::new(StringValue { value })
    }
    pub fn rc_from(value: &StringExpression) -> Rc<dyn Value> {
        StringValue::rc_value(value.value.clone())
    }
}

impl Value for StringValue {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::String(&self.value)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::evaluates_to;

    #[test]
    fn test_string_equals() {
        assert_eq!(
            StringValue::rc_value("foo".to_string()).type_matcher(),
            StringValue::rc_value("foo".to_string()).type_matcher(),
        );
        assert_ne!(
            StringValue::rc_value("foo".to_string()).type_matcher(),
            StringValue::rc_value("bar".to_string()).type_matcher(),
        );
    }

    #[test]
    fn test_string_evaluate() {
        let integer_expr = StringExpression::new("foo".to_string());
        evaluates_to(integer_expr.evaluate(&mut Scope::new()),
                     StringValue::rc_value("foo".to_string()));
    }
}
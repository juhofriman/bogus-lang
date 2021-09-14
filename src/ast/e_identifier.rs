use crate::ast::{Expression, Scope, Value, EvaluationError};
use std::rc::Rc;

pub struct IdentifierExpression {
    value: String,
}

impl IdentifierExpression {
    pub fn new(value: String) -> IdentifierExpression {
        IdentifierExpression {
            value,
        }
    }
    pub fn rc(value: String) -> Rc<IdentifierExpression> {
        Rc::new(IdentifierExpression {
            value,
        })
    }
    pub fn name(&self) -> String {
        self.value.clone()
    }
}

impl Expression for IdentifierExpression {
    fn get_identifier(&self) -> Result<&String, EvaluationError> {
        Ok(&self.value)
    }
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        match scope.resolve(&self.value) {
            Some(value) => Ok(value),
            None => Err(EvaluationError::cant_resolve(&self.value))
        }
    }
    fn visualize(&self, level: usize) {
        println!("{} Identifier({})", "-".repeat(level), self.value);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::{evaluates_to, errors_to};
    use crate::ast::v_integer::IntegerValue;

    #[test]
    fn test_resolve_not_found() {
        let mut scope = Scope::new();
        let expr = IdentifierExpression::new("foo".to_string());

        errors_to(
            expr.evaluate(&mut scope),
            "Can't resolve variable `foo`"
        )
    }

    #[test]
    fn test_resolve_found() {
        let mut scope = Scope::new();
        scope.store("foo".to_string(), IntegerValue::rc_value(1));
        let expr = IdentifierExpression::new("foo".to_string());

        evaluates_to(
            expr.evaluate(&mut scope),
            IntegerValue::rc_value(1)
        )
    }
}
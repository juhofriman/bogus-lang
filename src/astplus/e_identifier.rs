use crate::astplus::{Expression, Scope, Value, EvaluationError, TypeMatcher};

pub struct IdentifierExpression {
    value: String,
}

impl IdentifierExpression {
    pub fn new(value: String) -> IdentifierExpression {
        IdentifierExpression {
            value,
        }
    }
    pub fn boxed(value: String) -> Box<IdentifierExpression> {
        Box::new(IdentifierExpression {
            value,
        })
    }
}

impl Expression for IdentifierExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<dyn Value>, EvaluationError> {
        match scope.resolve(&self.value) {
            Some(value) => Ok(value),
            None => Err(EvaluationError::cant_resolve(&self.value))
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::astplus::tests::{evaluates_to, errors_to};
    use crate::astplus::v_integer::IntegerValue;

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
        scope.store("foo".to_string(), IntegerValue::boxed_value(1));
        let expr = IdentifierExpression::new("foo".to_string());

        evaluates_to(
            expr.evaluate(&mut scope),
            IntegerValue::boxed_value(1)
        )
    }
}
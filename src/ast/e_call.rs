use crate::ast::{Expression, Scope, Value, EvaluationError};
use std::rc::Rc;

pub struct CallExpression {
    identifier: String,
}

impl CallExpression {
    pub fn new(identifier: String) -> CallExpression {
        CallExpression {
            identifier,
        }
    }
    pub fn rc(identifier: String) -> Rc<CallExpression> {
        Rc::new(CallExpression::new(identifier))
    }
}

impl Expression for CallExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        match scope.resolve(&self.identifier) {
            Some(value) => value.call(scope),
            None => Err(EvaluationError::cant_resolve(&self.identifier))
        }
    }

    fn visualize(&self, level: usize) {
        println!("{} CallExpression", "-".repeat(level));
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::{evaluates_to, errors_to, evaluates_to_void};
    use crate::ast::v_integer::{IntegerValue, IntegerExpression};
    use crate::ast::s_fun::FunStatement;

    #[test]
    fn test_call() {
        let mut scope = Scope::new();
        evaluates_to_void(
            FunStatement::new(
                "foo".to_string(),
                IntegerExpression::rc(123))
                .evaluate(&mut scope)
        );
        errors_to(
            CallExpression::new("bar".to_string()).evaluate(&mut scope),
            "Can't resolve variable `bar`",
        )
    }

    #[test]
    fn test_resolve_found() {
        let mut scope = Scope::new();

        evaluates_to_void(
            FunStatement::new(
                "foo".to_string(),
                IntegerExpression::rc(123))
                .evaluate(&mut scope)
        );

        evaluates_to(
            CallExpression::new("foo".to_string()).evaluate(&mut scope),
            IntegerValue::rc_value(123),
        );
        evaluates_to(
            CallExpression::new("foo".to_string()).evaluate(&mut scope),
            IntegerValue::rc_value(123),
        );
    }
}
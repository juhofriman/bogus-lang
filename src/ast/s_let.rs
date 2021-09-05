use crate::ast::{Expression, Value, EvalError};
use crate::ast::scope::Scope;

pub struct LetStatement {
    pub identifier: String,
    pub expression: Box<dyn Expression>
}

impl Expression for LetStatement {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<Value>, EvalError> {
        let value = self.expression.evaluate(scope)?;
        scope.store(self.identifier.as_str(), *value);
        Ok(Box::new(Value::Void))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::evals_to;

    #[test]
    fn test_let() {
        // Ingenious :D
        let statement = LetStatement {
            identifier: "foo".to_string(),
            expression: Box::new(Value::Integer(1)),
        };
        let mut scope = Scope::new();
        evals_to(statement.evaluate(&mut scope), Value::Void);
        assert_eq!(scope.resolve("foo"), Some(&Value::Integer(1)));
    }
}
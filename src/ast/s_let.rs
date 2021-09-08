use crate::ast::{Expression, Value, EvalError};
use crate::ast::scope::Scope;

pub struct LetStatement {
    pub identifier: String,
    pub expression: Box<dyn Expression>,
    _private: ()
}

impl LetStatement {
    pub fn new(name: Value, expression: Box<dyn Expression>) -> LetStatement {
        if let Value::Identifier(identifier) = name {
            return LetStatement {
                identifier,
                expression,
                _private: (),
            }
        }
        panic!("Can't build LetStatement from name: {:?}", name);
    }
}

impl Expression for LetStatement {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<Value>, EvalError> {
        let value = self.expression.evaluate(scope)?;
        scope.store(self.identifier.as_str(), *value);
        Ok(Box::new(Value::Void))
    }
    fn visualize(&self, level: usize) {
        println!("{} LetStatement `{}`", "-".repeat(level), self.identifier);
        self.expression.visualize(level + 1);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::evals_to;

    #[test]
    fn test_let_statement() {
        // Ingenious :D
        let statement = LetStatement::new(
            Value::Identifier("foo".to_string()),
            Box::new(Value::Integer(1)));
        let mut scope = Scope::new();
        evals_to(statement.evaluate(&mut scope), Value::Void);
        assert_eq!(scope.resolve("foo"), Some(&Value::Integer(1)));
    }
}
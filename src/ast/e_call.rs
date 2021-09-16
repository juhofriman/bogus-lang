use crate::ast::{Expression, Scope, Value, EvaluationError};
use std::rc::Rc;

pub struct CallExpression {
    target: Rc<dyn Expression>,
    args: Vec<Rc<dyn Expression>>,
}

impl CallExpression {
    pub fn new(identifier: Rc<dyn Expression>, args: Vec<Rc<dyn Expression>>) -> CallExpression {
        CallExpression {
            target: identifier,
            args,
        }
    }
    pub fn rc(identifier: Rc<dyn Expression>, args: Vec<Rc<dyn Expression>>) -> Rc<CallExpression> {
        Rc::new(CallExpression::new(identifier, args))
    }
}

impl Expression for CallExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        let target_expr = self.target.evaluate(scope)?;
        let mut evaled: Vec<Rc<dyn Value>> = vec![];
        for i in &self.args {
            evaled.push(i.evaluate(scope)?)
        }
        target_expr.call(scope, evaled)
    }

    fn visualize(&self, level: usize) {
        println!("{} CallExpression", "-".repeat(level));
        for a in &self.args {
            a.visualize(level+1)
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::{evaluates_to, errors_to, evaluates_to_void};
    use crate::ast::v_integer::{IntegerValue, IntegerExpression};
    use crate::ast::s_fun::FunStatement;
    use crate::ast::e_identifier::IdentifierExpression;

    #[test]
    fn test_non_resolved_call() {
        let mut scope = Scope::new();
        errors_to(
            CallExpression::new(
                IdentifierExpression::rc("bar".to_string()),
                vec![]).evaluate(&mut scope),
            "Can't resolve variable `bar`",
        )
    }

    #[test]
    fn test_resolved_call() {
        let mut scope = Scope::new();

        evaluates_to_void(
            FunStatement::new(
                "foo".to_string(),
                vec![],
                IntegerExpression::rc(123))
                .evaluate(&mut scope)
        );

        evaluates_to(
            CallExpression::new(
                IdentifierExpression::rc("foo".to_string()),
                vec![]).evaluate(&mut scope),
            IntegerValue::rc_value(123),
        );
        evaluates_to(
            CallExpression::new(
                IdentifierExpression::rc("foo".to_string()),
                vec![]).evaluate(&mut scope),
            IntegerValue::rc_value(123),
        );
    }

    #[test]
    fn test_argument_arity() {
        let mut scope = Scope::new();

        evaluates_to_void(
            FunStatement::new(
                "foo".to_string(),
                vec![IdentifierExpression::new("a".to_string())],
                IntegerExpression::rc(123))
                .evaluate(&mut scope)
        );

        errors_to(
            CallExpression::new(
                IdentifierExpression::rc("foo".to_string()),
                vec![]).evaluate(&mut scope),
            "Expecting 1 arguments for call but 0 given",
        );

        evaluates_to(
            CallExpression::new(
                IdentifierExpression::rc("foo".to_string()),
                vec![
                    IntegerExpression::rc(1)
                ]).evaluate(&mut scope),
            IntegerValue::rc_value(123),
        );

        errors_to(
            CallExpression::new(
                IdentifierExpression::rc("foo".to_string()),
                vec![
                    IntegerExpression::rc(1),
                    IntegerExpression::rc(1)
                ]).evaluate(&mut scope),
            "Expecting 1 arguments for call but 2 given",
        );

    }
}
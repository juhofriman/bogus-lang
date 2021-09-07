use crate::ast::{Expression, Value, EvalError, Operable};
use crate::ast::scope::Scope;

pub struct MultiplicationExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    _private: (),
}

impl MultiplicationExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> MultiplicationExpression {
        MultiplicationExpression {
            left,
            right,
            _private: (),
        }
    }
}

impl Expression for MultiplicationExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<Value>, EvalError> {
        let left_res = self.left.evaluate(scope)?;
        let right_res = self.right.evaluate(scope)?;
        let result = left_res.apply_multiplication(&right_res)?;
        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::{run_expression_tests, ExpressionTest, Expected, evals_to};

    fn create_multiplication_expression(left: Value, right: Value) -> Box<MultiplicationExpression> {
        Box::new(MultiplicationExpression::new(
            Box::new(left),
            Box::new(right),
        ))
    }

    #[test]
    fn test_plus_expression() {
        let cases = vec![

            // int to int
            ExpressionTest {
                expression: create_multiplication_expression(
                    Value::Integer(1),
                    Value::Integer(1),
                ),
                expected: Expected::EvaluatesTo(Value::Integer(1)),
            },
            ExpressionTest {
                expression: create_multiplication_expression(
                    Value::Integer(-1),
                    Value::Integer(1),
                ),
                expected: Expected::EvaluatesTo(Value::Integer(-1)),
            },

            // int to string
            ExpressionTest {
                expression: create_multiplication_expression(
                    Value::String("foo".to_string()),
                    Value::Integer(1),
                ),
                expected: Expected::ErrorsTo("Can't apply String * Integer"),
            },
            ExpressionTest {
                expression: create_multiplication_expression(
                    Value::Integer(1),
                    Value::String("foo".to_string()),
                ),
                expected: Expected::ErrorsTo("Can't apply Integer * String"),
            },

            // String to String
            ExpressionTest {
                expression: create_multiplication_expression(
                    Value::String("foo".to_string()),
                    Value::String("bar".to_string()),
                ),
                expected: Expected::ErrorsTo("Can't apply String * String"),
            },

            // Null to int
            ExpressionTest {
                expression: create_multiplication_expression(
                    Value::Integer(1),
                    Value::Null,
                ),
                expected: Expected::ErrorsTo("Can't apply Integer * Null"),
            },
            ExpressionTest {
                expression: create_multiplication_expression(
                    Value::Null,
                    Value::Integer(1),
                ),
                expected: Expected::ErrorsTo("Can't apply Null * Integer"),
            },
        ];

        run_expression_tests(cases, None);
    }

    #[test]
    fn test_nested_expressions() {
        let mut scope = Scope::new();
        let expr = MultiplicationExpression::new(
            Box::new(MultiplicationExpression::new(
                Box::new(Value::Integer(5)),
                Box::new(Value::Integer(5)),
            )),
            Box::new(Value::Integer(1)),
        );
        evals_to(expr.evaluate(&mut scope), Value::Integer(25));
    }

    #[test]
    fn test_with_identifiers() {
        let mut scope = Scope::new();
        scope.store("a", Value::Integer(2));
        scope.store("b", Value::Integer(3));
        scope.store("c", Value::Integer(1));
        let expr = MultiplicationExpression::new(
            Box::new(MultiplicationExpression::new(
                Box::new(Value::Identifier("a".to_string())),
                Box::new(Value::Identifier("b".to_string())),
            )),
            Box::new(Value::Identifier("c".to_string())),
        );
        evals_to(expr.evaluate(&mut scope), Value::Integer(6));
    }
}

use crate::ast::{Expression, Value, EvalError, Operable};
use crate::ast::scope::Scope;

pub struct MinusExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
}

impl Expression for MinusExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<Value>, EvalError> {
        let left_res = self.left.evaluate(scope)?;
        let right_res = self.right.evaluate(scope)?;
        let result = left_res.apply_minus(&right_res)?;
        Ok(Box::new(result))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::{run_expression_tests, ExpressionTest, Expected, evals_to};

    fn create_minus_expression(left: Value, right: Value) -> Box<MinusExpression> {
        Box::new(MinusExpression {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[test]
    fn test_minus_expression() {
        let cases = vec![

            // int to int
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(1),
                    Value::Integer(1)
                ),
                expected: Expected::EvaluatesTo(Value::Integer(0)),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(-1),
                    Value::Integer(1)
                ),
                expected: Expected::EvaluatesTo(Value::Integer(-2)),
            },

            // int to string
            ExpressionTest {
                expression: create_minus_expression(
                    Value::String("foo".to_string()),
                    Value::Integer(1)
                ),
                expected: Expected::ErrorsTo("Can't apply String - Integer"),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::String("foo".to_string()),
                    Value::Integer(-1)
                ),
                expected: Expected::ErrorsTo("Can't apply String - Integer"),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(1),
                    Value::String("foo".to_string())
                ),
                expected: Expected::ErrorsTo("Can't apply Integer - String"),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(-1),
                    Value::String("foo".to_string())
                ),
                expected: Expected::ErrorsTo("Can't apply Integer - String"),
            },

            // String to String
            ExpressionTest {
                expression: create_minus_expression(
                    Value::String("foo".to_string()),
                    Value::String("bar".to_string())
                ),
                expected: Expected::ErrorsTo("Can't apply String - String"),
            },

            // Null to int
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(1),
                    Value::Null
                ),
                expected: Expected::ErrorsTo("Can't apply Integer - Null"),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Null,
                    Value::Integer(1),
                ),
                expected: Expected::ErrorsTo("Can't apply Null - Integer"),
            },
        ];

        run_expression_tests(cases, None);
    }

    #[test]
    fn test_nested_expressions() {
        let mut scope = Scope::new();
        let expr = MinusExpression {
            left: Box::new(MinusExpression {
                left: Box::new(Value::Integer(5)),
                right: Box::new(Value::Integer(5)),
            }),
            right: Box::new(Value::Integer(1)),
        };
        evals_to(expr.evaluate(&mut scope), Value::Integer(-1));
    }

    #[test]
    fn test_with_identifiers() {
        let mut scope = Scope::new();
        scope.store("a", Value::Integer(12));
        scope.store("b", Value::Integer(4));
        let expr = MinusExpression {
            left: Box::new(MinusExpression {
                left: Box::new(Value::Identifier("a".to_string())),
                right: Box::new(Value::Integer(4)),
            }),
            right: Box::new(Value::Identifier("b".to_string())),
        };
        evals_to(expr.evaluate(&mut scope), Value::Integer(4));
    }
}

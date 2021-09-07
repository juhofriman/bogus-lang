use crate::ast::{Expression, Value, EvalError, Operable};
use crate::ast::scope::Scope;

pub struct PrefixMinusExpression {
    pub expression: Box<dyn Expression>,
    _private: (),
}

impl PrefixMinusExpression {
    pub fn new(expression: Box<dyn Expression>) -> PrefixMinusExpression {
        PrefixMinusExpression {
            expression,
            _private: (),
        }
    }
}

impl Expression for PrefixMinusExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<Value>, EvalError> {
        let left_res = self.expression.evaluate(scope)?;
        let result = left_res.apply_prefix_minus()?;
        Ok(Box::new(result))
    }
    fn visualize(&self, level: usize) {
        println!("{} PrefixMinusExpression", "-".repeat(level));
        self.expression.visualize(level + 1);
    }
}

pub struct MinusExpression {
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
    _private: (),
}

impl MinusExpression {
    pub fn new(left: Box<dyn Expression>, right: Box<dyn Expression>) -> MinusExpression {
        MinusExpression {
            left,
            right,
            _private: (),
        }
    }
}

impl Expression for MinusExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<Value>, EvalError> {
        let left_res = self.left.evaluate(scope)?;
        let right_res = self.right.evaluate(scope)?;
        let result = left_res.apply_minus(&right_res)?;
        Ok(Box::new(result))
    }
    fn visualize(&self, level: usize) {
        println!("{} MinusExpression", "-".repeat(level));
        self.left.visualize(level + 1);
        self.right.visualize(level + 1);
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::{run_expression_tests, ExpressionTest, Expected, evals_to};

    fn create_minus_expression(left: Value, right: Value) -> Box<MinusExpression> {
        Box::new(MinusExpression::new(
            Box::new(left),
            Box::new(right),
        ))
    }

    #[test]
    fn test_prefix_minus() {
        let cases = vec![

            // int to int
            ExpressionTest {
                expression: Box::new(PrefixMinusExpression::new(
                    Box::new(Value::Integer(1))
                )),
                expected: Expected::EvaluatesTo(Value::Integer(-1)),
            },
        ];

        run_expression_tests(cases, None);
    }

    #[test]
    fn test_minus_expression() {
        let cases = vec![

            // int to int
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(1),
                    Value::Integer(1),
                ),
                expected: Expected::EvaluatesTo(Value::Integer(0)),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(-1),
                    Value::Integer(1),
                ),
                expected: Expected::EvaluatesTo(Value::Integer(-2)),
            },

            // int to string
            ExpressionTest {
                expression: create_minus_expression(
                    Value::String("foo".to_string()),
                    Value::Integer(1),
                ),
                expected: Expected::ErrorsTo("Can't apply String - Integer"),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::String("foo".to_string()),
                    Value::Integer(-1),
                ),
                expected: Expected::ErrorsTo("Can't apply String - Integer"),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(1),
                    Value::String("foo".to_string()),
                ),
                expected: Expected::ErrorsTo("Can't apply Integer - String"),
            },
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(-1),
                    Value::String("foo".to_string()),
                ),
                expected: Expected::ErrorsTo("Can't apply Integer - String"),
            },

            // String to String
            ExpressionTest {
                expression: create_minus_expression(
                    Value::String("foo".to_string()),
                    Value::String("bar".to_string()),
                ),
                expected: Expected::ErrorsTo("Can't apply String - String"),
            },

            // Null to int
            ExpressionTest {
                expression: create_minus_expression(
                    Value::Integer(1),
                    Value::Null,
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
        let expr = MinusExpression::new(
            Box::new(MinusExpression::new(
                Box::new(Value::Integer(5)),
                Box::new(Value::Integer(5)),
            )),
            Box::new(Value::Integer(1)),
        );
        evals_to(expr.evaluate(&mut scope), Value::Integer(-1));
    }

    #[test]
    fn test_with_identifiers() {
        let mut scope = Scope::new();
        scope.store("a", Value::Integer(12));
        scope.store("b", Value::Integer(4));
        let expr = MinusExpression::new(
            Box::new(MinusExpression::new(
                Box::new(Value::Identifier("a".to_string())),
                Box::new(Value::Integer(4)),
            )),
            Box::new(Value::Identifier("b".to_string())),
        );
        evals_to(expr.evaluate(&mut scope), Value::Integer(4));
    }
}

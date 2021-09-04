#[derive(Debug)]
struct EvalError {
    msg: String,
}

trait Expression {
    fn evaluate(&self) -> Result<Box<Value>, EvalError>;
}

trait Operable {
    fn apply_plus(&self, other: &Value) -> Result<Value, EvalError>;
}

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Integer(i32),
    String(String),

    Null,
}

impl Value {
    fn name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "Integer",
            Value::String(_) => "String",
            Value::Null => "Null",
        }
    }
}

impl Expression for Value {
    fn evaluate(&self) -> Result<Box<Value>, EvalError> {
        Ok(Box::new(self.clone()))
    }
}

impl Operable for Value {
    fn apply_plus(&self, other: &Value) -> Result<Value, EvalError> {
        Ok(matcher_from_value(self).apply_plus(other)?)
    }
}



////////////////////////////

trait OperatorApplyMatcher {

    fn name(&self) -> &'static str;

    fn apply_plus(&self, other: &Value) -> Result<Value, EvalError> {
        match other {
            Value::Integer(val) => self.apply_plus_with_integer(val),
            Value::String(val) => self.apply_plus_with_string(val),
            Value::Null => self.apply_plus_with_null(),
            anything => Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), anything.name()) })
        }
    }
    fn apply_plus_with_integer(&self, _other: &i32) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), "Integer") })
    }

    fn apply_plus_with_string(&self, _other: &String) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), "String") })
    }

    fn apply_plus_with_null(&self) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), "Null") })
    }
}

struct Matcher<'a, T> {
    value: &'a T,
}

fn matcher_from_value(value: &Value) -> Box<dyn OperatorApplyMatcher + '_> {
    match value {
        Value::Integer(value) => Box::new(Matcher { value }),
        Value::String(value) => Box::new(Matcher { value }),
        Value::Null => Box::new(NullMatcher {}),
        _ => Box::new(FailingMatcher { wrapped_type: value.name() }),
    }
}

impl OperatorApplyMatcher for Matcher<'_, i32> {
    fn name(&self) -> &'static str {
        "Integer"
    }

    fn apply_plus_with_integer(&self, other: &i32) -> Result<Value, EvalError> {
        Ok(Value::Integer(self.value + other))
    }
    fn apply_plus_with_string(&self, other: &String) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.to_string().as_str());
        new.push_str(other.as_str());
        Ok(Value::String(new))
    }
}

impl OperatorApplyMatcher for Matcher<'_, String> {
    fn name(&self) -> &'static str {
        "String"
    }

    fn apply_plus_with_integer(&self, other: &i32) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.as_str());
        new.push_str(other.to_string().as_str());
        Ok(Value::String(new))
    }
    fn apply_plus_with_string(&self, other: &String) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.as_str());
        new.push_str(other.as_str());
        Ok(Value::String(new))
    }
    fn apply_plus_with_null(&self) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.as_str());
        new.push_str("null");
        Ok(Value::String(new))
    }
}

struct NullMatcher {}

impl OperatorApplyMatcher for NullMatcher {
    fn name(&self) -> &'static str {
        "Null"
    }

    fn apply_plus_with_string(&self, other: &String) -> Result<Value, EvalError> {
        let mut new = String::from("null");
        new.push_str(other.as_str());
        Ok(Value::String(new))
    }
}

struct FailingMatcher {
    wrapped_type: &'static str,
}

impl OperatorApplyMatcher for FailingMatcher {
    fn name(&self) -> &'static str {
        self.wrapped_type
    }

    fn apply_plus(&self, other: &Value) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), other.name()) })
    }
}

/////////////////////////////




struct PlusExpression {
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

impl Expression for PlusExpression {
    fn evaluate(&self) -> Result<Box<Value>, EvalError> {
        let left_res = self.left.evaluate()?;
        let right_res = self.right.evaluate()?;
        let result = left_res.apply_plus(&right_res)?;
        Ok(Box::new(result))
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::tests::Expected::EvaluatesTo;

    enum Expected<'a> {
        EvaluatesTo(Value),
        ErrorsTo(&'a str)
    }

    struct ExpressionTest<'a> {
        expression: Box<dyn Expression>,
        expected: Expected<'a>,
    }

    #[test]
    fn test_evaluate_value() {
        let val = Value::Integer(1);
        evals_to(val.evaluate(), Value::Integer(1));
        let val = Value::String("hello".to_string());
        evals_to(val.evaluate(), Value::String("hello".to_string()));

        let cases = vec![
            ExpressionTest {
                expression: Box::new(Value::Integer(1)),
                expected: Expected::EvaluatesTo(Value::Integer(1)),
            },
            ExpressionTest {
                expression: Box::new(Value::String("foo".to_string())),
                expected: Expected::EvaluatesTo(Value::String("foo".to_string())),
            },
            ExpressionTest {
                expression: Box::new(Value::Null),
                expected: Expected::EvaluatesTo(Value::Null),
            },
        ];

        run_expression_tests(cases);
    }

    fn create_plus_expression(left: Value, right: Value) -> Box<PlusExpression> {
        Box::new(PlusExpression {
            left: Box::new(left),
            right: Box::new(right),
        })
    }

    #[test]
    fn test_plus_expression() {
        let cases = vec![

            // int to int
            ExpressionTest {
                expression: create_plus_expression(
                    Value::Integer(1),
                    Value::Integer(1)
                ),
                expected: Expected::EvaluatesTo(Value::Integer(2)),
            },
            ExpressionTest {
                expression: create_plus_expression(
                    Value::Integer(-1),
                    Value::Integer(1)
                ),
                expected: Expected::EvaluatesTo(Value::Integer(0)),
            },

            // int to string
            ExpressionTest {
                expression: create_plus_expression(
                    Value::String("foo".to_string()),
                    Value::Integer(1)
                ),
                expected: Expected::EvaluatesTo(Value::String("foo1".to_string())),
            },
            ExpressionTest {
                expression: create_plus_expression(
                    Value::String("foo".to_string()),
                    Value::Integer(-1)
                ),
                expected: Expected::EvaluatesTo(Value::String("foo-1".to_string())),
            },
            ExpressionTest {
                expression: create_plus_expression(
                    Value::Integer(1),
                Value::String("foo".to_string())
                ),
                expected: Expected::EvaluatesTo(Value::String("1foo".to_string())),
            },
            ExpressionTest {
                expression: create_plus_expression(
                    Value::Integer(-1),
                    Value::String("foo".to_string())
                ),
                expected: Expected::EvaluatesTo(Value::String("-1foo".to_string())),
            },

            // String to String
            ExpressionTest {
                expression: create_plus_expression(
                    Value::String("foo".to_string()),
                    Value::String("bar".to_string())
                ),
                expected: Expected::EvaluatesTo(Value::String("foobar".to_string())),
            },

            // Null to int
            ExpressionTest {
                expression: create_plus_expression(
                    Value::Integer(1),
                    Value::Null
                ),
                expected: Expected::ErrorsTo("Can't apply Integer + Null"),
            },
            ExpressionTest {
                expression: create_plus_expression(
                    Value::Null,
                    Value::Integer(1),
                ),
                expected: Expected::ErrorsTo("Can't apply Null + Integer"),
            },
        ];

        run_expression_tests(cases);
    }

    #[test]
    fn test_nested_expressions() {
        let expr = PlusExpression {
            left: Box::new(PlusExpression {
                left: Box::new(Value::Integer(5)),
                right: Box::new(Value::Integer(5)),
            }),
            right: Box::new(Value::Integer(1)),
        };
        evals_to(expr.evaluate(), Value::Integer(11));
    }

    fn run_expression_tests(cases: Vec<ExpressionTest>) {
        for case in cases {
            match case.expected {
                Expected::EvaluatesTo(value) => evals_to(case.expression.evaluate(), value),
                Expected::ErrorsTo(error_msg) => fails_to(case.expression.evaluate(), error_msg),
            }
        }
    }

    fn evals_to(result: Result<Box<Value>, EvalError>, expected_val: Value) {
        match result {
            Ok(received_val) => assert_eq!(*received_val, expected_val),
            Err(err) => panic!("Expected value but got Err: {}", err.msg),
        }
    }

    fn fails_to(result: Result<Box<Value>, EvalError>, expecter_err: &str) {
        match result {
            Ok(received_val) => panic!("Expected Err but got Value: {:?}", received_val),
            Err(err) => assert_eq!(err.msg, expecter_err)
        }
    }
}
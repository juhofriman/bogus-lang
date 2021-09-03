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
    Integer(u32),
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

fn matcher_from_value(value: &Value) -> Box<dyn OperatorApplyMatcher + '_> {
    match value {
        Value::Integer(value) => Box::new(Matcher { value }),
        _ => Box::new(FailingMatcher { wrapped_type: value.name() }),
    }
}

////////////////////////////

trait OperatorApplyMatcher {

    fn name(&self) -> &'static str;

    fn apply_plus(&self, other: &Value) -> Result<Value, EvalError> {
        match other {
            Value::Integer(val) => self.apply_plus_with_integer(val),
            Value::String(val) => self.apply_plus_with_string(val),
            anything => Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), anything.name()) })
        }
    }
    fn apply_plus_with_integer(&self, _other: &u32) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), "Integer") })
    }

    fn apply_plus_with_string(&self, _other: &String) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), "Integer") })
    }
}

struct Matcher<'a, T> {
    value: &'a T,
}

impl OperatorApplyMatcher for Matcher<'_, u32> {
    fn name(&self) -> &'static str {
        "Integer"
    }

    fn apply_plus_with_integer(&self, other: &u32) -> Result<Value, EvalError> {
        Ok(Value::Integer(self.value + other))
    }
    fn apply_plus_with_string(&self, other: &String) -> Result<Value, EvalError> {
        let mut new = String::new();
        new.push_str(self.value.to_string().as_str());
        new.push_str(other.as_str());
        Ok(Value::String(new))
    }
}

// Tämä niinkuin nullille ja funkkarille
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

    #[test]
    fn test_evaluate_value() {
        let val = Value::Integer(1);
        evals_to(val.evaluate(), Value::Integer(1));
        let val = Value::String("hello".to_string());
        evals_to(val.evaluate(), Value::String("hello".to_string()));
    }

    #[test]
    fn test_integer_plus_integer() {
        evals_to(PlusExpression {
            left: Box::new(Value::Integer(1)),
            right: Box::new(Value::Integer(1)),
        }.evaluate(), Value::Integer(2));
    }

    #[test]
    fn test_integer_plus_string() {
        evals_to(PlusExpression {
            left: Box::new(Value::Integer(1)),
            right: Box::new(Value::String("foo".to_string())),
        }.evaluate(), Value::String("1foo".to_string()));
    }

    #[test]
    fn test_plus_fails() {
        fails_to(PlusExpression {
            left: Box::new(Value::Integer(1)),
            right: Box::new(Value::Null),
        }.evaluate(), "Can't apply Integer + Null");

        fails_to(PlusExpression {
            left: Box::new(Value::Null),
            right: Box::new(Value::Integer(1)),
        }.evaluate(), "Can't apply Null + Integer");
    }

    #[test]
    fn test_nested_plus() {
        let expr = PlusExpression {
            left: Box::new(PlusExpression {
                left: Box::new(Value::Integer(5)),
                right: Box::new(Value::Integer(5)),
            }),
            right: Box::new(Value::Integer(1)),
        };
        evals_to(expr.evaluate(), Value::Integer(11));
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
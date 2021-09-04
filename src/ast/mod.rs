use crate::ast::m_null::NullMatcher;
use crate::ast::scope::Scope;

mod m_integer;
mod m_string;
mod m_null;
mod e_plus;
mod e_minus;
mod scope;

/// Common error in evaluation
#[derive(Debug)]
pub struct EvalError {
    msg: String,
}

/// Expression has evaluate(&self). Evaluating expression returns Boxed Value.
pub trait Expression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<Value>, EvalError>;
}

/// Operable gives possibility to apply operators. Operations return NEW Value.
pub trait Operable {
    fn apply_plus(&self, other: &Value) -> Result<Value, EvalError>;
    fn apply_minus(&self, other: &Value) -> Result<Value, EvalError>;

    // ... and all operators will eventually follow ...
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Identifier(String),

    Integer(i32),
    String(String),

    Null,
    Void,
}

impl Value {
    fn name(&self) -> &'static str {
        match self {
            Value::Identifier(_) => "Identifier",
            Value::Integer(_) => "Integer",
            Value::String(_) => "String",
            Value::Null => "Null",
            Value::Void => "Void",
        }
    }
}

impl Expression for Value {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<Value>, EvalError> {
        match self {
            // Identifiers are resolved from given scope
            Value::Identifier(name) => {
                match scope.resolve(name.as_str()) {
                    Some(value) => Ok(Box::new(value.clone())),
                    None => Err( EvalError {
                        msg: format!("Can't resolve identifier '{}'", name)
                    } )
                }
            },
            // Unfortunately current implementation enforces to clone values when evaluated
            _ => Ok(Box::new(self.clone()))
        }
    }
}

impl Operable for Value {
    fn apply_plus(&self, other: &Value) -> Result<Value, EvalError> {
        Ok(matcher_from_value(self).apply_plus(other)?)
    }
    fn apply_minus(&self, other: &Value) -> Result<Value, EvalError> {
        Ok(matcher_from_value(self).apply_minus(other)?)
    }
}

pub trait OperatorApplyMatcher {

    fn name(&self) -> &'static str;

    fn apply_plus(&self, other: &Value) -> Result<Value, EvalError> {
        match other {
            Value::Integer(val) => self.apply_plus_with_integer(val),
            Value::String(val) => self.apply_plus_with_string(val),
            Value::Null => self.apply_plus_with_null(),
            anything => Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), anything.name()) })
        }
    }

    fn apply_minus(&self, other: &Value) -> Result<Value, EvalError> {
        match other {
            Value::Integer(val) => self.apply_minus_with_integer(val),
            Value::String(val) => self.apply_minus_with_string(val),
            Value::Null => self.apply_minus_with_null(),
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

    fn apply_minus_with_integer(&self, _other: &i32) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} - {}", self.name(), "Integer") })
    }

    fn apply_minus_with_string(&self, _other: &String) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} - {}", self.name(), "String") })
    }

    fn apply_minus_with_null(&self) -> Result<Value, EvalError> {
        Err(EvalError { msg: format!("Can't apply {} - {}", self.name(), "Null") })
    }
}

pub struct Matcher<'a, T> {
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

/// Failing matcher fails always, this is the default matcher for any value
/// that do not have own matcher
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::scope::Scope;

    pub enum Expected<'a> {
        EvaluatesTo(Value),
        ErrorsTo(&'a str)
    }

    pub struct ExpressionTest<'a> {
        pub expression: Box<dyn Expression>,
        pub expected: Expected<'a>,
    }

    #[test]
    fn test_evaluate_value() {
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

        run_expression_tests(cases, None);
    }

    #[test]
    fn test_evaluate_identifier() {
        let mut scope = Scope::new();
        scope.store("a", Value::Integer(1));

        evals_to(Value::Identifier("a".to_string()).evaluate(&mut scope),
                 Value::Integer(1));

        fails_to(Value::Identifier("b".to_string()).evaluate(&mut scope),
                 "Can't resolve identifier 'b'");
    }

    pub fn run_expression_tests(cases: Vec<ExpressionTest>, base_scope: Option<&Scope>) {
        for case in cases {
            let mut scope = match base_scope {
                // This needs to clone the given base scope
                Some(_) => panic!("Base scope not implemented!"),
                None => Scope::new(),
            };
            match case.expected {
                Expected::EvaluatesTo(value) =>
                    evals_to(case.expression.evaluate(&mut scope), value),
                Expected::ErrorsTo(error_msg) =>
                    fails_to(case.expression.evaluate(&mut scope), error_msg),
            }
        }
    }

    pub fn evals_to(result: Result<Box<Value>, EvalError>, expected_val: Value) {
        match result {
            Ok(received_val) => assert_eq!(*received_val, expected_val),
            Err(err) => panic!("Expected value but got Err: {}", err.msg),
        }
    }

    pub fn fails_to(result: Result<Box<Value>, EvalError>, expecter_err: &str) {
        match result {
            Ok(received_val) => panic!("Expected Err but got Value: {:?}", received_val),
            Err(err) => assert_eq!(err.msg, expecter_err)
        }
    }
}
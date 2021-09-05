use crate::ast::m_null::NullMatcher;
use crate::ast::scope::Scope;
use std::fmt::{Display, Formatter};

mod m_integer;
mod m_string;
mod m_null;
pub mod e_plus;
pub mod e_minus;
pub mod scope;
pub mod s_let;

/// Common error in evaluation
#[derive(Debug)]
pub struct EvalError {
    msg: String,
}

impl Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Eval Error: {}", self.msg)
    }
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
    fn type_name(&self) -> &'static str {
        match self {
            Value::Identifier(_) => "Identifier",
            Value::Integer(_) => "Integer",
            Value::String(_) => "String",
            Value::Null => "Null",
            Value::Void => "Void",
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Identifier(val) =>
                write!(f, "{}", val),
            Value::Integer(val) =>
                write!(f, "{}", val),
            Value::String(val) =>
                write!(f, "{}", val),
            Value::Null =>
                write!(f, "{}", self.type_name()),
            Value::Void =>
                write!(f, "{}", self.type_name()),
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
            anything => Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), anything.type_name()) })
        }
    }

    fn apply_minus(&self, other: &Value) -> Result<Value, EvalError> {
        match other {
            Value::Integer(val) => self.apply_minus_with_integer(val),
            Value::String(val) => self.apply_minus_with_string(val),
            Value::Null => self.apply_minus_with_null(),
            anything => Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), anything.type_name()) })
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
        _ => Box::new(FailingMatcher { wrapped_type: value.type_name() }),
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
        Err(EvalError { msg: format!("Can't apply {} + {}", self.name(), other.type_name()) })
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::scope::Scope;
    use crate::ast::s_let::LetStatement;
    use crate::ast::e_plus::PlusExpression;

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

    #[test]
    fn test_simple_program() {
        let mut scope = Scope::new();
        let s1 = LetStatement {
            identifier: "foo".to_string(),
            expression: Box::new(Value::Integer(1)),
        };
        let s2 = LetStatement {
            identifier: "bar".to_string(),
            expression: Box::new(Value::Integer(2)),
        };
        let s3 = LetStatement {
            identifier: "bax".to_string(),
            expression: Box::new(PlusExpression {
                left: Box::new(Value::Identifier("foo".to_string())),
                right: Box::new(Value::Identifier("bar".to_string())),
            }),
        };
        evals_to(s1.evaluate(&mut scope), Value::Void);
        evals_to(s2.evaluate(&mut scope), Value::Void);
        evals_to(s3.evaluate(&mut scope), Value::Void);

        assert_eq!(scope.resolve("foo"), Some(&Value::Integer(1)));
        assert_eq!(scope.resolve("bar"), Some(&Value::Integer(2)));
        assert_eq!(scope.resolve("bax"), Some(&Value::Integer(3)));
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
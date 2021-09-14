use crate::ast::scope::Scope;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

pub mod v_integer;
pub mod scope;
pub mod e_plus;
pub mod e_identifier;
pub mod s_fun;
pub mod e_call;
mod v_void;
pub mod e_minus;
pub mod s_let;
pub mod e_multiplication;

#[derive(Debug)]
pub struct EvaluationError {
    msg: String,
}

impl EvaluationError {
    pub fn new(msg: String) -> EvaluationError {
        EvaluationError {
            msg,
        }
    }
    pub fn not_callable(me: TypeMatcher) -> EvaluationError {
        EvaluationError::new(format!("{} is not callable", me))
    }
    pub fn cant_resolve(name: &str) -> EvaluationError {
        EvaluationError::new(format!("Can't resolve variable `{}`", name))
    }
    pub fn does_not_support_prefix_minus(me: TypeMatcher) -> EvaluationError {
        EvaluationError::new(format!("{} does not support prefix minus", me))
    }
    pub fn operator_not_applicable(
        operator: &str,
        me: TypeMatcher,
        he_or_she: TypeMatcher) -> EvaluationError {
        EvaluationError::new(
            format!("Can't apply {} {} {}",
                    me,
                operator,
                    he_or_she))
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeMatcher<'a> {
    Integer(&'a i32),
    Null,
    Void,
    Function,
}

impl Display for TypeMatcher<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeMatcher::Integer(v) => write!(f, "{}", v),
            TypeMatcher::Null => write!(f, "Null"),
            TypeMatcher::Void => write!(f, "Void"),
            TypeMatcher::Function => write!(f, "Fn"),
        }
    }
}

// Hmm, programmatic equality should be in Value trait, this way TypeMatcher can
// be used for test assertions (Void and such)
// impl PartialEq for TypeMatcher<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         match self {
//             TypeMatcher::Integer(self_val) => match other {
//                 TypeMatcher::Integer(other_val) => self_val == other_val,
//                 _ => false,
//             },
//             _ => panic!("PartialEq not implemented for {:?}", self)
//         }
//     }
// }

pub trait Expression {
    fn get_identifier(&self) -> Result<&String, EvaluationError> {
        Err( EvaluationError { msg: "Token does not have identifier".to_string() } )
    }
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError>;
    fn visualize(&self, level: usize);
}

pub trait Value {
    fn type_matcher(&self) -> TypeMatcher;
    fn apply_prefix_minus(&self) -> Result<Rc<dyn Value>, EvaluationError> {
        Err( EvaluationError::does_not_support_prefix_minus(self.type_matcher()))
    }
    fn apply_plus(&self, other: Rc<dyn Value>) ->  Result<Rc<dyn Value>, EvaluationError> {
        Err( EvaluationError::operator_not_applicable(
            "+",
            self.type_matcher(),
            other.type_matcher()))
    }
    fn apply_minus(&self, other: Rc<dyn Value>) ->  Result<Rc<dyn Value>, EvaluationError> {
        Err( EvaluationError::operator_not_applicable(
            "-",
            self.type_matcher(),
            other.type_matcher()))
    }
    fn apply_multiplication(&self, other: Rc<dyn Value>) ->  Result<Rc<dyn Value>, EvaluationError> {
        Err( EvaluationError::operator_not_applicable(
            "*",
            self.type_matcher(),
            other.type_matcher()))
    }
    fn call(&self, _scope: &mut Scope, _args: Vec<Rc<dyn Value>>) -> Result<Rc<dyn Value>, EvaluationError> {
        Err( EvaluationError::not_callable(self.type_matcher()))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Internal implementation test helpers

    pub fn evaluates_to(result: Result<Rc<dyn Value>, EvaluationError>,
                        expected: Rc<dyn Value>) {
        match result {
            Ok(val) => assert_eq!(val.type_matcher(), expected.type_matcher()),
            Err(e) => panic!("Unexpected err: {:?}", e)
        }
    }

    pub fn evaluates_to_void(result: Result<Rc<dyn Value>, EvaluationError>) {
        match result {
            Ok(val) => match val.type_matcher() {
                TypeMatcher::Void => (),
                t => panic!("Expecting Void, but {} received", t)
            },
            Err(e) => panic!("Unexpected err: {:?}", e)
        }
    }

    pub fn errors_to(result: Result<Rc<dyn Value>, EvaluationError>,
                     expected_msg: &str) {
        match result {
            Ok(val) =>
                panic!("Expected evaluation to fail, but got: {:?}", val.type_matcher()),
            Err(e) => assert_eq!(e.msg, expected_msg),
        }
    }
}
use crate::astplus::scope::Scope;
use std::fmt::{Display, Formatter};

pub mod v_integer;
pub mod scope;
pub mod e_plus;

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

#[derive(Debug)]
pub enum TypeMatcher<'a> {
    Integer(&'a i32),
    Null,
}

impl Display for TypeMatcher<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeMatcher::Integer(_) => write!(f, "Integer"),
            TypeMatcher::Null => write!(f, "Null"),
        }
    }
}

impl PartialEq for TypeMatcher<'_> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TypeMatcher::Integer(self_val) => match other {
                TypeMatcher::Integer(other_val) => self_val == other_val,
                _ => false,
            },
            _ => panic!("PartialEq not implemented for {:?}", self)
        }
    }
}

pub trait Expression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Box<dyn Value>, EvaluationError>;
}

pub trait Value {
    fn type_matcher(&self) -> TypeMatcher;
    fn apply_plus(&self, other: Box<dyn Value>) ->  Result<Box<dyn Value>, EvaluationError> {
        Err( EvaluationError::operator_not_applicable(
            "+",
            self.type_matcher(),
            other.type_matcher()))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // Internal implementation test helpers
    pub fn evaluates_to(result: Result<Box<dyn Value>, EvaluationError>,
                    expected: Box<dyn Value>) {
        match result {
            Ok(val) => assert_eq!(val.type_matcher(), expected.type_matcher()),
            Err(e) => panic!("Unexpected err: {:?}", e)
        }
    }
}
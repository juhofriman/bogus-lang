use crate::astplus::scope::Scope;

pub mod v_integer;
pub mod scope;

#[derive(Debug)]
pub struct EvaluationError {
    msg: String,
}

impl EvaluationError {
    pub fn new(msg: &str) -> EvaluationError {
        EvaluationError {
            msg: msg.to_string()
        }
    }
}

#[derive(Debug)]
pub enum TypeMatcher<'a> {
    Integer(&'a i32),
    Null,
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
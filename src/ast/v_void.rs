use crate::ast::{Value, TypeMatcher};

pub struct Void;

impl Value for Void {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Void
    }
}

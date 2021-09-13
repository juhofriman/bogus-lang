use crate::astplus::{Expression, Scope, Value, EvaluationError, TypeMatcher};
use std::rc::Rc;

pub struct Void;

impl Value for Void {

    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Void
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::astplus::tests::evaluates_to;

}
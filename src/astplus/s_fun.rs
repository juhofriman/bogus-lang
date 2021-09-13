use crate::astplus::scope::Scope;
use std::rc::Rc;
use crate::astplus::{Expression, Value, EvaluationError, TypeMatcher};
use crate::astplus::v_void::Void;

pub struct FunStatement {
    identifier: String,
    expression: Rc<dyn Expression>,
}

impl FunStatement {
    pub fn new(identifier: String, expression: Rc<dyn Expression>) -> FunStatement {
        FunStatement {
            identifier,
            expression,
        }
    }
}

impl Expression for FunStatement {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        scope.store(self.identifier.clone(),
                    Rc::new(Function { expression: self.expression.clone() }));
        Ok(Rc::new(Void))
    }
}

struct Function {
    expression: Rc<dyn Expression>,
}

impl Value for Function {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Function
    }
    fn call(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        self.expression.evaluate(scope)
    }
}
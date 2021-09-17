use std::rc::Rc;
use crate::ast::{Value, Expression, EvaluationError};
use crate::ast::s_fun::Function;
use crate::ast::e_identifier::IdentifierExpression;
use crate::ast::scope::Scope;
use crate::ast::v_void::Void;



pub struct NameAndValue {
    pub name: String,
    pub value: Rc<dyn Value>,
}

struct RustExpression {
    native_hook: fn(scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError>,
}

impl Expression for RustExpression {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        (self.native_hook)(scope)
    }
    fn visualize(&self, _level: usize) {
        todo!()
    }
}

pub fn io_functions() -> [NameAndValue; 2] {
    [
        NameAndValue {
            name: "print".to_string(),
            value: Function::rc(
                vec![IdentifierExpression::new("a".to_string())],
                Rc::new(RustExpression {
                    native_hook: |scope| {
                        print!("{}", scope.resolve_result(&"a".to_string())?.type_matcher());
                        Ok(Rc::new(Void))
                    }
                }),
            ),
        },
        NameAndValue {
            name: "println".to_string(),
            value: Function::rc(
                vec![IdentifierExpression::new("a".to_string())],
                Rc::new(RustExpression {
                    native_hook: |scope| {
                        println!("{}", scope.resolve_result(&"a".to_string())?.type_matcher());
                        Ok(Rc::new(Void))
                    }
                }),
            ),
        }
    ]
}
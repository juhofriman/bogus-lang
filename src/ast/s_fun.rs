use crate::ast::scope::Scope;
use std::rc::Rc;
use crate::ast::{Expression, Value, EvaluationError, TypeMatcher};
use crate::ast::v_void::Void;
use crate::ast::e_identifier::IdentifierExpression;

pub struct FunStatement {
    identifier: String,
    args: Rc<Vec<IdentifierExpression>>,
    expression: Rc<dyn Expression>,
}

impl FunStatement {
    pub fn new(identifier: String, args: Vec<IdentifierExpression>, expression: Rc<dyn Expression>) -> FunStatement {
        FunStatement {
            identifier,
            args: Rc::new(args),
            expression,
        }
    }
    pub fn rc(identifier: String, args: Vec<IdentifierExpression>, expression: Rc<dyn Expression>) -> Rc<FunStatement> {
        Rc::new(FunStatement::new(identifier, args, expression))
    }
}

impl Expression for FunStatement {
    fn evaluate(&self, scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        scope.store(self.identifier.clone(),
                    Rc::new(Function { expression: self.expression.clone(), args: self.args.clone() }));
        Ok(Rc::new(Void))
    }
    fn visualize(&self, level: usize) {
        println!("{} FunStatement {}({} args)", "-".repeat(level), self.identifier, self.args.len());
        self.expression.visualize(level + 1);
    }
}

pub struct AnonFunction {
    expression: Rc<dyn Expression>,
    args: Rc<Vec<IdentifierExpression>>,
}

impl AnonFunction {
    pub fn rc(args: Vec<IdentifierExpression>, expression: Rc<dyn Expression>) -> Rc<AnonFunction> {
        Rc::new( AnonFunction {
            expression,
            args: Rc::new(args),
        })
    }
}

impl Expression for AnonFunction {
    fn evaluate(&self, _scope: &mut Scope) -> Result<Rc<dyn Value>, EvaluationError> {
        Ok(Rc::new(Function { expression: self.expression.clone(), args: self.args.clone() }))
    }
    fn visualize(&self, level: usize) {
        println!("{} AnonFunction _({} args)", "-".repeat(level), self.args.len());
        self.expression.visualize(level + 1);
    }
}

pub struct Function {
    expression: Rc<dyn Expression>,
    args: Rc<Vec<IdentifierExpression>>,
}

impl Function {
    pub fn rc(args: Vec<IdentifierExpression>, expression: Rc<dyn Expression>) -> Rc<Function> {
        Rc::new( Function {
            expression,
            args: Rc::new(args),
        })
    }
}

impl Value for Function {
    fn type_matcher(&self) -> TypeMatcher {
        TypeMatcher::Function
    }
    fn call(&self, scope: &mut Scope, args: Vec<Rc<dyn Value>>) -> Result<Rc<dyn Value>, EvaluationError> {
        if self.args.len() != args.len() {
            return Err( EvaluationError::invalid_arity(self.args.len(), args.len()) )
        }
        let mut new_scope = Scope::sub(&scope);
        for (i, a) in self.args.iter().enumerate() {
            match args.get(i) {
                Some(e) => {
                    new_scope.store(a.name(), e.clone())
                },
                None => return Err(EvaluationError { msg: format!("No arg for binding {}", i) })
            }

        }
        // TODO: this probably must wrap ReturnValue out
        self.expression.evaluate(&mut new_scope)
    }
}
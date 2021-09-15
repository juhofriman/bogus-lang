use crate::parser::{Parselet, ParseError};
use crate::lexer::Lexer;
use crate::ast::v_integer::{IntegerExpression};
use crate::ast::Expression;
use std::rc::Rc;

pub struct IntegerParselet {
    pub value: i32,
}

impl Parselet for IntegerParselet {

    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Ok(IntegerExpression::rc(self.value))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Err(ParseError { msg: "Can't parse integer in LED position".to_string() })
    }
}

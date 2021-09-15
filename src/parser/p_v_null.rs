use crate::parser::{Parselet, ParseError};
use crate::lexer::Lexer;
use std::rc::Rc;
use crate::ast::Expression;
use crate::ast::v_null::Null;

pub struct NullParselet {}

impl Parselet for NullParselet {

    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Ok(Null::rc())
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Err(ParseError { msg: "Can't parse null in LED position".to_string() })
    }
}
use crate::parser::{Parselet, ParseError};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;

pub struct SemicolonParselet {}

impl Parselet for SemicolonParselet {

    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Err( ParseError { msg: "Can't parse ; in prefix position".to_string() } )
    }

    fn led(&self, _lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Ok(left)
    }
}
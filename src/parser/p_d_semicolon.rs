use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::astplus::Expression;
use std::rc::Rc;

pub struct SemicolonParselet {}

impl Parselet for SemicolonParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse ; in prefix position".to_string() } )
    }

    fn led(&self, _lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Ok(Some(left))
    }
}
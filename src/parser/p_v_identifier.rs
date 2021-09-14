use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::astplus::{Expression};
use crate::astplus::e_identifier::IdentifierExpression;
use std::rc::Rc;

pub struct IdentifierParselet {
    pub value: String,
}

impl Parselet for IdentifierParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Ok(Some(IdentifierExpression::rc(self.value.clone())))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err(ParseError { msg: "Can't parse identifier in LED position".to_string() })
    }
}
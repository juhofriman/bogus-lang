use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::astplus::Expression;
use std::rc::Rc;

pub struct LeftParensParselet {}

impl Parselet for LeftParensParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        let expr = parse_expression(
            0,
            lexer)?;
        Ok(Some(expr))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse ( in NUD position".to_string() } )
    }
}

pub struct RightParensParselet {}

impl Parselet for RightParensParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse ) in prefix position".to_string() } )
    }

    fn led(&self, _lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Ok(Some(left))
    }
}
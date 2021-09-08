use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use crate::ast::e_multiplication::MultiplicationExpression;

pub struct MultiplicationParselet {}

impl Parselet for MultiplicationParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse * in prefix position".to_string() } )
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let right = parse_expression(
            10,
            lexer)?;

        Ok(Some(Box::new(MultiplicationExpression::new(
            left,
            right,
        ))))
    }
}
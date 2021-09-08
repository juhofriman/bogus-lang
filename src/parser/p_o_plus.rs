use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use crate::ast::e_plus::PlusExpression;

pub struct PlusParselet {}

impl Parselet for PlusParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let expression = parse_expression(
            0,
            lexer)?;
        // This does not create extra expression. Side effect is that +"foo" -> "foo".
        Ok(Some(expression))
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let right = parse_expression(
            5,
            lexer)?;

        Ok(Some(Box::new(PlusExpression::new(
            left,
            right,
        ))))
    }
}
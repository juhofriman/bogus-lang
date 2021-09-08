use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use crate::ast::e_minus::{PrefixMinusExpression, MinusExpression};

pub struct MinusParselet {}

impl Parselet for MinusParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let expression = parse_expression(
            5,
            lexer)?;
        Ok(Some(Box::new(PrefixMinusExpression::new(expression))))
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let right = parse_expression(
            5,
            lexer)?;

        Ok(Some(Box::new(MinusExpression::new(
            left,
            right,
        ))))
    }
}
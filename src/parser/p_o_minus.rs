use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use std::rc::Rc;
use crate::ast::Expression;
use crate::ast::e_minus::{PrefixMinusExpression, MinusExpression};

pub struct MinusParselet {}

impl Parselet for MinusParselet {

    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        let expression = parse_expression(
            5,
            lexer)?;
        Ok(Rc::new(PrefixMinusExpression::new(expression)))
    }

    fn led(&self, lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        let right = parse_expression(
            5,
            lexer)?;

        Ok(MinusExpression::rc(
            left,
            right,
        ))
    }
}
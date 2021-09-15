use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use crate::ast::e_plus::PlusExpression;
use std::rc::Rc;


pub struct PlusParselet {}

impl Parselet for PlusParselet {

    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        let expression = parse_expression(
            0,
            lexer)?;
        // This does not create extra expression. Side effect is that +"foo" -> "foo".
        Ok(expression)
    }

    fn led(&self, lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        let right = parse_expression(
            5,
            lexer)?;

        Ok(Rc::new(PlusExpression::new(
            left,
            right,
        )))
    }
}
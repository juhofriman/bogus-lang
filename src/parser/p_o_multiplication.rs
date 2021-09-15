use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::e_multiplication::MultiplicationExpression;

pub struct MultiplicationParselet {}

impl Parselet for MultiplicationParselet {

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse * in prefix position".to_string() } )
    }

    fn led(&self, lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        let right = parse_expression(
            10,
            lexer)?;

        Ok(Some(Rc::new(MultiplicationExpression::new(
            left,
            right,
        ))))
    }
}
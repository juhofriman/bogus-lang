use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::s_let::LetStatement;


pub struct LetParselet {}

impl Parselet for LetParselet {

    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        let next_token = lexer.next_or_err()?;
        let identifier = next_token.is_identifier()?;
        lexer.next_or_err()?.is_assing()?;

        let expr = parse_expression(
            0,
            lexer)?;

        Ok(LetStatement::rc(
            identifier,
            expr,
        ))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Err(ParseError { msg: "Can't parse let in infix position".to_string() })
    }
}
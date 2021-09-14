use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::astplus::Expression;
use std::rc::Rc;
use crate::astplus::s_let::LetStatement;


pub struct LetParselet {}

impl Parselet for LetParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        match lexer.next() {
            Some(token) => {
                let identifier = token.is_identifier()?;
                lexer.next()
                    .ok_or(ParseError { msg: "Expecting = but EOF encountered".to_string() })?
                    .is_assing()?;
                let expr = parse_expression(
                    0,
                    lexer)?;
                Ok(Some(Rc::new(LetStatement::new(
                    identifier,
                    expr,
                ))))
            },
            None => Err( ParseError { msg: "Expecting identifier but EOF encountered".to_string() } )
        }
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse let in infix position".to_string() } )
    }
}
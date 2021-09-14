use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::s_let::LetStatement;
use crate::ast::s_fun::FunStatement;

pub struct FunParselet {}

impl Parselet for FunParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        match lexer.next() {
            Some(token) => {
                let identifier = token.is_identifier()?;
                lexer.next()
                    .ok_or(ParseError { msg: "Expecting = but EOF encountered".to_string() })?
                    .is_left_parens()?;
                lexer.next()
                    .ok_or(ParseError { msg: "Expecting = but EOF encountered".to_string() })?
                    .is_right_parens()?;
                lexer.next()
                    .ok_or(ParseError { msg: "Expecting = but EOF encountered".to_string() })?
                    .is_arrow()?;
                let expr = parse_expression(
                    0,
                    lexer)?;
                Ok(Some(Rc::new(FunStatement::new(
                    identifier,
                    expr,
                ))))
            },
            None => Err( ParseError { msg: "Expecting identifier but EOF encountered".to_string() } )
        }
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse fun in infix position".to_string() } )
    }
}
use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;



use crate::ast::s_if::IfStatement;

pub struct IfParselet {}

impl Parselet for IfParselet {
    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        let cond = parse_expression(1, lexer)?;
        let branch = parse_expression(1, lexer)?;
        Ok(IfStatement::rc(cond, branch))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Err(ParseError { msg: "Can't parse if in infix position".to_string() })
    }
}
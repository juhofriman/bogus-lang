use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;



use crate::ast::s_return::ReturnStatement;

pub struct ReturnParselet {}

impl Parselet for ReturnParselet {
    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        let return_expression = parse_expression(1, lexer)?;
        Ok(ReturnStatement::rc(return_expression))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        todo!()
    }
}
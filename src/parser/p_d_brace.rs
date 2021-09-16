use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;

use crate::ast::s_grouped::GroupedStatement;

pub struct LeftBraceParselet {}

impl Parselet for LeftBraceParselet {
    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        let mut children: Vec<Rc<dyn Expression>> = vec![];
        loop {
            if lexer.peek_or_err()?.is_right_brace().is_ok() {
                lexer.next();
                break;
            }
            children.push(parse_expression(1, lexer)?);
            if lexer.peek_or_err()?.is_semicolon().is_ok() {
                lexer.next();
            }
        }
        Ok(GroupedStatement::rc(children))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Err( ParseError { msg: "Can't parse { in LED position".to_string() } )
    }
}

pub struct RightBraceParselet {}

impl Parselet for RightBraceParselet {
    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Err( ParseError { msg: "Can't parse } in NUD position".to_string() } )
    }

    fn led(&self, _lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Ok(left)
    }
}
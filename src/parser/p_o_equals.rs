use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;

use std::rc::Rc;
use crate::ast::e_equals::EqualsExpression;

pub enum EqualsOrNequals {
    Equals,
    Nequals,
}

pub struct EqualsParselet {
    pub equality_type: EqualsOrNequals,
}

impl Parselet for EqualsParselet {
    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Err(ParseError { msg: "Can't parse == in prefix position".to_string() })
    }

    fn led(&self, lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        let right = parse_expression(
            5,
            lexer)?;

        match self.equality_type {
            EqualsOrNequals::Equals => Ok(EqualsExpression::rc(
                false,
                left,
                right,
            )),
            EqualsOrNequals::Nequals => Ok(EqualsExpression::rc(
                true,
                left,
                right,
            ))
        }
    }
}
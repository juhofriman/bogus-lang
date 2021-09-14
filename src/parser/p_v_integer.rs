use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::astplus::v_integer::{IntegerExpression};
use crate::astplus::Expression;
use std::rc::Rc;

pub struct IntegerParselet {
    pub value: i32,
}

impl Parselet for IntegerParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Ok(Some(Rc::new(IntegerExpression::new(self.value))))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Ok(None)
    }
}

use crate::parser::{Parselet, ParseError};
use crate::lexer::Lexer;

use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::v_boolean::BooleanExpression;

pub struct BooleanParselet {
    pub value: bool,
}

impl Parselet for BooleanParselet {

    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Ok(BooleanExpression::rc(self.value))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Err(ParseError { msg: "Can't parse Boolean in LED position".to_string() })
    }
}

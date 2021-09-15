use crate::parser::{Parselet, ParseError};
use crate::lexer::Lexer;
use crate::ast::Expression;
use crate::ast::v_string::StringExpression;
use std::rc::Rc;

pub struct StringParselet {
    pub value: String,
}

impl Parselet for StringParselet {

    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Ok(StringExpression::rc(self.value.clone()))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Err( ParseError { msg: "Can't parse string in LED position".to_string() } )
    }
}

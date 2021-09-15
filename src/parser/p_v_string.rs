// use crate::parser::{Parselet, ParseError};
// use crate::lexer::Lexer;
// use crate::ast::{Expression, Value};
//
// pub struct StringParselet {
//     pub value: String,
// }
//
// impl Parselet for StringParselet {
//
//     fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
//         Ok(Some(Box::new(Value::String(self.value.clone()))))
//     }
//
//     fn led(&self, _lexer: &mut Lexer, _left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
//         Err( ParseError { msg: "Can't parse string in LED".to_string() } )
//     }
// }

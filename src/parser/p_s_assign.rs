use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::s_assign::AssignStatement;


pub struct AssignParselet {}

impl Parselet for AssignParselet {

    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Err( ParseError { msg: "Can't parse = in NUD position".to_string() })
    }

    fn led(&self, _lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        match left.get_identifier() {
            Ok(identifier) => Ok(
                AssignStatement::rc(
                    identifier.clone(),
                    parse_expression(1, _lexer)?
                )),
            Err(_) => Err( ParseError { msg: "Expecting identifier before =".to_string() })
        }
    }
}
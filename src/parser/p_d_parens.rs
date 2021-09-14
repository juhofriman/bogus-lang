use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::e_call::CallExpression;

pub struct LeftParensParselet {}

impl Parselet for LeftParensParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        let expr = parse_expression(
            0,
            lexer)?;
        Ok(Some(expr))
    }

    fn led(&self, lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        match left.get_identifier() {
            Ok(identifier) => {
                lexer.next()
                    .ok_or(ParseError { msg: "Expecting ) but EOF encountered".to_string() })?
                    .is_right_parens()?;
                Ok(Some(CallExpression::rc(identifier.clone())))
            },
            Err(_) => Err( ParseError { msg: "Expecting identifier before left parens".to_string() } )
        }
    }
}

pub struct RightParensParselet {}

impl Parselet for RightParensParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse ) in prefix position".to_string() } )
    }

    fn led(&self, _lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Ok(Some(left))
    }
}
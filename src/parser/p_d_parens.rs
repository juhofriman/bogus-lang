use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::e_call::CallExpression;

pub struct LeftParensParselet {}

impl Parselet for LeftParensParselet {
    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        let expr = parse_expression(
            0,
            lexer)?;
        Ok(expr)
    }

    fn led(&self, lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        // Idea: this could check, that left.is_callable()!!!
        // now this evaluates to Fn `fun () -> 1()`, this `(fun () -> 1)()` works as expected
        let mut args: Vec<Rc<dyn Expression>> = vec![];
        loop {
            if lexer.peek_or_err()?.is_right_parens().is_ok() {
                lexer.next();
                break;
            }
            args.push(parse_expression(1, lexer)?);
            if lexer.peek_or_err()?.is_comma().is_ok() {
                lexer.next();
            }
        }
        Ok(CallExpression::rc(left, args))
    }
}

pub struct RightParensParselet {}

impl Parselet for RightParensParselet {
    fn nud(&self, _lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        Err(ParseError { msg: "Can't parse ) in prefix position".to_string() })
    }

    fn led(&self, _lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Ok(left)
    }
}
use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::s_fun::FunStatement;
use crate::ast::e_identifier::IdentifierExpression;
use crate::lexer::tokens::TokenKind;

pub struct FunParselet {}

impl Parselet for FunParselet {

    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        let identifier = lexer.next_or_err()?.is_identifier()?;
        lexer.next_or_err()?
            .is_left_parens()?;

        let mut args: Vec<IdentifierExpression> = vec![];
        loop {
            let token = lexer.next_or_err()?;
            if token.is_right_parens().is_ok() {
                break;
            }

            match &token.token_kind {
                TokenKind::Identifier(name) => {
                    args.push(IdentifierExpression::new(name.clone()))
                }
                TokenKind::Comma => continue,
                _ => return Err(ParseError { msg: "Expecting identifier or ,".to_string() })
            }
        }

        lexer.next_or_err()?
            .is_arrow()?;

        let expr = parse_expression(
            0,
            lexer)?;

        Ok(FunStatement::rc(
            identifier,
            args,
            expr,
        ))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError> {
        Err(ParseError { msg: "Can't parse fun in infix position".to_string() })
    }
}
use crate::parser::{Parselet, ParseError, parse_expression};
use crate::lexer::Lexer;
use crate::ast::Expression;
use std::rc::Rc;
use crate::ast::s_fun::FunStatement;
use crate::ast::e_identifier::IdentifierExpression;
use crate::lexer::tokens::TokenKind;

pub struct FunParselet {}

impl Parselet for FunParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        match lexer.next() {
            Some(token) => {
                let identifier = token.is_identifier()?;
                lexer.next()
                    .ok_or(ParseError { msg: "Expecting = but EOF encountered".to_string() })?
                    .is_left_parens()?;
                let mut args: Vec<IdentifierExpression> = vec![];
                loop {
                    match &lexer.next() {
                        Some(token) => {
                            if token.is_right_parens().is_ok() {
                                break;
                            }
                            match &token.token_kind {
                                TokenKind::Identifier(name) => {
                                    args.push(IdentifierExpression::new(name.clone()))
                                },
                                TokenKind::Comma => continue,
                                _ => return Err(ParseError { msg: "Expecting identifier or ,".to_string() })
                            }
                        },
                        None => return Err(ParseError { msg: "Expecting = but EOF encountered".to_string() }),
                    }
                }

                lexer.next()
                    .ok_or(ParseError { msg: "Expecting = but EOF encountered".to_string() })?
                    .is_arrow()?;
                let expr = parse_expression(
                    0,
                    lexer)?;
                Ok(Some(Rc::new(FunStatement::new(
                    identifier,
                    args,
                    expr,
                ))))
            },
            None => Err( ParseError { msg: "Expecting identifier but EOF encountered".to_string() } )
        }
    }

    fn led(&self, _lexer: &mut Lexer, _left: Rc<dyn Expression>) -> Result<Option<Rc<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse fun in infix position".to_string() } )
    }
}
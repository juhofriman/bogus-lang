use std::rc::Rc;
use crate::lexer::{Lexer, UnexpectedEOFError};
use crate::lexer::tokens::{Token, TokenKind};
use crate::ast::{Expression};
use crate::parser::p_o_plus::PlusParselet;
use crate::parser::p_v_identifier::IdentifierParselet;
use crate::parser::p_v_integer::IntegerParselet;
use crate::parser::p_o_minus::MinusParselet;
use crate::parser::p_s_let::LetParselet;
use crate::parser::p_d_semicolon::SemicolonParselet;
use crate::parser::p_o_multiplication::MultiplicationParselet;
use crate::parser::p_d_parens::{LeftParensParselet, RightParensParselet};
use crate::parser::p_s_fun::FunParselet;
use crate::parser::p_d_comma::CommaParselet;
use crate::parser::p_v_string::StringParselet;
use crate::parser::p_v_null::NullParselet;
use crate::parser::p_d_brace::{LeftBraceParselet, RightBraceParselet};
use crate::parser::p_s_return::ReturnParselet;
use crate::parser::p_s_assign::AssignParselet;
use crate::parser::p_v_boolean::BooleanParselet;
use crate::parser::p_o_equals::{EqualsParselet, EqualsOrNequals};
use crate::parser::p_s_if::IfParselet;

mod p_o_plus;
mod p_o_minus;
mod p_o_multiplication;
mod p_d_parens;
mod p_v_identifier;
mod p_v_integer;
mod p_v_string;
mod p_s_let;
mod p_d_semicolon;
mod p_v_null;
mod p_s_fun;
mod p_d_comma;
mod p_d_brace;
mod p_s_return;
mod p_s_assign;
mod p_v_boolean;
mod p_o_equals;
mod p_s_if;

pub struct ParseError {
    pub msg: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parsing Error: {}", self.msg)
    }
}

impl From<UnexpectedEOFError> for ParseError {
    fn from(err: UnexpectedEOFError) -> Self {
        ParseError {
            msg: format!("Lexer: {}", err)
        }
    }
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
}

impl Parser<'_> {
    pub fn new(lexer: &mut Lexer) -> Parser {
        Parser {
            lexer: lexer
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Rc<dyn Expression>>, ParseError> {
        let mut output: Vec<Rc<dyn Expression>> = vec![];

        while self.lexer.has_next() {
            output.push(parse_expression(0, &mut self.lexer)?)
        }

        Ok(output)
    }
}

fn get_parselet(token: &Token) -> Box<dyn Parselet> {
    return match &token.token_kind {
        TokenKind::Identifier(name) => Box::new(IdentifierParselet { value: name.clone() }),
        TokenKind::Integer(value) => Box::new(IntegerParselet { value: *value }),
        TokenKind::Str(value) => Box::new(StringParselet { value: value.clone() }),
        TokenKind::True => Box::new(BooleanParselet { value: true }),
        TokenKind::False => Box::new(BooleanParselet { value: false }),
        TokenKind::Plus => Box::new(PlusParselet {}),
        TokenKind::Minus => Box::new(MinusParselet {}),
        TokenKind::Equals => Box::new(EqualsParselet { equality_type: EqualsOrNequals::Equals }),
        TokenKind::NotEquals => Box::new(EqualsParselet { equality_type: EqualsOrNequals::Nequals }),
        TokenKind::Multiplication => Box::new(MultiplicationParselet {}),
        TokenKind::LeftParens => Box::new(LeftParensParselet {}),
        TokenKind::RightParens => Box::new(RightParensParselet {}),
        TokenKind::LeftBrace => Box::new(LeftBraceParselet {}),
        TokenKind::RightBrace => Box::new(RightBraceParselet {}),
        TokenKind::If => Box::new(IfParselet {}),
        TokenKind::Let => Box::new(LetParselet {}),
        TokenKind::Fun => Box::new(FunParselet {}),
        TokenKind::Return => Box::new(ReturnParselet {}),
        TokenKind::Semicolon => Box::new(SemicolonParselet {}),
        TokenKind::Comma => Box::new(CommaParselet {}),
        TokenKind::Null => Box::new(NullParselet {}),
        TokenKind::Assign => Box::new(AssignParselet {}),
        _ => { panic!("get_parselet() not implemented for {:?}", token.token_kind); }
    };
}

fn rbp_for(token: Option<&Token>) -> u32 {
    if let Some(token) = token {
        return match token.token_kind {
            TokenKind::Identifier(_) => 0,
            TokenKind::Assign => 1,
            TokenKind::Integer(_) => 0,
            TokenKind::Plus => 5,
            TokenKind::Minus => 5,
            TokenKind::Multiplication => 10,
            TokenKind::Equals => 30, // Dunno?
            TokenKind::NotEquals => 30, // Dunno?
            TokenKind::LeftParens => 50,
            TokenKind::RightParens => 1,
            TokenKind::LeftBrace => 0,
            TokenKind::RightBrace => 1,
            TokenKind::Let => 0,
            TokenKind::Fun => 0,
            TokenKind::Return => 0,
            TokenKind::Semicolon => 1,
            TokenKind::Comma => 0,
            _ => { panic!("rbp (right binding power) is not defined for {:?}", token); }
        };
    }
    0
}

pub trait Parselet {
    fn nud(&self, lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError>;
    fn led(&self, lexer: &mut Lexer, left: Rc<dyn Expression>) -> Result<Rc<dyn Expression>, ParseError>;
}

pub fn parse_expression(
    current_rbp: u32,
    lexer: &mut Lexer) -> Result<Rc<dyn Expression>, ParseError> {
    // Is it possible that nud or led returns None? Or is None always a parsing error?

    let mut left = get_parselet(lexer.next_or_err()?).nud(lexer)?;

    while rbp_for(lexer.peek()) > current_rbp {
        left = get_parselet(lexer.next_or_err()?).led(lexer, left)?;
    }

    Ok(left)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::{TypeMatcher};
    use crate::ast::scope::Scope;

    #[test]
    fn parse_simple_literals() {
        evaluate_and_assert("1", vec![
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("2", vec![
            TypeMatcher::Integer(&2),
        ]);
        evaluate_and_assert("1234566", vec![
            TypeMatcher::Integer(&1234566),
        ]);
        evaluate_and_assert("+1", vec![
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("-1", vec![
            TypeMatcher::Integer(&-1),
        ]);
        evaluate_and_assert("\"Hello world!\"", vec![
            TypeMatcher::String("Hello world!"),
        ]);
        evaluate_and_assert("null", vec![
            TypeMatcher::Null,
        ]);
        evaluate_and_assert("true", vec![
            TypeMatcher::Boolean(&true),
        ]);
        evaluate_and_assert("false", vec![
            TypeMatcher::Boolean(&false),
        ]);
    }

    #[test]
    fn test_simple_operators() {
        evaluate_and_assert("1 + 1", vec![
            TypeMatcher::Integer(&2),
        ]);
        evaluate_and_assert("1 - 1", vec![
            TypeMatcher::Integer(&0),
        ]);
        evaluate_and_assert("-1 + 1", vec![
            TypeMatcher::Integer(&0),
        ]);
        evaluate_and_assert("1 + -1", vec![
            TypeMatcher::Integer(&0),
        ]);
        evaluate_and_assert("1 * 1", vec![
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("-1 * 1", vec![
            TypeMatcher::Integer(&-1),
        ]);
        evaluate_and_assert("1 * -1", vec![
            TypeMatcher::Integer(&-1),
        ]);
        evaluate_and_assert("-1 * -1", vec![
            TypeMatcher::Integer(&1),
        ]);
    }

    #[test]
    fn test_parenthesis_expressions() {
        evaluate_and_assert("(1)", vec![
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("(1 + 1)", vec![
            TypeMatcher::Integer(&2),
        ]);
        evaluate_and_assert("1 + 1 * 2", vec![
            TypeMatcher::Integer(&3),
        ]);
        evaluate_and_assert("(1 + 1) * 2", vec![
            TypeMatcher::Integer(&4),
        ]);
        evaluate_and_assert("2* (1 + 1)", vec![
            TypeMatcher::Integer(&4),
        ]);
        evaluate_and_assert("1 == 1", vec![
            TypeMatcher::Boolean(&true),
        ]);
        evaluate_and_assert("1 != 1", vec![
            TypeMatcher::Boolean(&false),
        ]);
    }

    #[test]
    fn parse_let_statement() {
        evaluate_and_assert("let a = 1", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("let a = 1;", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("let a = 1; let b = 2;", vec![
            TypeMatcher::Void,
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("let a = 1; let b = 2", vec![
            TypeMatcher::Void,
            TypeMatcher::Void,
        ]);
    }

    #[test]
    fn parse_assign_statement() {
        evaluate_and_assert("let a = 1; a = 5; a", vec![
            TypeMatcher::Void,
            TypeMatcher::Void,
            TypeMatcher::Integer(&5)
        ]);
    }

    #[test]
    fn parse_fun_statement() {
        evaluate_and_assert("fun a() -> 1", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("fun a() -> 1;", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("fun a() -> 1; a();", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
        ]);

        evaluate_and_assert("fun a(b) -> b", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("fun a(b, c) -> b + c", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("fun a(b) -> b;", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("fun a(b, c) -> b + c;", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("fun a(b) -> b; a(1)", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("fun a(b) -> b; a(1);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("fun a(a, b) -> a + b", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("fun a(a, b) -> a + b;", vec![
            TypeMatcher::Void,
        ]);
        evaluate_and_assert("fun a(a, b) -> a + b; a(1, 2)", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&3),
        ]);
        evaluate_and_assert("fun a(a, b) -> a + b; a(1, 2);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&3),
        ]);
        evaluate_and_assert("fun a(a, b, c, d, e) -> a + b + c + d + e; a(1, 2, 3, 4, 5);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&15),
        ]);
        evaluate_and_assert("fun a(a) -> a; a(5 * 5);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&25),
        ]);
        evaluate_and_assert("fun a(a) -> a; a(5 * 5);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&25),
        ]);
        evaluate_and_assert("fun a(a, b) -> a + b; a(5 * 5, 5 + 5);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&35),
        ]);
    }

    #[test]
    fn function_accessing_outer_scope() {
        evaluate_and_assert("let a = 1; fun b() -> a; b();", vec![
            TypeMatcher::Void,
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("let a = 1; fun b(a) -> a; b(2);", vec![
            TypeMatcher::Void,
            TypeMatcher::Void,
            TypeMatcher::Integer(&2),
        ]);
    }

    #[test]
    fn anonymous_function() {
        evaluate_and_assert("let a = fun (a) -> a + 1; a(1)", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&2),
        ]);
        evaluate_and_assert("fun a(b) -> b(); a(fun () -> 1)", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("fun a() -> fun () -> 1; let b = a(); b();", vec![
            TypeMatcher::Void,
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("fun a() -> fun () -> 1; a()();", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("fun a() -> fun (a) -> a; a()(5);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&5),
        ]);
        evaluate_and_assert("fun a(a) -> a(); a(fun () -> 1);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
        ]);
        evaluate_and_assert("fun a(a) -> a(5); a(fun (b) -> b);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&5),
        ]);
        evaluate_and_assert("fun a() -> { let b = 1; let c = 2 return b + c; } a();", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&3)
        ]);
        // Early return test
        evaluate_and_assert("fun a() -> { return 1; let b = 1; let c = 2 return b + c; } a();", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&1)
        ]);
    }

    #[test]
    fn test_if_statements() {
        evaluate_and_assert("fun a(b) -> { if b { return 1; } return 2; }; a(true); a(false);", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&1),
            TypeMatcher::Integer(&2),
        ]);
    }

    #[test]
    fn parse_weird_things() {
        evaluate_and_assert("1 + 2; 2+3;", vec![
            TypeMatcher::Integer(&3),
            TypeMatcher::Integer(&5),
        ]);
        evaluate_and_assert("let a = 5; 1 + 2; let c = 2+3;", vec![
            TypeMatcher::Void,
            TypeMatcher::Integer(&3),
            TypeMatcher::Void,
        ]);
    }

    fn evaluate_and_assert(input: &str, expected: Vec<TypeMatcher>) {
        match Lexer::new(input) {
            Err(e) => panic!("Lexing failed: {}", e),
            Ok(mut lexer) => {
                let mut parser = Parser::new(&mut lexer);
                match parser.parse() {
                    Err(e) => panic!("Parse error: {}", e),
                    Ok(things) => {
                        assert_eq!(things.len(), expected.len());
                        let mut scope = Scope::new();
                        for (index, received_expression) in things.iter().enumerate() {
                            match received_expression.evaluate(&mut scope) {
                                Ok(res) =>
                                    assert_eq!(res.type_matcher(), *expected.get(index).unwrap(),
                                               "Right from input: {}", input),
                                Err(e) => panic!("Eval error: {:?} input: {}", e, input)
                            }
                        }
                    }
                }
            }
        }
    }
}
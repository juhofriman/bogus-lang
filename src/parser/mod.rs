use crate::lexer::{Lexer};
use crate::lexer::tokens::{Token, TokenKind};
use crate::ast::{Expression, Value};
use std::process::{exit, id};
use crate::ast::e_plus::PlusExpression;
use crate::ast::e_minus::MinusExpression;
use crate::ast::s_let::LetStatement;

pub struct ParseError {
    msg: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parsing Error: {}", self.msg)
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

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Expression>>, ParseError> {
        let mut output: Vec<Box<dyn Expression>> = vec![];

        while self.lexer.has_next() {
            let parselet = get_parselet(self.lexer.next());
            output.push(parselet.parse(&mut self.lexer)?)
        }

        Ok(output)
    }
}

fn get_parselet(token: Option<&Token>) -> Box<dyn Parselet> {
    if let Some(token) = token {
        return match &token.token_kind {
            TokenKind::Integer(value) => Box::new(IntegerParselet { value: *value }),
            TokenKind::Str(value) => Box::new(StringParselet { value: value.clone() }),
            TokenKind::Plus => Box::new(PlusParselet { }),
            TokenKind::Minus => Box::new(MinusParselet { }),
            _ => panic!("get_parselet() not implemented for {:?}", token.token_kind)
        };
    }
    panic!("get_parselet() called with None -> lexer consumed")
}

fn rbp_for(token: Option<&Token>) -> u32 {
    if let Some(token) = token {
        return match token.token_kind {
            TokenKind::Integer(_) => 1,
            TokenKind::Plus => 5,
            TokenKind::Minus => 5,
            _ => { panic!("rbp (right binding power) is not defined for {:?}", token); }
        }
    }
    0
}

pub trait Parselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError>;
    fn nud(&self, lexer: &mut Lexer) -> Option<Box<dyn Expression>>;
    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Option<Box<dyn Expression>>;
}

fn parse_expression(
    current_rbp: u32,
    parselet: &Parselet,
    lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
    let mut left = parselet.nud(lexer);

    while rbp_for(lexer.peek()) > current_rbp {
        left = get_parselet(lexer.next()).led(lexer, left.unwrap());
    }

    match left {
        Some(expr) => Ok(expr),
        None => Err(ParseError { msg: "left was None".to_string() })
    }
}


pub struct IntegerParselet {
    value: i32,
}

impl Parselet for IntegerParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, self, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Option<Box<dyn Expression>> {
        Some(Box::new(Value::Integer(self.value)))
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        None
    }
}

pub struct StringParselet {
    pub value: String,
}

impl Parselet for StringParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, self, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Option<Box<dyn Expression>> {
        Some(Box::new(Value::String(self.value.clone())))
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let right = parse_expression(0, &*get_parselet(lexer.next()), lexer);
        match right {
            Ok(right) => {
                Some(Box::new(PlusExpression::new(
                    left,
                    right,
                )))
            },
            Err(r) => {
                println!("Err: {}", r);
                None
            }
        }
    }
}

pub struct PlusParselet {}

impl Parselet for PlusParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        todo!()
    }

    fn nud(&self, lexer: &mut Lexer) -> Option<Box<dyn Expression>> {
        todo!()
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let right = parse_expression(0, &*get_parselet(lexer.next()), lexer);
        match right {
            Ok(right) => {
                Some(Box::new(PlusExpression::new(
                    left,
                    right,
                )))
            },
            Err(r) => {
                println!("Err: {}", r);
                None
            }
        }
    }
}

pub struct MinusParselet {}

impl Parselet for MinusParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        todo!()
    }

    fn nud(&self, lexer: &mut Lexer) -> Option<Box<dyn Expression>> {
        todo!()
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Option<Box<dyn Expression>> {
        let right = parse_expression(0, &*get_parselet(lexer.next()), lexer);
        match right {
            Ok(right) => {
                Some(Box::new(MinusExpression::new(
                    left,
                    right,
                )))
            },
            Err(r) => {
                println!("Err: {}", r);
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::ast::{Expression, Value};
    use crate::ast::scope::Scope;

    #[test]
    fn parse_simple_literals() {
        parses_to("1".to_string(), vec![
            Box::new(Value::Integer(1)),
        ]);
        parses_to("2".to_string(), vec![
            Box::new(Value::Integer(2)),
        ]);
        parses_to("\"Hello world!\"".to_string(), vec![
            Box::new(Value::String("Hello world!".to_string())),
        ]);
    }

    fn parses_to(input: String, expected_output: Vec<Box<dyn Expression>>) {
        match Lexer::new(input.as_str()) {
            Ok(mut lexer) => unsafe {
                let mut parser = Parser::new(&mut lexer);
                let result = parser.parse().unwrap_or_else(|_| panic!("nonono"));
                assert_eq!(expected_output.len(), result.len(),
                           "Parsed output did not yield expected count of expressions");
                for (index, received_expression) in result.iter().enumerate() {
                    // Looks like only reasonable way to assert parse output is to evaluate clauses...
                    let mut scope = Scope::new();
                    assert_eq!(
                        expected_output.get_unchecked(index)
                            .evaluate(&mut scope)
                            .unwrap_or_else(|_| panic!("nonono")),
                        received_expression
                            .evaluate(&mut scope)
                            .unwrap_or_else(|_| panic!("nonono"))
                    );
                }
            },
            Err(lexing_err) => panic!("Lexing failed: {}", lexing_err)
        }
    }
}
use crate::lexer::{Lexer};
use crate::lexer::tokens::{Token, TokenKind};
use crate::ast::{Expression, Value};
use crate::ast::e_plus::PlusExpression;
use crate::ast::e_minus::{MinusExpression, PrefixMinusExpression};
use crate::ast::e_multiplication::MultiplicationExpression;

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
            output.push(parse_expression(0, &mut self.lexer)?)
        }

        Ok(output)
    }
}

fn get_parselet(token: Option<&Token>) -> Result<Box<dyn Parselet>, ParseError> {
    if let Some(token) = token {
        return match &token.token_kind {
            TokenKind::Integer(value) => Ok(Box::new(IntegerParselet { value: *value })),
            TokenKind::Str(value) => Ok(Box::new(StringParselet { value: value.clone() })),
            TokenKind::Plus => Ok(Box::new(PlusParselet {})),
            TokenKind::Minus => Ok(Box::new(MinusParselet {})),
            TokenKind::Multiplication => Ok(Box::new(MultiplicationParselet {})),
            TokenKind::LeftParens => Ok(Box::new(LeftParensParselet {})),
            TokenKind::RightParens => Ok(Box::new(RightParensParselet {})),
            _ => { panic!("get_parselet() not implemented for {:?}", token.token_kind); }
        };
    }
    Err(ParseError { msg: "Expecting more input".to_string() })
}

fn rbp_for(token: Option<&Token>) -> u32 {
    if let Some(token) = token {
        return match token.token_kind {
            TokenKind::Integer(_) => 2,
            TokenKind::Plus => 5,
            TokenKind::Minus => 5,
            TokenKind::Multiplication => 10,
            TokenKind::LeftParens => 50,
            TokenKind::RightParens => 1,
            _ => { panic!("rbp (right binding power) is not defined for {:?}", token); }
        };
    }
    0
}

pub trait Parselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError>;
    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError>;
    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError>;
}

fn parse_expression(
    current_rbp: u32,
    lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
    // Is it possible that nud or led returns None? Or is None always a parsing error?
    let mut left = get_parselet(lexer.next())?.nud(lexer)?;

    while rbp_for(lexer.peek()) > current_rbp {
        left = get_parselet(lexer.next())?.led(lexer, left.unwrap())?;

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
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        Ok(Some(Box::new(Value::Integer(self.value))))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        Ok(None)
    }
}

pub struct StringParselet {
    pub value: String,
}

impl Parselet for StringParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        Ok(Some(Box::new(Value::String(self.value.clone()))))
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let right = parse_expression(0,  lexer);
        match right {
            Ok(right) => {
                Ok(Some(Box::new(PlusExpression::new(
                    left,
                    right,
                ))))
            }
            Err(r) => {
                println!("Err: {}", r);
                Ok(None)
            }
        }
    }
}

pub struct PlusParselet {}

impl Parselet for PlusParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let expression = parse_expression(
            0,
            lexer)?;
        // This does not create extra expression. Side effect is that +"foo" -> "foo".
        Ok(Some(expression))
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let right = parse_expression(
            5,
            lexer)?;

        Ok(Some(Box::new(PlusExpression::new(
            left,
            right,
        ))))
    }
}

pub struct MinusParselet {}

impl Parselet for MinusParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let expression = parse_expression(
            5,
            lexer)?;
        Ok(Some(Box::new(PrefixMinusExpression::new(expression))))
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let right = parse_expression(
            5,
            lexer)?;

        Ok(Some(Box::new(MinusExpression::new(
            left,
            right,
        ))))
    }
}

pub struct MultiplicationParselet {}

impl Parselet for MultiplicationParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse * in prefix position".to_string() } )
    }

    fn led(&self, lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let right = parse_expression(
            10,
            lexer)?;

        Ok(Some(Box::new(MultiplicationExpression::new(
            left,
            right,
        ))))
    }
}

pub struct LeftParensParselet {}

impl Parselet for LeftParensParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        let expr = parse_expression(
            0,
            lexer)?;
        Ok(Some(expr))
    }

    fn led(&self, _lexer: &mut Lexer, _left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse ( in NUD position".to_string() } )
    }
}

pub struct RightParensParselet {}

impl Parselet for RightParensParselet {
    fn parse(&self, lexer: &mut Lexer) -> Result<Box<dyn Expression>, ParseError> {
        parse_expression(0, lexer)
    }

    fn nud(&self, _lexer: &mut Lexer) -> Result<Option<Box<dyn Expression>>, ParseError> {
        Err( ParseError { msg: "Can't parse ) in prefix position".to_string() } )
    }

    fn led(&self, _lexer: &mut Lexer, left: Box<dyn Expression>) -> Result<Option<Box<dyn Expression>>, ParseError> {
        Ok(Some(left))
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
        parses_to("-1".to_string(), vec![
            Box::new(Value::Integer(-1)),
        ]);
        parses_to("+1".to_string(), vec![
            Box::new(Value::Integer(1)),
        ]);
        parses_to("2".to_string(), vec![
            Box::new(Value::Integer(2)),
        ]);
        parses_to("\"Hello world!\"".to_string(), vec![
            Box::new(Value::String("Hello world!".to_string())),
        ]);
    }

    #[test]
    fn parse_simple_expressions() {
        parses_to("1 + 2".to_string(), vec![
            Box::new(Value::Integer(3)),
        ]);
        parses_to("1 - 2".to_string(), vec![
            Box::new(Value::Integer(-1)),
        ]);
        parses_to("1 - -2".to_string(), vec![
            Box::new(Value::Integer(3)),
        ]);
        parses_to("-1 - 2".to_string(), vec![
            Box::new(Value::Integer(-3)),
        ]);
        parses_to("1 * 2".to_string(), vec![
            Box::new(Value::Integer(2)),
        ]);
        parses_to("-1 * 2".to_string(), vec![
            Box::new(Value::Integer(-2)),
        ]);
        parses_to("1 * -2".to_string(), vec![
            Box::new(Value::Integer(-2)),
        ]);
        parses_to("-1 * -2".to_string(), vec![
            Box::new(Value::Integer(2)),
        ]);
        parses_to("2 * 2 + 1".to_string(), vec![
            Box::new(Value::Integer(5)),
        ]);
        parses_to("1 + 2 * 2".to_string(), vec![
            Box::new(Value::Integer(5)),
        ]);
        parses_to("\"Hello world!\" + 123".to_string(), vec![
            Box::new(Value::String("Hello world!123".to_string())),
        ]);
        parses_to("123 + \"Hello world!\"".to_string(), vec![
            Box::new(Value::String("123Hello world!".to_string())),
        ]);

        parses_to("(1 + 2)".to_string(), vec![
            Box::new(Value::Integer(3)),
        ]);
        parses_to("2 * (1 + 2)".to_string(), vec![
            Box::new(Value::Integer(6)),
        ]);
        parses_to("(1 + 2) * 2".to_string(), vec![
            Box::new(Value::Integer(6)),
        ]);
        parses_to("(1 + 2) * (( 2 + 4 ) * 2)".to_string(), vec![
            Box::new(Value::Integer(36)),
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
                            .unwrap_or_else(|_| panic!("nonono")),
                        "Input was: {}",
                        input
                    );
                }
            },
            Err(lexing_err) => panic!("Lexing failed: {}", lexing_err)
        }
    }
}
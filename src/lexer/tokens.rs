use std::fmt::{Display, Formatter};
use crate::parser::ParseError;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub source_ref: SourceRef,
    pub token_kind: TokenKind,
}

impl Token {

    pub fn is_identifier(&self) -> Result<String, ParseError> {
        match &self.token_kind {
            TokenKind::Identifier(name) => Ok(name.to_string()),
            _ => Err( ParseError {
                msg: format!("Expecting Identifier but {} found", self)
            })
        }
    }

    pub fn is_assing(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::Assign => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting Identifier but {} found", self)
            })
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_kind, self.source_ref)
    }
}

/// Creates a token with given kind and source references
pub fn create_token(token_kind: TokenKind, line: u32, column: u32) -> Token {
    Token {
        token_kind,
        source_ref: SourceRef {
            line,
            column,
        },
    }
}

#[derive(Debug, PartialEq)]
pub struct SourceRef {
    pub line: u32,
    pub column: u32,
}

impl Display for SourceRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}:{}]", self.line, self.column)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Reserved words
    Let,
    Const,
    Fun,

    // Operators
    Assign,
    Equals,
    Minus,
    Plus,
    Multiplication,
    Division,
    Arrow,

    // Delimiters
    Comma,
    Dot,
    Semicolon,
    LeftParens,
    RightParens,

    // Identifier
    Identifier(String),

    // Literals
    Integer(i32),
    Float(f32),
    Str(String),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn token_usage() {
        let tokens = vec![
            create_token(TokenKind::Let, 0, 0),
            create_token(TokenKind::Identifier("seppo".to_string()), 0, 3),
        ];
        assert_eq!(tokens.len(), 2)
    }
}
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

    pub fn is_left_parens(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::LeftParens => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting LeftParens but {} found", self)
            })
        }
    }

    pub fn is_right_parens(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::RightParens => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting RightParens but {} found", self)
            })
        }
    }

    pub fn is_arrow(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::Arrow => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting -> but {} found", self)
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
pub enum TokenType {
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
    Identifier,

    // Literals
    Integer,
    Float,
    Str,
    Null,
}

impl TokenType {
    pub fn token_is(&self, token: &Token) -> bool {
        match &token.token_kind {
            // Reserved words
            TokenKind::Let => self == &TokenType::Let,
            TokenKind::Const => self == &TokenType::Const,
            TokenKind::Fun => self == &TokenType::Fun,

            // Operators
            TokenKind::Assign => self == &TokenType::Assign,
            TokenKind::Equals => self == &TokenType::Equals,
            TokenKind::Minus => self == &TokenType::Minus,
            TokenKind::Plus => self == &TokenType::Plus,
            TokenKind::Multiplication => self == &TokenType::Multiplication,
            TokenKind::Division => self == &TokenType::Division,
            TokenKind::Arrow => self == &TokenType::Arrow,

            // Delimiters
            TokenKind::Comma => self == &TokenType::Comma,
            TokenKind::Dot => self == &TokenType::Dot,
            TokenKind::Semicolon => self == &TokenType::Semicolon,
            TokenKind::LeftParens => self == &TokenType::LeftParens,
            TokenKind::RightParens => self == &TokenType::RightParens,

            // Identifier
            TokenKind::Identifier(_) => self == &TokenType::Identifier,

            // Literals
            TokenKind::Integer(_) => self == &TokenType::Integer,
            TokenKind::Float(_)=> self == &TokenType::Float,
            TokenKind::Str(_) => self == &TokenType::Str,
            TokenKind::Null => self == &TokenType::Null,
        }
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
    Null,
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
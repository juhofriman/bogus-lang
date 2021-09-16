use std::fmt::{Display, Formatter};
use crate::parser::ParseError;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub source_ref: SourceRef,
    pub token_kind: TokenKind,
}

impl Token {

    pub fn new(token_kind: TokenKind, line: u32, column: u32) -> Token {
        Token {
            token_kind,
            source_ref: SourceRef {
                line,
                column,
            },
        }
    }

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
                msg: format!("Expecting = but {} found", self)
            })
        }
    }

    pub fn is_left_parens(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::LeftParens => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting ( but {} found", self)
            })
        }
    }

    pub fn is_right_parens(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::RightParens => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting ) but {} found", self)
            })
        }
    }

    pub fn is_right_brace(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::RightBrace => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting ) but {} found", self)
            })
        }
    }

    pub fn is_semicolon(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::Semicolon => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting -> but {} found", self)
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

    pub fn is_comma(&self) -> Result<(), ParseError> {
        match &self.token_kind {
            TokenKind::Comma => Ok(()),
            _ => Err( ParseError {
                msg: format!("Expecting , but {} found", self)
            })
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.token_kind, self.source_ref)
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
    Return,

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
    LeftBrace,
    RightBrace,

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
        assert!(Token::new(TokenKind::Let, 0, 0).is_identifier().is_err());
        assert!(Token::new(
            TokenKind::Identifier("foo".to_string()), 0, 0)
            .is_identifier().is_ok()
        );
    }
}
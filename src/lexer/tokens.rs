use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub source_ref: SourceRef,
    pub token_kind: TokenKind,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} [{}:{}]", self.token_kind, self.source_ref.line, self.source_ref.column)
    }
}

/// Creates a token with given kind and source references
pub fn create_token(token_kind: TokenKind, line: u32, column: u32, offset: usize) -> Token {
    Token {
        token_kind,
        source_ref: SourceRef {
            line,
            column: column - (offset as u32),
        },
    }
}

#[derive(Debug, PartialEq)]
pub struct SourceRef {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Reserved words
    Let,
    Fun,

    // Operators
    Equals,
    Minus,
    Plus,
    Arrow,

    // Delimiters
    Semicolon,
    LeftParens,
    RightParens,

    // Identifier
    Identifier(String),

    // Literals
    Integer(i32),
    Str(String),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn token_usage() {
        let tokens = vec![
            create_token(TokenKind::Let, 0, 0, 0),
            create_token(TokenKind::Identifier("seppo".to_string()), 0, 3, 0),
        ];
        assert_eq!(tokens.len(), 2)
    }
}
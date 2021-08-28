
type Literal = String;

#[derive(Debug, PartialEq)]
pub struct SourceRef {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_kind: TokenKind,
    pub source_ref: SourceRef,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Let,
    Fun,

    Equals,
    Semicolon,
    LeftParens,
    RightParens,

    Identifier(Literal),

    Integer(i32),
    Str(String),
}

fn source_r(line: u32, column: u32, offset: usize) -> SourceRef {
    SourceRef {
        line,
        column: column - (offset as u32)
    }
}

pub fn token(token_kind: TokenKind, line: u32, column: u32, offset: usize) -> Token {
    Token {
        token_kind,
        source_ref: source_r(line, column, offset),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn token_usage() {
        let tokens = vec![
            Token {
                token_kind: TokenKind::Let,
                source_ref: source_r(0, 0, 0),
            },
            Token {
                token_kind: TokenKind::Identifier("seppo".to_string()),
                source_ref: source_r(1, 3, 2),
            }
        ];
        assert_eq!(tokens.len(), 2)
    }
}
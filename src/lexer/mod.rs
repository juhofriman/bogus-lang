use crate::lexer::tokens::{Token, TokenKind, token};

mod tokens;

pub struct Lexer {
    tokens: Vec<Token>,
    pointer: usize
}

impl Lexer {

    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pointer);
        self.pointer += 1;
        token
    }

    pub fn has_next(&self) -> bool {
        if self.tokens.is_empty() {
            return false
        }
        self.pointer <= self.tokens.len() - 1
    }
}

fn buffer_2_token(buffer: &mut LexBuffer, source_marker: &SourceMarker) -> Option<Token> {
    if buffer.mode == LexingState::Integer {
        buffer.mode = LexingState::Normal;
        return Some(token(
            TokenKind::Integer(buffer.buffer.parse().unwrap()),
            source_marker.line,
            source_marker.column,
            buffer.buffer.len()))
    }
    if buffer.mode == LexingState::String {
        buffer.mode = LexingState::Normal;
        return Some(token(
            TokenKind::Str(buffer.buffer.to_string()),
            source_marker.line,
            source_marker.column,
            buffer.buffer.len() + 1))
    }
    match buffer.buffer.as_str() {
        "let" => Some(token(
            TokenKind::Let,
            source_marker.line,
            source_marker.column,
            buffer.buffer.len())),
        "=" => Some(token(
            TokenKind::Equals,
            source_marker.line,
            source_marker.column,
            buffer.buffer.len())),
        ";" => Some(token(
            TokenKind::Semicolon,
            source_marker.line,
            source_marker.column,
            buffer.buffer.len())),
        "(" => Some(token(
            TokenKind::LeftParens,
            source_marker.line,
            source_marker.column,
            buffer.buffer.len())),
        ")" => Some(token(
            TokenKind::RightParens,
            source_marker.line,
            source_marker.column,
            buffer.buffer.len())),
        _ if buffer.buffer.is_empty() => {
            None
        },
        _ => Some(token(
            TokenKind::Identifier(buffer.buffer.to_string()),
            source_marker.line,
            source_marker.column,
            buffer.buffer.len())),
    }
}

fn is_delimiting(c: &char) -> bool {
    match c {
        ';' => true,
        '(' => true,
        ')' => true,
        _ => false
    }
}

fn preflow(buffer: &mut LexBuffer, c: char, source_marker: &mut SourceMarker) -> (Option<Token>, Option<Token>) {
    match c {
        _ if c.is_whitespace() && buffer.mode != LexingState::String  => {
            let token = buffer_2_token(buffer, source_marker);
            buffer.buffer.clear();
            (token, None)
        },
        _ if is_delimiting(&c) && buffer.mode != LexingState::String => {
            let token = buffer_2_token(buffer, source_marker);
            buffer.buffer.clear();
            buffer.buffer.push(c);
            source_marker.column += 1;
            let delimiting_token = buffer_2_token(buffer, source_marker);
            buffer.buffer.clear();
            (token, delimiting_token)
        },
        _ if buffer.buffer.is_empty() && c.is_digit(10) => {
            buffer.mode = LexingState::Integer;
            buffer.buffer.push(c);
            (None, None)
        },
        _ if buffer.buffer.is_empty() && c == '"' => {
            buffer.mode = LexingState::String;
            (None, None)
        },
        _ if buffer.mode == LexingState::String && c == '"' => {
            let token = buffer_2_token(buffer, source_marker);
            buffer.buffer.clear();
            (token, None)
        },
        _ if buffer.mode == LexingState::Integer && !c.is_digit(10) => {
            let token = buffer_2_token(buffer, source_marker);
            buffer.buffer.clear();
            buffer.buffer.push(c);
            (token, None)
        }
        _ => {
            buffer.buffer.push(c);
            (None, None)
        }
    }
}

#[derive(PartialEq)]
enum LexingState {
    Normal,
    Integer,
    String,
}

struct LexBuffer {
    buffer: String,
    mode: LexingState,
}

struct SourceMarker {
    line: u32,
    column: u32,
}

fn lex_source(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut buffer = LexBuffer {
        buffer: String::new(),
        mode: LexingState::Normal,
    };
    let mut src_mark = SourceMarker {
        line: 1,
        column: 0,
    };
    for c in source.chars() {
        // if let Some(token) = preflow(&mut buffer, c, &src_mark) {
        //     tokens.push(token);
        // }
        match preflow(&mut buffer, c, &mut src_mark) {
            (None, None) => (),
            (Some(token), None) => {
                tokens.push(token);
            },
            (None, Some(token)) => {
                tokens.push(token);
            },
            (Some(token), Some(another_token)) => {
                tokens.push(token);
                tokens.push(another_token);
            },
            all => panic!("Illegal token push {:?}", all)
        }
        if c == '\n' {
            src_mark.line += 1;
            src_mark.column = 0;
        } else {
            src_mark.column += 1;
        }
    }
    if let Some(token) = buffer_2_token(&mut buffer, &src_mark) {
        tokens.push(token);
    }

    tokens
}

pub fn create_lexer(source: &str) -> Lexer {
    Lexer {
        pointer: 0,
        tokens: lex_source(source)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::lexer::tokens::TokenKind::{Let, Identifier, Equals, Integer, Str, Semicolon, RightParens, LeftParens};

    #[test]
    fn test_lexer_single_tokens() {
        token_lexes_to("let", Let);
        token_lexes_to("=", Equals);
        token_lexes_to("foo",Identifier("foo".to_string()));
        for nmbr in 0..100 {
            token_lexes_to(&nmbr.to_string(),Integer(nmbr));
        }
        token_lexes_to("\"\"", Str("".to_string()));
        token_lexes_to("\"foo\"", Str("foo".to_string()));
        token_lexes_to(";", Semicolon);
        token_lexes_to("(", LeftParens);
        token_lexes_to(")", RightParens);
    }

    #[test]
    fn test_lexer_reasonable_statemens() {
        with_input_lexes_to("let a = 1;", vec![
            dummy_token(Let),
            dummy_token(Identifier("a".to_string())),
            dummy_token(Equals),
            dummy_token(Integer(1)),
            dummy_token(Semicolon),
        ]);
        with_input_lexes_to("call(1);", vec![
            dummy_token(Identifier("call".to_string())),
            dummy_token(LeftParens),
            dummy_token(Integer(1)),
            dummy_token(RightParens),
            dummy_token(Semicolon),
        ]);
        with_input_lexes_to("call(a);", vec![
            dummy_token(Identifier("call".to_string())),
            dummy_token(LeftParens),
            dummy_token(Identifier("a".to_string())),
            dummy_token(RightParens),
            dummy_token(Semicolon),
        ]);
    }

    #[test]
    fn test_lexing_with_line_and_column_references() {
        with_input_lexes_to_assert_columns(
            "let foo = 1;\nlet bar = \"bar value with whitespace\";",
            vec![
                token_at(Let, 1, 0),
                token_at(Identifier("foo".to_string()), 1, 4),
                token_at(Equals, 1, 8),
                token_at(Integer(1), 1, 10),
                token_at(Semicolon, 1, 11),

                token_at(Let, 2, 0),
                token_at(Identifier("bar".to_string()), 2, 4),
                token_at(Equals, 2, 8),
                token_at(Str("bar value with whitespace".to_string()), 2, 10),
                token_at(Semicolon, 2, 37),
            ]
        )

        // Problems
        // ;;;;; makes invalid column reference
    }

    #[test]
    fn test_interesting_corner_cases() {
        with_input_lexes_to(
            "\"let a = 1\"",
            vec![
                dummy_token(Str("let a = 1".to_string())),
            ]
        );
        with_input_lexes_to(
            "let\n a\n = \n1;",
            vec![
                dummy_token(Let),
                dummy_token(Identifier("a".to_string())),
                dummy_token(Equals),
                dummy_token(Integer(1)),
                dummy_token(Semicolon),
            ]
        );
        with_input_lexes_to(
            "let foo;let bar;",
            vec![
                dummy_token(Let),
                dummy_token(Identifier("foo".to_string())),
                dummy_token(Semicolon),
                dummy_token(Let),
                dummy_token(Identifier("bar".to_string())),
                dummy_token(Semicolon),
            ]
        );
        with_input_lexes_to(
            "call(1);ball(2);",
            vec![
                dummy_token(Identifier("call".to_string())),
                dummy_token(LeftParens),
                dummy_token(Integer(1)),
                dummy_token(RightParens),
                dummy_token(Semicolon),
                dummy_token(Identifier("ball".to_string())),
                dummy_token(LeftParens),
                dummy_token(Integer(2)),
                dummy_token(RightParens),
                dummy_token(Semicolon),
            ]
        );
    }

    fn token_lexes_to(input: &str, expected_kind: TokenKind) {
        with_input_lexes_to(input, vec![dummy_token(expected_kind)]);
    }

    fn with_input_lexes_to(input: &str, expected_tokens: Vec<Token>) {
        do_lexing_assertion(input, expected_tokens, false)
    }

    fn with_input_lexes_to_assert_columns(input: &str, expected_tokens: Vec<Token>) {
        do_lexing_assertion(input, expected_tokens, true)
    }

    /// Asserts given input lexes to expected tokens
    /// Note: this also checks lines and columns
    fn do_lexing_assertion(input: &str, expected_tokens: Vec<Token>, assert_refs: bool) {
        let mut lexer = create_lexer(input);
        for expected_token in expected_tokens {
            let (received_line, received_column) = advance_expect(&mut lexer, &expected_token.token_kind);
            if assert_refs {
                assert_eq!(
                    expected_token.source_ref.line,
                    received_line,
                    "Wrong line ref for token {:?} in source `{}`",
                    expected_token.token_kind,
                    input
                );
                assert_eq!(
                    expected_token.source_ref.column,
                    received_column,
                    "Wrong column ref for token {:?} in source `{}`",
                    expected_token.token_kind,
                    input
                );
            }
        }
        assert!(!lexer.has_next(), "lexer had more tokens after expected!")
    }


    // Advances lexer and asserts token kind
    // returns (line, column) of received token
    // panics if lexer is consumed completely
    fn advance_expect(lexer: &mut Lexer, kind: &TokenKind) -> (u32, u32) {
        assert!(lexer.has_next());
        match lexer.next() {
            None => panic!("Expecting token but None returned! (has_next() returned true before!)"),
            Some(token) => {
                assert_eq!(token.token_kind, *kind);
                (token.source_ref.line, token.source_ref.column)
            }
        }
    }

    /// Creates a token that can be used as a reference in tests
    /// It adds offset to 0, which means that expected column is the one given
    fn token_at(kind: TokenKind, line: u32, column: u32) -> Token {
        token(kind, line, column, 0)
    }

    /// Creates dummy token, with source references zeroed
    /// Cannot be used for assertions with line and column references but
    /// is much more fast to write
    fn dummy_token(kind: TokenKind) -> Token {
        token(kind, 0, 0, 0)
    }

}

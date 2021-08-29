use crate::lexer::tokens::{Token, TokenKind, create_token, SourceRef};
use core::fmt;

mod tokens;

pub struct Lexer {
    tokens: Vec<Token>,
    pointer: usize,
}

impl Lexer {
    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pointer);
        self.pointer += 1;
        token
    }

    pub fn has_next(&self) -> bool {
        if self.tokens.is_empty() {
            return false;
        }
        self.pointer <= self.tokens.len() - 1
    }
}

pub struct LexingError {
    msg: String,
    location: SourceRef,
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexing Error: {} @ {}", self.msg, self.location)
    }
}

/// Creates consumable lexer for given input
pub fn create_lexer(source: &str) -> Result<Lexer, LexingError> {
    let tokens = lex_source(source)?;
    Ok(Lexer {
        pointer: 0,
        tokens,
    })
}

fn lex_source(source: &str) -> Result<Vec<Token>, LexingError> {
    let mut tokens: Vec<Token> = vec![];
    let mut buffer = LexBuffer {
        mode: LexingState::Normal,
        buffer: String::new(),
        current_line: 1,
        current_column: 0,
        token_column_marker: 0,
        string_escape_flag: false,
    };
    let mut character_iter = source.chars().peekable();
    while let Some(char) = character_iter.next() {
        if let Some(token) = buffer.push_char(char, character_iter.peek())? {
            tokens.push(token);
        }
    }
    Ok(tokens)
}

#[derive(PartialEq)]
enum LexingState {
    Normal,
    Integer,
    String,
    LineComment,
}

/// LexBuffer is used for lexing the given input statefully
struct LexBuffer {
    buffer: String,
    mode: LexingState,
    current_line: u32,
    current_column: u32,
    token_column_marker: u32,
    string_escape_flag: bool,
}

impl LexBuffer {
    fn pop_buffer(&mut self, kind: TokenKind) -> Token {
        self.buffer.clear();
        let new_token = create_token(kind,
                                     self.current_line,
                                     self.token_column_marker);
        self.token_column_marker = self.current_column;
        new_token
    }

    fn pop_buffer_cond(
        &mut self,
        kind: TokenKind,
        peek: Option<&char>,
        cond: fn(&char) -> bool) -> Result<Option<Token>, LexingError> {
        match peek {
            None => Ok(Some(self.pop_buffer(kind))),
            Some(peek) => {
                if cond(peek) {
                    return Ok(Some(self.pop_buffer(kind)));
                }
                Ok(None)
            }
        }
    }

    fn push_char(&mut self, current_char: char, peek: Option<&char>) -> Result<Option<Token>, LexingError> {
        self.current_column += 1;
        if current_char == '\n' {
            if self.mode == LexingState::LineComment {
                self.mode = LexingState::Normal
            }
            self.current_line += 1;
            self.current_column = 0;
            self.token_column_marker = 0;
            return Ok(None);
        }

        match current_char {
            _ if self.mode == LexingState::LineComment => (),
            _ if self.string_escape_flag == true => {
                // This should check that next is actually escapable
                self.buffer.push(current_char);
                self.string_escape_flag = false;
                return Ok(None);
            }
            _ if current_char == '/' && *peek.unwrap_or(&' ') == '/' && self.mode == LexingState::Normal => {
                self.mode = LexingState::LineComment;
                return Ok(None);
            }
            _ if self.mode != LexingState::String && current_char.is_whitespace() => {
                self.token_column_marker += 1;
                return Ok(None);
            }
            _ if self.buffer.is_empty() && current_char.is_digit(10) => {
                self.mode = LexingState::Integer;
                self.buffer.push(current_char);
            }
            _ if self.buffer.is_empty() && self.mode != LexingState::String && current_char == '"' => {
                self.mode = LexingState::String;
                return Ok(None);
            }
            _ if self.mode == LexingState::String && current_char == '\\' => {
                self.string_escape_flag = true;
                return Ok(None);
            }
            _ if self.mode == LexingState::String && current_char == '"' => {}
            _ if self.mode == LexingState::String && peek.is_none() => {
                return Err(LexingError {
                    msg: "string is not terminated".to_string(),
                    location: SourceRef {
                        line: self.current_line,
                        column: self.token_column_marker,
                    }
                });
            }
            _ => {
                self.buffer.push(current_char);
            }
        }

        match self.mode {
            LexingState::LineComment => {
                Ok(None)
            }
            LexingState::Integer => {
                let value_to_be = self.buffer.parse();
                if value_to_be.is_err() {
                    return Err(LexingError {
                        msg: "identifier can't start with digit".to_string(),
                        location: SourceRef {
                            line: self.current_line,
                            column: self.token_column_marker,
                        }
                    })
                }
                let value_to_be = value_to_be.unwrap();
                let ready = self.pop_buffer_cond(TokenKind::Integer(value_to_be),
                                                 peek,
                                                 |peek| is_delimiting(peek))?;
                if ready.is_some() {
                    self.mode = LexingState::Normal;
                }
                Ok(ready)
            }
            LexingState::String => {
                if current_char == '"' {
                    self.mode = LexingState::Normal;
                    return Ok(Some(self.pop_buffer(TokenKind::Str(self.buffer.to_string()))));
                }
                Ok(None)
            }
            LexingState::Normal => {
                match self.buffer.as_str() {
                    "let" => self.pop_buffer_cond(
                        TokenKind::Let,
                        peek,
                        |peek| is_delimiting(peek)),
                    "fun" => self.pop_buffer_cond(
                        TokenKind::Fun,
                        peek,
                        |peek| is_delimiting(peek)),
                    "(" => Ok(Some(self.pop_buffer(TokenKind::LeftParens))),
                    ")" => Ok(Some(self.pop_buffer(TokenKind::RightParens))),
                    "," => Ok(Some(self.pop_buffer(TokenKind::Comma))),
                    "-" => self.pop_buffer_cond(
                        TokenKind::Minus,
                        peek,
                        |peek| *peek != '>'),
                    "+" => Ok(Some(self.pop_buffer(TokenKind::Plus))),
                    "/" => Ok(Some(self.pop_buffer(TokenKind::Division))),
                    "->" => Ok(Some(self.pop_buffer(TokenKind::Arrow))),
                    ";" => Ok(Some(self.pop_buffer(TokenKind::Semicolon))),
                    "=" => self.pop_buffer_cond(
                        TokenKind::Assign,
                        peek,
                        |peek| *peek != '='),
                    "==" => Ok(Some(self.pop_buffer(TokenKind::Equals))),
                    // KLUDGE: this will actually MAKE a token even if it's not used
                    // Should be refactored to returning token from peek fn
                    _ => self.pop_buffer_cond(
                        TokenKind::Identifier(self.buffer.to_string()),
                        peek,
                        |peek| is_delimiting(peek),
                    )
                }
            }
        }
    }
}


fn is_delimiting(c: &char) -> bool {
    match c {
        _ if c.is_whitespace() => true,
        ';' => true,
        '(' => true,
        ')' => true,
        '+' => true,
        '-' => true,
        '/' => true,
        ',' => true,
        '=' => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::lexer::tokens::TokenKind::{Let, Identifier, Assign, Integer, Str, Semicolon, RightParens, LeftParens, Arrow, Minus, Plus, Fun, Comma, Division, Equals};

    // Internal implementation test helpers

    #[test]
    fn test_lex_buffer() {
        with_fresh_buffer(|buffer| {
            assert!(buffer.push_char('l', Some(&'e')).unwrap_or_else(|_| panic!("Unexpected Err")).is_none());
            assert!(buffer.push_char('e', Some(&'t')).unwrap_or_else(|_| panic!("Unexpected Err")).is_none());
            assert!(buffer.push_char('t', None).unwrap_or_else(|_| panic!("Unexpected Err")).is_some());
        });

        with_fresh_buffer(|buffer| {
            assert!(buffer.push_char('l', Some(&'e')).unwrap_or_else(|_| panic!("Unexpected Err")).is_none());
            assert!(buffer.push_char('e', Some(&'t')).unwrap_or_else(|_| panic!("Unexpected Err")).is_none());
            assert!(buffer.push_char('t', Some(&'t')).unwrap_or_else(|_| panic!("Unexpected Err")).is_none());
            assert!(buffer.push_char('t', Some(&'u')).unwrap_or_else(|_| panic!("Unexpected Err")).is_none());
            assert!(buffer.push_char('u', None).unwrap_or_else(|_| panic!("Unexpected Err")).is_some());
        });

        with_fresh_buffer(|buffer| {
            assert!(buffer.push_char('-', None).unwrap_or_else(|_| panic!("Unexpected Err")).is_some());
        });

        with_fresh_buffer(|buffer| {
            assert!(buffer.push_char('-', Some(&'>')).unwrap_or_else(|_| panic!("Unexpected Err")).is_none());
            assert!(buffer.push_char('>', None).unwrap_or_else(|_| panic!("Unexpected Err")).is_some());
        });
    }

    fn with_fresh_buffer(body: fn(&mut LexBuffer) -> ()) {
        body(&mut LexBuffer {
            mode: LexingState::Normal,
            buffer: String::new(),
            current_line: 0,
            current_column: 0,
            token_column_marker: 0,
            string_escape_flag: false,
        })
    }

    // Actual lexer tests

    #[test]
    fn test_lexer_single_tokens() {
        token_lexes_to("let", Let);
        token_lexes_to("fun", Fun);
        token_lexes_to("=", Assign);
        token_lexes_to("==", Equals);
        token_lexes_to("foo", Identifier("foo".to_string()));
        for nmbr in 0..100 {
            token_lexes_to(&nmbr.to_string(), Integer(nmbr));
        }
        token_lexes_to("\"\"", Str("".to_string()));
        token_lexes_to("\"foo\"", Str("foo".to_string()));
        token_lexes_to(";", Semicolon);
        token_lexes_to("(", LeftParens);
        token_lexes_to(")", RightParens);
        token_lexes_to("->", Arrow);
        token_lexes_to("-", Minus);
        token_lexes_to("+", Plus);
        token_lexes_to("/", Division);
        token_lexes_to(",", Comma);
    }

    #[test]
    fn test_string_escape() {
        // this is "\""
        with_input_lexes_to("\"\\\"\"", vec![dummy_token(Str("\"".to_string()))]);
        with_input_lexes_to("\"hello \\\"world\\\"\"", vec![dummy_token(Str("hello \"world\"".to_string()))]);
        // this is "\\"
        with_input_lexes_to("\"\\\\\"", vec![dummy_token(Str("\\".to_string()))]);
    }

    #[test]
    fn test_lexer_reasonable_statemens() {
        with_input_lexes_to("let a = 1;", vec![
            dummy_token(Let),
            dummy_token(Identifier("a".to_string())),
            dummy_token(Assign),
            dummy_token(Integer(1)),
            dummy_token(Semicolon),
        ]);
        with_input_lexes_to("let a=1;", vec![
            dummy_token(Let),
            dummy_token(Identifier("a".to_string())),
            dummy_token(Assign),
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
        with_input_lexes_to("call(a, b, c);", vec![
            dummy_token(Identifier("call".to_string())),
            dummy_token(LeftParens),
            dummy_token(Identifier("a".to_string())),
            dummy_token(Comma),
            dummy_token(Identifier("b".to_string())),
            dummy_token(Comma),
            dummy_token(Identifier("c".to_string())),
            dummy_token(RightParens),
            dummy_token(Semicolon),
        ]);
        with_input_lexes_to("call(a,b,c);", vec![
            dummy_token(Identifier("call".to_string())),
            dummy_token(LeftParens),
            dummy_token(Identifier("a".to_string())),
            dummy_token(Comma),
            dummy_token(Identifier("b".to_string())),
            dummy_token(Comma),
            dummy_token(Identifier("c".to_string())),
            dummy_token(RightParens),
            dummy_token(Semicolon),
        ]);
        with_input_lexes_to("1 / 2", vec![
            dummy_token(Integer(1)),
            dummy_token(Division),
            dummy_token(Integer(2)),
        ]);
        with_input_lexes_to("1 == 2", vec![
            dummy_token(Integer(1)),
            dummy_token(Equals),
            dummy_token(Integer(2)),
        ]);
        with_input_lexes_to("1==2", vec![
            dummy_token(Integer(1)),
            dummy_token(Equals),
            dummy_token(Integer(2)),
        ]);
    }

    #[test]
    fn test_lexing_with_line_and_column_references() {
        with_input_lexes_to_assert_columns(
            "let foo = 1;\nlet bar = \"bar value with whitespace\";",
            vec![
                token_at(Let, 1, 0),
                token_at(Identifier("foo".to_string()), 1, 4),
                token_at(Assign, 1, 8),
                token_at(Integer(1), 1, 10),
                token_at(Semicolon, 1, 11),
                token_at(Let, 2, 0),
                token_at(Identifier("bar".to_string()), 2, 4),
                token_at(Assign, 2, 8),
                token_at(Str("bar value with whitespace".to_string()), 2, 10),
                token_at(Semicolon, 2, 37),
            ],
        )
    }

    #[test]
    fn test_error_cases() {
        with_input_errors_to("1234var", LexingError {
            msg: "identifier can't start with digit".to_string(),
            location: SourceRef {
                line: 1,
                column: 3,
            }
        });
        with_input_errors_to("\"hello", LexingError {
            msg: "string is not terminated".to_string(),
            location: SourceRef {
                line: 1,
                column: 3,
            }
        });
        with_input_errors_to("\"hello // comment", LexingError {
            msg: "string is not terminated".to_string(),
            location: SourceRef {
                line: 1,
                column: 3,
            }
        });
    }

    #[test]
    fn test_interesting_corner_cases() {
        with_input_lexes_to(
            "",
            vec![],
        );
        with_input_lexes_to(
            " ",
            vec![],
        );
        with_input_lexes_to(
            "\t",
            vec![],
        );
        with_input_lexes_to(
            "\n",
            vec![],
        );
        with_input_lexes_to(
            "1 + 2",
            vec![
                dummy_token(Integer(1)),
                dummy_token(Plus),
                dummy_token(Integer(2)),
            ],
        );
        with_input_lexes_to(
            "1+2",
            vec![
                dummy_token(Integer(1)),
                dummy_token(Plus),
                dummy_token(Integer(2)),
            ],
        );
        with_input_lexes_to(
            "\"let a = 1\"",
            vec![
                dummy_token(Str("let a = 1".to_string())),
            ],
        );
        with_input_lexes_to(
            "let\n a\n = \n1;",
            vec![
                dummy_token(Let),
                dummy_token(Identifier("a".to_string())),
                dummy_token(Assign),
                dummy_token(Integer(1)),
                dummy_token(Semicolon),
            ],
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
            ],
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
            ],
        );
    }

    #[test]
    fn test_comments() {
        with_input_lexes_to("// Hello world", vec![]);
        with_input_lexes_to("let a = 5; // Hello world", vec![
            dummy_token(Let),
            dummy_token(Identifier("a".to_string())),
            dummy_token(Assign),
            dummy_token(Integer(5)),
            dummy_token(Semicolon),
        ]);
        with_input_lexes_to("let a = 5;// Hello world", vec![
            dummy_token(Let),
            dummy_token(Identifier("a".to_string())),
            dummy_token(Assign),
            dummy_token(Integer(5)),
            dummy_token(Semicolon),
        ]);
    }

    fn token_lexes_to(input: &str, expected_kind: TokenKind) {
        with_input_lexes_to(input, vec![dummy_token(expected_kind)]);
    }

    fn with_input_lexes_to(input: &str, expected_tokens: Vec<Token>) {
        do_lexing_assertion(input, expected_tokens, false)
    }

    fn with_input_errors_to(input: &str, expected_error: LexingError) {
        match create_lexer(input) {
            Ok(_) => panic!("Expecting lexer to error, but working lexer was returned. Input: {}", input),
            Err(error) => {
                assert_eq!(expected_error.msg, error.msg)
            }
        }
    }

    fn with_input_lexes_to_assert_columns(input: &str, expected_tokens: Vec<Token>) {
        do_lexing_assertion(input, expected_tokens, true)
    }

    /// Asserts given input lexes to expected tokens
    /// Note: this also checks lines and columns
    fn do_lexing_assertion(input: &str, expected_tokens: Vec<Token>, assert_refs: bool) {
        match create_lexer(input) {
            Ok(mut lexer) => {
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
            },
            Err(error) => panic!("Unexpected lexing error: {}", error.msg)
        }
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
        create_token(kind, line, column)
    }

    /// Creates dummy token, with source references zeroed
    /// Cannot be used for assertions with line and column references but
    /// is much more fast to write
    fn dummy_token(kind: TokenKind) -> Token {
        create_token(kind, 0, 0)
    }
}

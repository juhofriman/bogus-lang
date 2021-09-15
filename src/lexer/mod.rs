pub mod tokens;

use tokens::{Token, TokenKind, SourceRef };

use core::fmt;
use crate::lexer::ShouldContinue::{BailOut, Continue};

/// Consumable lexer instance, create with create_lexer()
pub struct Lexer {
    tokens: Vec<Token>,
    pointer: usize,
}

impl Lexer {

    pub fn new(source: &str) -> Result<Lexer, LexingError> {
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
            // This forwards LexingError
            if let Some(token) = buffer.push_char(char, character_iter.peek())? {
                tokens.push(token);
            }
        }
        Ok(Lexer {
            pointer: 0,
            tokens,
        })
    }

    /// Advances lexer and returns next token
    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pointer);
        self.pointer += 1;
        token
    }

    /// Returns next() and wraps None to UnexpectedEOFError
    pub fn next_or_err(&mut self) -> Result<&Token, UnexpectedEOFError> {
        match self.next() {
            Some(token) => Ok(token),
            None => Err( UnexpectedEOFError {})
        }
    }

    /// Peeks next token without advancing the lexer
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pointer)
    }

    /// Returns peek() and wraps None to UnexpectedEOFError
    pub fn peek_or_err(&mut self) -> Result<&Token, UnexpectedEOFError> {
        match self.peek() {
            Some(token) => Ok(token),
            None => Err( UnexpectedEOFError {})
        }
    }

    /// Tells if lexer has next token
    pub fn has_next(&self) -> bool {
        self.pointer + 1 <= self.tokens.len()
    }
}

/// An Error that happens during lexing
pub struct LexingError {
    msg: String,
    location: SourceRef,
}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexing Error: {} @ {}", self.msg, self.location)
    }
}

/// UnexpectedEOFError can be propagated to parser error
/// Notifies unexpected end of received input
pub struct UnexpectedEOFError {}

impl fmt::Display for UnexpectedEOFError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected EOF")
    }
}

/// State enum for lexer. Lexer behaves differently in different modes
#[derive(PartialEq, Debug)]
enum LexingState {
    Normal,
    Integer,
    Float,
    String,
    LineComment,
}

/// LexBuffer is used for stateful lexing the given input
struct LexBuffer {
    buffer: String,
    mode: LexingState,
    current_line: u32,
    current_column: u32,
    token_column_marker: u32,
    string_escape_flag: bool,
}

/// Utility enum for LexBuffer, for determining whether lexer should
/// proceed to next stage with current_character
enum ShouldContinue {
    Continue,
    BailOut,
}

impl LexBuffer {

    /// Pushes character to buffer.
    /// Returns Some(Token) if push makes new token ready. Returns None if
    /// Lexer expects more input
    fn push_char(&mut self, current_char: char, peek: Option<&char>) -> Result<Option<Token>, LexingError> {

        // Increment counters
        if let BailOut = self.proceed_with_counters(&current_char) {
            return Ok(None);
        }

        // Fill buffers and bail out if necessary
        if let BailOut = self.fill_buffer(&current_char, peek) {
            return Ok(None);
        }

        // If something is ready, pop new token out
        match self.mode {

            // This just has to be here, fill_buffer already BailsOut in LineComment mode
            LexingState::LineComment => {
                Ok(None)
            }

            //
            LexingState::Integer => {
                if is_delimiting_opt(peek) {
                    let value: Result<i32, _> = self.buffer.parse();
                    return match value {
                        Ok(value) => {
                            Ok(Some(self.pop_buffer(TokenKind::Integer(value))))
                        }
                        Err(_) => Err(LexingError {
                            msg: "identifier can't start with digit".to_string(),
                            location: SourceRef {
                                line: self.current_line,
                                column: self.token_column_marker,
                            },
                        })
                    };
                }
                Ok(None)
            }

            LexingState::Float => {
                if is_delimiting_opt(peek) {
                    let value: Result<f32, _> = self.buffer.parse();
                    return match value {
                        Ok(value) => {
                            Ok(Some(self.pop_buffer(TokenKind::Float(value))))
                        }
                        Err(_) => Err(LexingError {
                            msg: "identifier can't start with digit".to_string(),
                            location: SourceRef {
                                line: self.current_line,
                                column: self.token_column_marker,
                            },
                        })
                    };
                }
                Ok(None)
            }

            LexingState::String => {
                if current_char == '"' {
                    return Ok(Some(self.pop_buffer(TokenKind::Str(self.buffer.to_string()))));
                }
                if peek.is_none() {
                    return Err(LexingError {
                        msg: "string is not terminated".to_string(),
                        location: SourceRef {
                            line: self.current_line,
                            column: self.token_column_marker,
                        },
                    });
                }
                Ok(None)
            }

            LexingState::Normal => {
                match self.buffer.as_str() {

                    // match reserved words and operators

                    "let" => Ok(self.pop_buffer_cond(
                        TokenKind::Let,
                        is_delimiting_opt(peek))),
                    "const" => Ok(self.pop_buffer_cond(
                        TokenKind::Const,
                        is_delimiting_opt(peek))),
                    "fun" => Ok(self.pop_buffer_cond(
                        TokenKind::Fun,
                        is_delimiting_opt(peek))),
                    "null" => Ok(self.pop_buffer_cond(
                        TokenKind::Null,
                        is_delimiting_opt(peek))),
                    "(" => Ok(Some(self.pop_buffer(TokenKind::LeftParens))),
                    ")" => Ok(Some(self.pop_buffer(TokenKind::RightParens))),
                    "," => Ok(Some(self.pop_buffer(TokenKind::Comma))),
                    "." => Ok(Some(self.pop_buffer(TokenKind::Dot))),
                    "-" => Ok(self.pop_buffer_cond(
                        TokenKind::Minus,
                        char_is_not(peek, '>'))),
                    "+" => Ok(Some(self.pop_buffer(TokenKind::Plus))),
                    "/" => Ok(Some(self.pop_buffer(TokenKind::Division))),
                    "*" => Ok(Some(self.pop_buffer(TokenKind::Multiplication))),
                    "->" => Ok(Some(self.pop_buffer(TokenKind::Arrow))),
                    ";" => Ok(Some(self.pop_buffer(TokenKind::Semicolon))),
                    "=" => Ok(self.pop_buffer_cond(
                        TokenKind::Assign,
                        char_is_not(peek, '='))),
                    "==" => Ok(Some(self.pop_buffer(TokenKind::Equals))),

                    // Match identifiers

                    _ => {
                        if is_delimiting_opt(peek) {
                            let token_kind = TokenKind::Identifier(self.buffer.to_string());
                            let token = self.pop_buffer(token_kind);
                            return Ok(Some(token));
                        }
                        Ok(None)
                    }
                }
            }
        }
    }

    /// Increments the counters and resets LineComment mode if newline is encountered
    /// Returns BailOut, if current_char was newline and thus OK(None) should be returned
    fn proceed_with_counters(&mut self, current_char: &char) -> ShouldContinue {
        self.current_column += 1;
        match current_char {
            '\n' => {
                if self.mode == LexingState::LineComment {
                    self.mode = LexingState::Normal
                }
                self.current_line += 1;
                self.current_column = 0;
                self.token_column_marker = 0;
                BailOut
            },
            _ => Continue
        }
    }

    /// Fills buffer according to current state of the lexer
    /// Returns BailOut if Ok(None) should be returned (no token from this character)
    /// fill_buffer also changes LexingMode when required
    fn fill_buffer(&mut self, current_char: &char, peek: Option<&char>) -> ShouldContinue {
        match self.mode {

            // In LineComment mode, just BailOut always
            LexingState::LineComment => BailOut,

            // In Normal mode
            LexingState::Normal => match current_char {
                // Whitespace encountered, proceed to next column_marker and BailOut
                _ if current_char.is_whitespace() => {
                    self.token_column_marker += 1;
                    BailOut
                }
                // New line comment starts from this character
                '/' if char_is(peek, '/') => {
                    self.mode = LexingState::LineComment;
                    BailOut
                }
                // New integer starts when buffer is empty and current_character is digit
                // Needs to Continue, because it can be an integer with just one digit
                _ if self.buffer.is_empty() && current_char.is_digit(10) => {
                    self.mode = LexingState::Integer;
                    self.buffer.push(*current_char);
                    // Handle the situation where float has one digit and thus
                    // it must go to Float mode directly. I.e 1.12
                    if char_is(peek, '.') {
                        self.mode = LexingState::Float;
                        // BailOut because we don't want to pop integer out now
                        return BailOut
                    }
                    Continue
                }
                // New string starts, note that " is not pushed to buffer
                '"' if self.buffer.is_empty() => {
                    self.mode = LexingState::String;
                    BailOut
                }
                // Just push to buffer and Continue
                _ => {
                    self.buffer.push(*current_char);
                    Continue
                }
            },

            // In String mode
            LexingState::String => match current_char {
                // Escape flag encountered in string, set flag on and BailOut
                _ if self.string_escape_flag => {
                    // Escape flag was on, push this to buffer and BailOut because we're
                    // in between on String
                    self.buffer.push(*current_char);
                    self.string_escape_flag = false;
                    BailOut
                }
                // Escape flag is set encountered, set flag on and BailOut
                '\\' if !self.string_escape_flag => {
                    self.string_escape_flag = true;
                    BailOut
                }
                // In String mode when " is encountered, just Continue and new String will
                // pop out. The character is not pushed to buffer because it's not part of the data.
                '"' => Continue,
                // Just push character to buffer and continue
                // Continue is required because peek can be None (EOF) and
                // it must raise an error
                _ => {
                    self.buffer.push(*current_char);
                    Continue
                }
            },

            // In Integer mode just push to buffer always
            // Continue will check if the Integer is malformed
            LexingState::Integer => {
                self.buffer.push(*current_char);
                // Promote to float mode and BailOut because no Integer must be popped
                if char_is(peek, '.') {
                    self.mode = LexingState::Float;
                    return BailOut
                }
                Continue
            }

            // In Float mode just push to buffer always
            LexingState::Float => match current_char {
                _ => {
                    self.buffer.push(*current_char);
                    Continue
                }
            }
        }
    }

    /// Pops Token out of buffer with given kind and resets
    /// column markers and mode back to Normal
    fn pop_buffer(&mut self, kind: TokenKind) -> Token {

        // back to normal
        self.mode = LexingState::Normal;
        self.buffer.clear();

        // Create token with token_column_marker (the column that started token)
        let new_token = Token::new(kind,
                                     self.current_line,
                                     self.token_column_marker);

        // Set next token column marker to current_column
        self.token_column_marker = self.current_column;

        new_token
    }

    /// Pops buffer to given kind if should_pop is true
    fn pop_buffer_cond(
        &mut self,
        kind: TokenKind,
        should_pop: bool) -> Option<Token> {
        if should_pop {
            return Some(self.pop_buffer(kind));
        }
        None
    }

}

/// Tells if given character is delimiting
/// for example foo(.. creates two tokens: identifier and ( token.
fn is_delimiting(c: &char) -> bool {
    match c {
        _ if c.is_whitespace() => true,
        ';' => true,
        '(' => true,
        ')' => true,
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        ',' => true,
        '.' => true,
        '=' => true,
        _ => false
    }
}

// Wrapper for Option to check if char is delimiting
// Note that None is delimiting, because character of None is EOF
fn is_delimiting_opt(c: Option<&char>) -> bool {
    match c {
        Some(char) => is_delimiting(char),
        None => true,
    }
}

fn char_is(peek: Option<&char>, this: char) -> bool {
    match peek {
        Some(peek_char) => *peek_char == this,
        None => false
    }
}

fn char_is_not(peek: Option<&char>, this: char) -> bool {
    !char_is(peek, this)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::lexer::tokens::TokenKind::{Let, Identifier, Assign, Integer, Str, Semicolon, RightParens, LeftParens, Arrow, Minus, Plus, Fun, Comma, Division, Equals, Const, Float, Dot, Multiplication, Null};

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

    // Testing lexer usage

    #[test]
    fn test_lexer_usage() {
        match Lexer::new("let a = 1;") {
            Ok(mut lexer) => {
                assert!(lexer.has_next());
                assert_eq!(lexer.peek().unwrap().token_kind, Let);
                assert_eq!(lexer.next().unwrap().token_kind, Let);
                assert_eq!(lexer.peek().unwrap().token_kind, Identifier("a".to_string()));

                assert_eq!(lexer.next().unwrap().token_kind, Identifier("a".to_string()));
                assert_eq!(lexer.peek().unwrap().token_kind, Assign);

                assert_eq!(lexer.next().unwrap().token_kind, Assign);
                assert_eq!(lexer.peek().unwrap().token_kind, Integer(1));

                assert_eq!(lexer.next().unwrap().token_kind, Integer(1));
                assert_eq!(lexer.peek().unwrap().token_kind, Semicolon);

                assert_eq!(lexer.next().unwrap().token_kind, Semicolon);

                assert!(lexer.peek().is_none());
                assert!(lexer.next().is_none());
            },
            Err(error) => panic!("create_lexer did not work: {}", error.msg)
        }
    }

    // Actual lexer tests

    #[test]
    fn test_lexer_single_tokens() {
        token_lexes_to("let", Let);
        token_lexes_to("const", Const);
        token_lexes_to("fun", Fun);
        token_lexes_to("=", Assign);
        token_lexes_to("==", Equals);
        token_lexes_to("foo", Identifier("foo".to_string()));
        for nmbr in 0..100 {
            token_lexes_to(&nmbr.to_string(), Integer(nmbr));
        }
        token_lexes_to("1.12", Float(1.12));
        token_lexes_to("124.99", Float(124.99));
        token_lexes_to("\"\"", Str("".to_string()));
        token_lexes_to("\"foo\"", Str("foo".to_string()));
        token_lexes_to(";", Semicolon);
        token_lexes_to("(", LeftParens);
        token_lexes_to(")", RightParens);
        token_lexes_to("->", Arrow);
        token_lexes_to("-", Minus);
        token_lexes_to("+", Plus);
        token_lexes_to("/", Division);
        token_lexes_to("*", Multiplication);
        token_lexes_to(",", Comma);
        token_lexes_to(".", Dot);
        token_lexes_to("null", Null);
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
        with_input_lexes_to("const a = 1;", vec![
            dummy_token(Const),
            dummy_token(Identifier("a".to_string())),
            dummy_token(Assign),
            dummy_token(Integer(1)),
            dummy_token(Semicolon),
        ]);
        with_input_lexes_to("const a=1;", vec![
            dummy_token(Const),
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
        with_input_lexes_to("foo.do();", vec![
            dummy_token(Identifier("foo".to_string())),
            dummy_token(Dot),
            dummy_token(Identifier("do".to_string())),
            dummy_token(LeftParens),
            dummy_token(RightParens),
            dummy_token(Semicolon),
        ]);
        with_input_lexes_to("1 / 2", vec![
            dummy_token(Integer(1)),
            dummy_token(Division),
            dummy_token(Integer(2)),
        ]);
        with_input_lexes_to("1 * 2", vec![
            dummy_token(Integer(1)),
            dummy_token(Multiplication),
            dummy_token(Integer(2)),
        ]);
        with_input_lexes_to("1*2", vec![
            dummy_token(Integer(1)),
            dummy_token(Multiplication),
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
        with_input_lexes_to("null + null", vec![
            dummy_token(Null),
            dummy_token(Plus),
            dummy_token(Null),
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
            },
        });
        with_input_errors_to("\"hello", LexingError {
            msg: "string is not terminated".to_string(),
            location: SourceRef {
                line: 1,
                column: 3,
            },
        });
        with_input_errors_to("\"hello // comment", LexingError {
            msg: "string is not terminated".to_string(),
            location: SourceRef {
                line: 1,
                column: 3,
            },
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
        match Lexer::new(input) {
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
        match Lexer::new(input) {
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
            }
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
        Token::new(kind, line, column)
    }

    /// Creates dummy token, with source references zeroed
    /// Cannot be used for assertions with line and column references but
    /// is much more fast to write
    fn dummy_token(kind: TokenKind) -> Token {
        Token::new(kind, 0, 0)
    }
}

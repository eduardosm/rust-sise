// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::is_atom_chr;
use crate::is_atom_string_chr;
use crate::ReadItem;
use crate::ReadItemKind;
use crate::Reader;

/// Position of a byte in the source file.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytePos(pub usize);

impl std::fmt::Display for BytePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// Represents a parse error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// There is an invalid character.
    IllegalChr { pos: BytePos, chr: u8 },

    /// There is an invalid character inside a string (enclosed with `"`).
    IllegalChrInString { pos: BytePos, chr: u8 },

    /// There is an invalid character inside a comment.
    IllegalChrInComment { pos: BytePos, chr: u8 },

    /// End-of-file is reached before finding the closing `"`.
    UnfinishedString { pos: BytePos },

    /// Unexpected token.
    UnexpectedToken { pos: BytePos, token: TokenKind },

    /// Found a token when expecting end-of-file.
    ExpectedEof { pos: BytePos },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParseError::IllegalChr { pos, chr } => {
                write!(f, "illegal character 0x{:02X} at byte {}", chr, pos)
            }
            ParseError::IllegalChrInString { pos, chr } => write!(
                f,
                "illegal character 0x{:02X} in string at byte {}",
                chr, pos,
            ),
            ParseError::IllegalChrInComment { pos, chr } => write!(
                f,
                "illegal character 0x{:02X} in comment at byte {}",
                chr, pos,
            ),
            ParseError::UnfinishedString { pos } => write!(f, "unfinished string at byte {}", pos),
            ParseError::UnexpectedToken { pos, ref token } => {
                write!(f, "unexpected token {:?} at byte {}", token, pos)
            }
            ParseError::ExpectedEof { pos } => write!(f, "expected end-of-file at byte {}", pos),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Eof,
    LeftParen,
    RightParen,
    Atom,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token<'a> {
    Eof,
    LeftParen,
    RightParen,
    Atom(&'a str),
}

/// Parser that decodes a SISE file from memory.
///
/// # Example
///
/// ```
/// use sise::Reader as _;
/// let data = b"(test (1 2 3))";
/// let mut parser = sise::Parser::new(data);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListBeginning);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("test"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListBeginning);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("1"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("2"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::Atom("3"));
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// assert_eq!(parser.read().unwrap().kind, sise::ReadItemKind::ListEnding);
/// parser.finish().unwrap();
/// ```
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    state: State,
}

enum State {
    Beginning,
    Parsing { depth: usize },
    Finishing,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            lexer: Lexer::new(data),
            state: State::Beginning,
        }
    }
}

impl<'a> Reader for Parser<'a> {
    type Error = ParseError;
    type String = &'a str;
    type Pos = BytePos;

    fn read(&mut self) -> Result<ReadItem<&'a str, BytePos>, ParseError> {
        match self.state {
            State::Beginning => {
                let (pos, token) = self.lexer.get_token()?;
                match token {
                    Token::Eof => Err(ParseError::UnexpectedToken {
                        pos,
                        token: TokenKind::Eof,
                    }),
                    Token::LeftParen => {
                        self.state = State::Parsing { depth: 0 };
                        Ok(ReadItem {
                            pos,
                            kind: ReadItemKind::ListBeginning,
                        })
                    }
                    Token::RightParen => Err(ParseError::UnexpectedToken {
                        pos,
                        token: TokenKind::RightParen,
                    }),
                    Token::Atom(atom) => {
                        self.state = State::Finishing;
                        Ok(ReadItem {
                            pos,
                            kind: ReadItemKind::Atom(atom),
                        })
                    }
                }
            }
            State::Parsing { ref mut depth } => {
                let (pos, token) = self.lexer.get_token()?;
                match token {
                    Token::Eof => Err(ParseError::UnexpectedToken {
                        pos,
                        token: TokenKind::Eof,
                    }),
                    Token::LeftParen => {
                        *depth += 1;
                        Ok(ReadItem {
                            pos,
                            kind: ReadItemKind::ListBeginning,
                        })
                    }
                    Token::RightParen => {
                        if *depth == 0 {
                            self.state = State::Finishing;
                        } else {
                            *depth -= 1;
                        }
                        Ok(ReadItem {
                            pos,
                            kind: ReadItemKind::ListEnding,
                        })
                    }
                    Token::Atom(atom) => Ok(ReadItem {
                        pos,
                        kind: ReadItemKind::Atom(atom),
                    }),
                }
            }
            State::Finishing => panic!("parsing finished"),
        }
    }

    fn finish(mut self) -> Result<(), ParseError> {
        match self.state {
            State::Finishing => {
                let (pos, token) = self.lexer.get_token()?;
                match token {
                    Token::Eof => Ok(()),
                    _ => Err(ParseError::ExpectedEof { pos }),
                }
            }
            _ => panic!("parsing not finished yet"),
        }
    }
}

struct Lexer<'a> {
    data: &'a [u8],
    pos: BytePos,
}

impl<'a> Lexer<'a> {
    #[inline]
    fn new(data: &'a [u8]) -> Self {
        Lexer {
            data,
            pos: BytePos(0),
        }
    }

    #[inline]
    fn peek_char(&self, i: usize) -> Option<u8> {
        self.data.get(i).cloned()
    }

    fn skip_chars(&mut self, len: usize) {
        self.data = &self.data[len..];
    }

    fn data_iter(&self, i: usize) -> std::iter::Cloned<std::slice::Iter<'a, u8>> {
        self.data[i..].iter().cloned()
    }

    fn take_atom(&mut self, len: usize) -> &'a str {
        let (atom, remaining) = self.data.split_at(len);
        self.data = remaining;
        std::str::from_utf8(atom).unwrap()
    }

    fn get_token(&mut self) -> Result<(BytePos, Token<'a>), ParseError> {
        loop {
            match self.peek_char(0) {
                // end-of-file
                None => {
                    return Ok((self.pos, Token::Eof));
                }
                // skip whitespace
                Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r') => {
                    let mut len = 1;
                    for c in self.data_iter(len) {
                        if c != b' ' && c != b'\t' && c != b'\n' && c != b'\r' {
                            break;
                        }
                        len += 1;
                    }
                    self.pos.0 += len;
                    self.skip_chars(len);
                }
                // skip comments
                Some(b';') => {
                    let mut len = 1;
                    for c in self.data_iter(len) {
                        match c {
                            b'\n' | b'\r' => {
                                len += 1;
                                break;
                            }
                            b' '..=b'~' | b'\t' => {
                                len += 1;
                            }
                            chr => {
                                self.pos.0 += len;
                                return Err(ParseError::IllegalChrInComment { chr, pos: self.pos });
                            }
                        }
                    }
                    self.pos.0 += len;
                    self.skip_chars(len);
                }
                // delimiters
                Some(b'(') => {
                    let pos = self.pos;
                    self.pos.0 += 1;
                    self.skip_chars(1);
                    return Ok((pos, Token::LeftParen));
                }
                Some(b')') => {
                    let pos = self.pos;
                    self.pos.0 += 1;
                    self.skip_chars(1);
                    return Ok((pos, Token::RightParen));
                }
                // atom
                Some(chr) if is_atom_chr(chr) || chr == b'"' => {
                    let len = self.lex_atom(chr)?;
                    let pos = self.pos;
                    self.pos.0 += len;
                    let atom = self.take_atom(len);
                    return Ok((pos, Token::Atom(atom)));
                }
                // invalid character
                Some(chr) => {
                    return Err(ParseError::IllegalChr { chr, pos: self.pos });
                }
            }
        }
    }

    fn lex_atom(&mut self, first_chr: u8) -> Result<usize, ParseError> {
        enum State {
            Normal,
            String,
            StringBackslash,
        }

        let mut state = if first_chr == b'"' {
            State::String
        } else {
            State::Normal
        };
        let mut len = 1;
        let mut iter = self.data_iter(len);
        loop {
            let chr = iter.next();
            match state {
                State::Normal => match chr {
                    Some(b'"') => {
                        len += 1;
                        state = State::String;
                    }
                    Some(c) if is_atom_chr(c) => {
                        len += 1;
                    }
                    Some(_) => {
                        return Ok(len);
                    }
                    None => {
                        return Ok(len);
                    }
                },
                State::String => match chr {
                    Some(b'"') => {
                        len += 1;
                        state = State::Normal;
                    }
                    Some(b'\\') => {
                        len += 1;
                        state = State::StringBackslash;
                    }
                    Some(c) if is_atom_string_chr(c) => {
                        len += 1;
                        state = State::String;
                    }
                    Some(c) => {
                        self.pos.0 += len;
                        return Err(ParseError::IllegalChrInString {
                            chr: c,
                            pos: self.pos,
                        });
                    }
                    None => {
                        self.pos.0 += len;
                        return Err(ParseError::UnfinishedString { pos: self.pos });
                    }
                },
                State::StringBackslash => match chr {
                    Some(c) if is_atom_string_chr(c) || c == b'"' || c == b'\\' => {
                        len += 1;
                        state = State::String;
                    }
                    Some(c) => {
                        self.pos.0 += len;
                        return Err(ParseError::IllegalChrInString {
                            chr: c,
                            pos: self.pos,
                        });
                    }
                    None => {
                        self.pos.0 += len;
                        return Err(ParseError::UnfinishedString { pos: self.pos });
                    }
                },
            }
        }
    }
}

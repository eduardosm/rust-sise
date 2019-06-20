// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::convert::TryFrom;

use crate::Pos;
use crate::ReprPosValue;
use crate::Reader;
use crate::ReadItem;
use crate::ReadItemKind;
use crate::is_atom_chr;
use crate::is_atom_string_chr;

/// Represents a parse error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// There is an invalid character.
    IllegalChr {
        pos: Pos,
        chr: u8,
    },

    /// There is an invalid character inside a string (enclosed with `"`).
    IllegalChrInString {
        pos: Pos,
        chr: u8,
    },

    /// There is an invalid character inside a comment.
    IllegalChrInComment {
        pos: Pos,
        chr: u8,
    },

    /// End-of-file is reached before finding the closing `"`.
    UnfinishedString {
        pos: Pos,
    },

    /// Unexpected token.
    UnexpectedToken {
        pos: Pos,
        token: TokenKind,
    },

    /// Found a token when expecting end-of-file.
    ExpectedEof {
        pos: Pos,
    },

    /// A line is longer than `u32::max_value()`.
    LineTooLong {
        line: u32,
    },

    /// There are more than `u32::max_value()` lines.
    TooManyLines,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParseError::IllegalChr { pos, chr } => {
                write!(f, "illegal character 0x{:02X} at {}:{}",
                       chr, ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::IllegalChrInString { pos, chr } => {
                write!(f, "illegal character 0x{:02X} in string at {}:{}",
                       chr, ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::IllegalChrInComment { pos, chr } => {
                write!(f, "illegal character 0x{:02X} in comment at {}:{}",
                       chr, ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::UnfinishedString { pos } => {
                write!(f, "unfinished string at {}:{}",
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::UnexpectedToken { pos, ref token } => {
                write!(f, "unexpected token {:?} at {}:{}",
                       token,
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::ExpectedEof { pos } => {
                write!(f, "expected end-of-file at {}:{}",
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::LineTooLong { line } => {
                write!(f, "line {} too long", ReprPosValue(line))
            }
            ParseError::TooManyLines => {
                write!(f, "too many lines")
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::IllegalChr { .. } => "illegal character",
            ParseError::IllegalChrInString { .. } => "illegal character in string",
            ParseError::IllegalChrInComment { .. } => "illegal character in comment",
            ParseError::UnfinishedString { .. } => "unfinished string",
            ParseError::UnexpectedToken { .. } => "unexpected token",
            ParseError::ExpectedEof { .. } => "expected end-of-file",
            ParseError::LineTooLong { .. } => "line too long",
            ParseError::TooManyLines => "too many lines",
        }
    }
}

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
    Parsing {
        depth: usize,
    },
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
    type Pos = Pos;

    fn read(&mut self) -> Result<ReadItem<&'a str, Pos>, ParseError> {
        match self.state {
            State::Beginning => {
                let (pos, token) = self.lexer.get_token()?;
                match token {
                    Token::Eof => {
                        Err(ParseError::UnexpectedToken {
                            pos,
                            token: TokenKind::Eof,
                        })
                    }
                    Token::LeftParen => {
                        self.state = State::Parsing { depth: 0 };
                        Ok(ReadItem {
                            pos,
                            kind: ReadItemKind::ListBeginning,
                        })
                    }
                    Token::RightParen => {
                        Err(ParseError::UnexpectedToken {
                            pos,
                            token: TokenKind::RightParen,
                        })
                    }
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
                    Token::Eof => {
                        Err(ParseError::UnexpectedToken {
                            pos,
                            token: TokenKind::Eof,
                        })
                    }
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
                    Token::Atom(atom) => {
                        Ok(ReadItem {
                            pos,
                            kind: ReadItemKind::Atom(atom),
                        })
                    }
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
    pos: Pos,
}

impl<'a> Lexer<'a> {
    #[inline]
    fn new(data: &'a [u8]) -> Self {
        Lexer {
            data,
            pos: Pos::new(0, 0),
        }
    }

    fn increase_line(&mut self) -> Result<(), ParseError> {
        self.pos.line = self.pos.line.checked_add(1).ok_or(ParseError::TooManyLines)?;
        self.pos.column = 0;
        Ok(())
    }

    fn increase_column(&mut self, len: usize) -> Result<(), ParseError> {
        let len = u32::try_from(len)
            .map_err(|_| ParseError::LineTooLong { line: self.pos.line })?;
        self.pos.column = self.pos.column.checked_add(len)
            .ok_or_else(|| ParseError::LineTooLong { line: self.pos.line })?;
        Ok(())
    }

    #[inline]
    fn peek_char(&self, i: usize) -> Option<u8> {
        self.data.get(i).cloned()
    }

    fn skip_chars(&mut self, len: usize) {
        self.data = &self.data[len ..];
    }

    fn data_iter(&self, i: usize) -> std::iter::Cloned<std::slice::Iter<'a, u8>> {
        self.data[i ..].iter().cloned()
    }

    fn take_atom(&mut self, len: usize) -> &'a str {
        let (atom, remaining) = self.data.split_at(len);
        self.data = remaining;
        std::str::from_utf8(atom).unwrap()
    }

    fn get_token(&mut self) -> Result<(Pos, Token<'a>), ParseError> {
        loop {
            match self.peek_char(0) {
                // end-of-file
                None => {
                    return Ok((self.pos, Token::Eof));
                }
                // skip whitespace
                Some(b' ') | Some(b'\t') => {
                    let mut len = 1;
                    for c in self.data_iter(len) {
                        if c != b' ' && c != b'\t' {
                            break;
                        }
                        len += 1;
                    }
                    self.increase_column(len)?;
                    self.skip_chars(len);
                }
                // skip line breaks
                Some(b'\n') => {
                    self.increase_line()?;
                    self.skip_chars(1);
                }
                Some(b'\r') => {
                    self.increase_line()?;
                    if self.peek_char(1) == Some(b'\n') {
                        self.skip_chars(2);
                    } else {
                        self.skip_chars(1);
                    }
                }
                // skip comments
                Some(b';') => {
                    let mut len = 1;
                    for c in self.data_iter(len) {
                        match c {
                            b'\n' | b'\r' => break,
                            b' ' ..= b'~' | b'\t' => {
                                len += 1;
                            }
                            chr => {
                                self.increase_column(len)?;
                                return Err(ParseError::IllegalChrInComment { chr, pos: self.pos });
                            }
                        }
                    }
                    self.increase_column(len)?;
                    self.skip_chars(len);
                }
                // delimiters
                Some(b'(') => {
                    let pos = self.pos;
                    self.increase_column(1)?;
                    self.skip_chars(1);
                    return Ok((pos, Token::LeftParen));
                }
                Some(b')') => {
                    let pos = self.pos;
                    self.increase_column(1)?;
                    self.skip_chars(1);
                    return Ok((pos, Token::RightParen));
                }
                // atom
                Some(chr) if is_atom_chr(chr) || chr == b'"' => {
                    let len = self.lex_atom(chr)?;
                    let pos = self.pos;
                    self.increase_column(len)?;
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

        let mut state = if first_chr == b'"' { State::String } else { State::Normal };
        let mut len = 1;
        let mut iter = self.data_iter(len);
        loop {
            let chr = iter.next();
            match state {
                State::Normal => {
                    match chr {
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
                    }
                }
                State::String => {
                    match chr {
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
                            self.increase_column(len)?;
                            return Err(ParseError::IllegalChrInString { chr: c, pos: self.pos });
                        }
                        None => {
                            self.increase_column(len)?;
                            return Err(ParseError::UnfinishedString { pos: self.pos });
                        }
                    }
                }
                State::StringBackslash => {
                    match chr {
                        Some(c) if is_atom_string_chr(c) || c == b'"' || c == b'\\' => {
                            len += 1;
                            state = State::String;
                        }
                        Some(c) => {
                            self.increase_column(len)?;
                            return Err(ParseError::IllegalChrInString { chr: c, pos: self.pos });
                        }
                        None => {
                            self.increase_column(len)?;
                            return Err(ParseError::UnfinishedString { pos: self.pos });
                        }
                    }
                }
            }
        }
    }
}

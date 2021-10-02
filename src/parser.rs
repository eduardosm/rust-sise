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

impl core::fmt::Display for BytePos {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}

/// Represents a parse error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// There is an invalid character.
    IllegalChr { pos: BytePos, chr: char },

    /// There is an invalid character inside a string (enclosed with `"`).
    IllegalChrInString { pos: BytePos, chr: char },

    /// There is an invalid character inside a comment.
    IllegalChrInComment { pos: BytePos, chr: char },

    /// End-of-file is reached before finding the closing `"`.
    UnfinishedString { pos: BytePos },

    /// Unexpected token.
    UnexpectedToken { pos: BytePos, token: TokenKind },

    /// Found a token when expecting end-of-file.
    ExpectedEof { pos: BytePos },
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            ParseError::IllegalChr { pos, chr } => {
                write!(f, "illegal character {:?} at byte {}", chr, pos)
            }
            ParseError::IllegalChrInString { pos, chr } => {
                write!(f, "illegal character {:?} in string at byte {}", chr, pos)
            }
            ParseError::IllegalChrInComment { pos, chr } => {
                write!(f, "illegal character {:?} in comment at byte {}", chr, pos)
            }
            ParseError::UnfinishedString { pos } => write!(f, "unfinished string at byte {}", pos),
            ParseError::UnexpectedToken { pos, ref token } => {
                write!(f, "unexpected token {:?} at byte {}", token, pos)
            }
            ParseError::ExpectedEof { pos } => write!(f, "expected end-of-file at byte {}", pos),
        }
    }
}

#[cfg(feature = "std")]
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
/// let data = "(test (1 2 3))";
/// let mut parser = sise::Parser::new(data);
/// assert_eq!(
///     parser.read().unwrap().kind,
///     sise::ReadItemKind::ListBeginning,
/// );
/// assert_eq!(
///     parser.read().unwrap().kind,
///     sise::ReadItemKind::Atom("test"),
/// );
/// assert_eq!(
///     parser.read().unwrap().kind,
///     sise::ReadItemKind::ListBeginning,
/// );
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
    pub fn new(data: &'a str) -> Self {
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
    input_str: &'a str,
    char_iter: core::str::CharIndices<'a>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            input_str: input,
            char_iter: input.char_indices(),
        }
    }

    fn get_token(&mut self) -> Result<(BytePos, Token<'a>), ParseError> {
        loop {
            match self.char_iter.next() {
                None => {
                    // end-of-file
                    return Ok((BytePos(self.input_str.len()), Token::Eof));
                }
                Some((chr_pos, chr)) => match chr {
                    // skip whitespace
                    ' ' | '\t' | '\n' | '\r' => {}
                    // skip comments
                    ';' => {
                        for (chr_pos, chr) in &mut self.char_iter {
                            match chr {
                                '\n' | '\r' => break,
                                '\t' | ' '..='~' => {}
                                chr => {
                                    return Err(ParseError::IllegalChrInComment {
                                        chr,
                                        pos: BytePos(chr_pos),
                                    })
                                }
                            }
                        }
                    }
                    // delimiters
                    '(' => return Ok((BytePos(chr_pos), Token::LeftParen)),
                    ')' => return Ok((BytePos(chr_pos), Token::RightParen)),
                    // atom
                    chr if is_atom_chr(chr) || chr == '"' => {
                        let begin_pos = chr_pos;
                        let end_pos = self.lex_atom(chr)?;
                        let atom = &self.input_str[begin_pos..end_pos];
                        return Ok((BytePos(begin_pos), Token::Atom(atom)));
                    }
                    // invalid character
                    chr => {
                        return Err(ParseError::IllegalChr {
                            chr,
                            pos: BytePos(chr_pos),
                        });
                    }
                },
            }
        }
    }

    fn lex_atom(&mut self, first_chr: char) -> Result<usize, ParseError> {
        enum State {
            Normal,
            String,
            StringBackslash,
        }

        let mut state = if first_chr == '"' {
            State::String
        } else {
            State::Normal
        };
        loop {
            let saved_iter = self.char_iter.clone();
            match state {
                State::Normal => match self.char_iter.next() {
                    None => return Ok(self.input_str.len()),
                    Some((chr_pos, chr)) => match chr {
                        '"' => state = State::String,
                        chr if is_atom_chr(chr) => {}
                        _ => {
                            self.char_iter = saved_iter;
                            return Ok(chr_pos);
                        }
                    },
                },
                State::String => match self.char_iter.next() {
                    None => {
                        return Err(ParseError::UnfinishedString {
                            pos: BytePos(self.input_str.len()),
                        })
                    }
                    Some((chr_pos, chr)) => match chr {
                        '"' => state = State::Normal,
                        '\\' => state = State::StringBackslash,
                        chr if is_atom_string_chr(chr) => {}
                        chr => {
                            return Err(ParseError::IllegalChrInString {
                                chr,
                                pos: BytePos(chr_pos),
                            })
                        }
                    },
                },
                State::StringBackslash => match self.char_iter.next() {
                    None => {
                        return Err(ParseError::UnfinishedString {
                            pos: BytePos(self.input_str.len()),
                        })
                    }
                    Some((chr_pos, chr)) => match chr {
                        chr if is_atom_string_chr(chr) || chr == '"' || chr == '\\' => {
                            state = State::String
                        }
                        chr => {
                            return Err(ParseError::IllegalChrInString {
                                chr,
                                pos: BytePos(chr_pos),
                            })
                        }
                    },
                },
            }
        }
    }
}

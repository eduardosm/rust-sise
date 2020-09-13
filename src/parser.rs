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
    input_iter: core::str::CharIndices<'a>,
    current_chr: Option<char>,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut input_iter = input.char_indices();
        let first_chr = input_iter.next();
        Lexer {
            input_str: input,
            input_iter,
            current_chr: first_chr.map(|(_, chr)| chr),
            current_pos: 0,
        }
    }

    fn next_char(&mut self) {
        match self.input_iter.next() {
            None => {
                self.current_pos = self.input_str.len();
                self.current_chr = None;
            }
            Some((byte_index, chr)) => {
                self.current_chr = Some(chr);
                self.current_pos = byte_index;
            }
        }
    }

    fn get_token(&mut self) -> Result<(BytePos, Token<'a>), ParseError> {
        loop {
            match self.current_chr {
                // end-of-file
                None => {
                    return Ok((BytePos(self.current_pos), Token::Eof));
                }
                // skip whitespace
                Some(' ') | Some('\t') | Some('\n') | Some('\r') => {
                    self.next_char();
                }
                // skip comments
                Some(';') => {
                    self.next_char();
                    while let Some(chr) = self.current_chr {
                        match chr {
                            '\n' | '\r' => {
                                self.next_char();
                                break;
                            }
                            '\t' | ' '..='~' => self.next_char(),
                            chr => {
                                return Err(ParseError::IllegalChrInComment {
                                    chr,
                                    pos: BytePos(self.current_pos),
                                });
                            }
                        }
                    }
                }
                // delimiters
                Some('(') => {
                    let pos = BytePos(self.current_pos);
                    self.next_char();
                    return Ok((pos, Token::LeftParen));
                }
                Some(')') => {
                    let pos = BytePos(self.current_pos);
                    self.next_char();
                    return Ok((pos, Token::RightParen));
                }
                // atom
                Some(chr) if is_atom_chr(chr) || chr == '"' => {
                    let begin_pos = self.current_pos;
                    self.next_char();
                    self.lex_atom(chr)?;
                    let end_pos = self.current_pos;
                    let atom = &self.input_str[begin_pos..end_pos];
                    return Ok((BytePos(begin_pos), Token::Atom(atom)));
                }
                // invalid character
                Some(chr) => {
                    return Err(ParseError::IllegalChr {
                        chr,
                        pos: BytePos(self.current_pos),
                    });
                }
            }
        }
    }

    fn lex_atom(&mut self, first_chr: char) -> Result<(), ParseError> {
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
            match state {
                State::Normal => match self.current_chr {
                    Some('"') => {
                        self.next_char();
                        state = State::String;
                    }
                    Some(chr) if is_atom_chr(chr) => {
                        self.next_char();
                    }
                    Some(_) => {
                        return Ok(());
                    }
                    None => {
                        return Ok(());
                    }
                },
                State::String => match self.current_chr {
                    Some('"') => {
                        self.next_char();
                        state = State::Normal;
                    }
                    Some('\\') => {
                        self.next_char();
                        state = State::StringBackslash;
                    }
                    Some(chr) if is_atom_string_chr(chr) => {
                        self.next_char();
                        state = State::String;
                    }
                    Some(chr) => {
                        return Err(ParseError::IllegalChrInString {
                            chr,
                            pos: BytePos(self.current_pos),
                        });
                    }
                    None => {
                        return Err(ParseError::UnfinishedString {
                            pos: BytePos(self.current_pos),
                        });
                    }
                },
                State::StringBackslash => match self.current_chr {
                    Some(chr) if is_atom_string_chr(chr) || chr == '"' || chr == '\\' => {
                        self.next_char();
                        state = State::String;
                    }
                    Some(chr) => {
                        return Err(ParseError::IllegalChrInString {
                            chr,
                            pos: BytePos(self.current_pos),
                        });
                    }
                    None => {
                        return Err(ParseError::UnfinishedString {
                            pos: BytePos(self.current_pos),
                        });
                    }
                },
            }
        }
    }
}

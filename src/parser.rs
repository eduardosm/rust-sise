// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::is_atom_chr;
use crate::is_atom_string_chr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ParsedItem<'a> {
    /// An atom
    ///
    /// The `usize` specifies its byte offset in the input file
    Atom(&'a str, usize),
    /// The start of a list (`(`)
    ///
    /// The `usize` specifies its byte offset in the input file
    ListStart(usize),
    /// The end of a list (`)`)
    ///
    /// The `usize` specifies its byte offset in the input file
    ListEnd(usize),
}

/// Represents a parse error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    /// There is an invalid character
    IllegalChr { pos: usize, chr: char },

    /// There is an invalid character inside a string (enclosed with `"`)
    IllegalChrInString { pos: usize, chr: char },

    /// There is an invalid character inside a comment
    IllegalChrInComment { pos: usize, chr: char },

    /// End-of-file is reached before finding the closing `"`
    UnfinishedString { pos: usize },

    /// Unexpected end-of-file
    UnexpectedEof { pos: usize },

    /// Unexpected `)`
    UnexpectedRightParen { pos: usize },

    /// Found a token when expecting end-of-file
    ExpectedEof { pos: usize },
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
            ParseError::UnexpectedEof { pos } => {
                write!(f, "unexpected end-of-file at byte {}", pos)
            }
            ParseError::UnexpectedRightParen { pos } => {
                write!(f, "unexpected `)` at byte {}", pos)
            }
            ParseError::ExpectedEof { pos } => write!(f, "expected end-of-file at byte {}", pos),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseError {}

/// Parser that decodes a SISE file from memory.
///
/// # Example
///
/// ```
/// let data = "(test (1 2 3))";
/// let mut parser = sise::Parser::new(data);
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::ListStart(0));
/// assert_eq!(
///     parser.next_item().unwrap(),
///     sise::ParsedItem::Atom("test", 1),
/// );
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::ListStart(6));
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::Atom("1", 7));
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::Atom("2", 9));
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::Atom("3", 11));
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::ListEnd(12));
/// assert_eq!(parser.next_item().unwrap(), sise::ParsedItem::ListEnd(13));
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

    pub fn next_item(&mut self) -> Result<ParsedItem<'a>, ParseError> {
        match self.state {
            State::Beginning => {
                let (pos, token) = self.lexer.get_token()?;
                match token {
                    Token::Eof => Err(ParseError::UnexpectedEof { pos }),
                    Token::LeftParen => {
                        self.state = State::Parsing { depth: 0 };
                        Ok(ParsedItem::ListStart(pos))
                    }
                    Token::RightParen => Err(ParseError::UnexpectedRightParen { pos }),
                    Token::Atom(atom) => {
                        self.state = State::Finishing;
                        Ok(ParsedItem::Atom(atom, pos))
                    }
                }
            }
            State::Parsing { ref mut depth } => {
                let (pos, token) = self.lexer.get_token()?;
                match token {
                    Token::Eof => Err(ParseError::UnexpectedEof { pos }),
                    Token::LeftParen => {
                        *depth += 1;
                        Ok(ParsedItem::ListStart(pos))
                    }
                    Token::RightParen => {
                        if *depth == 0 {
                            self.state = State::Finishing;
                        } else {
                            *depth -= 1;
                        }
                        Ok(ParsedItem::ListEnd(pos))
                    }
                    Token::Atom(atom) => Ok(ParsedItem::Atom(atom, pos)),
                }
            }
            State::Finishing => panic!("parsing finished"),
        }
    }

    pub fn finish(mut self) -> Result<(), ParseError> {
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token<'a> {
    Eof,
    LeftParen,
    RightParen,
    Atom(&'a str),
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

    fn get_token(&mut self) -> Result<(usize, Token<'a>), ParseError> {
        loop {
            match self.char_iter.next() {
                None => {
                    // end-of-file
                    return Ok((self.input_str.len(), Token::Eof));
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
                                        pos: chr_pos,
                                    })
                                }
                            }
                        }
                    }
                    // delimiters
                    '(' => return Ok((chr_pos, Token::LeftParen)),
                    ')' => return Ok((chr_pos, Token::RightParen)),
                    // atom
                    chr if is_atom_chr(chr) || chr == '"' => {
                        let begin_pos = chr_pos;
                        let end_pos = self.lex_atom(chr)?;
                        let atom = &self.input_str[begin_pos..end_pos];
                        return Ok((begin_pos, Token::Atom(atom)));
                    }
                    // invalid character
                    chr => {
                        return Err(ParseError::IllegalChr { chr, pos: chr_pos });
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
                            pos: self.input_str.len(),
                        })
                    }
                    Some((chr_pos, chr)) => match chr {
                        '"' => state = State::Normal,
                        '\\' => state = State::StringBackslash,
                        chr if is_atom_string_chr(chr) => {}
                        chr => return Err(ParseError::IllegalChrInString { chr, pos: chr_pos }),
                    },
                },
                State::StringBackslash => match self.char_iter.next() {
                    None => {
                        return Err(ParseError::UnfinishedString {
                            pos: self.input_str.len(),
                        })
                    }
                    Some((chr_pos, chr)) => match chr {
                        chr if is_atom_string_chr(chr) || chr == '"' || chr == '\\' => {
                            state = State::String
                        }
                        chr => return Err(ParseError::IllegalChrInString { chr, pos: chr_pos }),
                    },
                },
            }
        }
    }
}

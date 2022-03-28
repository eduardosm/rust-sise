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

/// Parser that decodes a SISE file into a sequence of [`ParsedItem`].
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
    rem_input: &'a str,
    rem_offset: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            rem_input: input,
            rem_offset: 0,
        }
    }

    #[must_use]
    #[inline]
    fn eat_any_char(&mut self) -> Option<char> {
        let mut iter = self.rem_input.chars();
        if let Some(chr) = iter.next() {
            let new_rem = iter.as_str();
            self.rem_offset += self.rem_input.len() - new_rem.len();
            self.rem_input = new_rem;
            Some(chr)
        } else {
            None
        }
    }

    #[must_use]
    #[inline]
    fn eat_char(&mut self, chr: char) -> bool {
        if let Some(new_rem) = self.rem_input.strip_prefix(chr) {
            self.rem_offset += self.rem_input.len() - new_rem.len();
            self.rem_input = new_rem;
            true
        } else {
            false
        }
    }

    #[must_use]
    #[inline]
    fn eat_char_if(&mut self, pred: impl FnMut(char) -> bool) -> bool {
        if let Some(new_rem) = self.rem_input.strip_prefix(pred) {
            self.rem_offset += self.rem_input.len() - new_rem.len();
            self.rem_input = new_rem;
            true
        } else {
            false
        }
    }

    fn get_token(&mut self) -> Result<(usize, Token<'a>), ParseError> {
        loop {
            let start_str = self.rem_input;
            let chr_pos = self.rem_offset;
            if self.eat_char(' ')
                || self.eat_char('\t')
                || self.eat_char('\n')
                || self.eat_char('\r')
            {
                // skip whitespace
            } else if self.eat_char(';') {
                // skip comments
                loop {
                    let chr_pos = self.rem_offset;
                    match self.eat_any_char() {
                        None => return Ok((self.rem_offset, Token::Eof)),
                        Some('\n' | '\r') => break,
                        Some('\t' | ' '..='~') => {}
                        Some(chr) => {
                            return Err(ParseError::IllegalChrInComment { chr, pos: chr_pos });
                        }
                    }
                }
            } else if self.eat_char('(') {
                return Ok((chr_pos, Token::LeftParen));
            } else if self.eat_char(')') {
                return Ok((chr_pos, Token::RightParen));
            } else if let Some(chr) = self.eat_any_char() {
                if is_atom_chr(chr) || chr == '"' {
                    let begin_pos = chr_pos;
                    let end_pos = self.lex_atom(chr)?;
                    let atom = &start_str[..(end_pos - begin_pos)];
                    return Ok((begin_pos, Token::Atom(atom)));
                } else {
                    // invalid character
                    return Err(ParseError::IllegalChr { chr, pos: chr_pos });
                }
            } else {
                // end-of-file
                return Ok((self.rem_offset, Token::Eof));
            }
        }
    }

    fn lex_atom(&mut self, first_chr: char) -> Result<usize, ParseError> {
        let mut in_string = first_chr == '"';
        loop {
            let chr_pos = self.rem_offset;
            if in_string {
                if self.eat_char('"') {
                    in_string = false;
                } else if self.eat_char('\\') {
                    let chr_pos = self.rem_offset;
                    if let Some(chr) = self.eat_any_char() {
                        if chr != '"' && chr != '\\' && !is_atom_string_chr(chr) {
                            return Err(ParseError::IllegalChrInString { chr, pos: chr_pos });
                        }
                    } else {
                        return Err(ParseError::UnfinishedString { pos: chr_pos });
                    }
                } else if let Some(chr) = self.eat_any_char() {
                    if !is_atom_string_chr(chr) {
                        return Err(ParseError::IllegalChrInString { chr, pos: chr_pos });
                    }
                } else {
                    return Err(ParseError::UnfinishedString { pos: chr_pos });
                }
            } else if self.eat_char('"') {
                in_string = true;
            } else if !self.eat_char_if(is_atom_chr) {
                return Ok(chr_pos);
            }
        }
    }
}

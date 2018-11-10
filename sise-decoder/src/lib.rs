// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate sise;

#[cfg(test)]
mod tests;

/// Structure to specify parsing limits.
#[derive(Clone, Debug)]
pub struct Limits {
    /// Maximum list nesting depth.
    max_depth: usize,

    /// Maximum atom length.
    max_atom_len: usize,

    /// Maximum number of elements in a list.
    max_list_len: usize,
}

impl Limits {
    /// Creates a `Limits` instance with all fields set to maximum.
    #[inline]
    pub fn unlimited() -> Self {
        Limits {
            max_depth: usize::max_value(),
            max_atom_len: usize::max_value(),
            max_list_len: usize::max_value(),
        }
    }

    /// Modifies the value of `max_depth` in `self` and returns
    /// the modified `self`.
    #[inline]
    pub fn max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    /// Modifies the value of `max_atom_len` in `self` and returns
    /// the modified `self`.
    #[inline]
    pub fn max_atom_len(mut self, max_atom_len: usize) -> Self {
        self.max_atom_len = max_atom_len;
        self
    }

    /// Modifies the value of `max_list_len` in `self` and returns
    /// the modified `self`.
    #[inline]
    pub fn max_list_len(mut self, max_list_len: usize) -> Self {
        self.max_list_len = max_list_len;
        self
    }
}

/// Represents a parse error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// There is an invalid character.
    IllegalChr {
        pos: sise::Pos,
        chr: u8,
    },

    /// There is an invalid character inside a string (enclosed with `"`).
    IllegalChrInString {
        pos: sise::Pos,
        chr: u8,
    },

    /// There is an invalid character inside a comment.
    IllegalChrInComment {
        pos: sise::Pos,
        chr: u8,
    },

    /// End-of-file is reached before finding the closing `"`.
    UnfinishedString {
        pos: sise::Pos,
    },

    /// Unexpected token.
    UnexpectedToken {
        pos: sise::Pos,
        token: Token,
    },

    /// Found a token when expecting end-of-file.
    ExpectedEof {
        pos: sise::Pos,
    },

    /// A line is longer than `u32::max_value()`.
    LineTooLong {
        line: u32,
    },

    /// There are more than `u32::max_value()` lines.
    TooManyLines,

    /// Maximum specified list nesting depth exceeded.
    TooDeep {
        pos: sise::Pos,
    },

    /// Maximum atom length exceeded.
    AtomTooLong {
        pos: sise::Pos,
    },

    /// Maximum number of list elements exceeded.
    ListTooLong {
        pos: sise::Pos,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::IllegalChr { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} at {}:{}",
                       chr, sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
            Error::IllegalChrInString { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} in string at {}:{}",
                       chr, sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
            Error::IllegalChrInComment { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} in comment at {}:{}",
                       chr, sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
            Error::UnfinishedString { pos } => {
                write!(f, "Unfinished string at {}:{}",
                       sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
            Error::UnexpectedToken { pos, ref token } => {
                write!(f, "Unexpected token {:?} at {}:{}",
                       token,
                       sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
            Error::ExpectedEof { pos } => {
                write!(f, "Expected end-of-file at {}:{}",
                       sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
            Error::LineTooLong { line } => {
                write!(f, "Line {} too long", sise::ReprPosValue(line))
            }
            Error::TooManyLines => {
                write!(f, "Too many lines")
            }
            Error::TooDeep { pos } => {
                write!(f, "Maximum depth exceeded at {}:{}",
                       sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
            Error::AtomTooLong { pos } => {
                write!(f, "Atom too long at {}:{}",
                       sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
            Error::ListTooLong { pos } => {
                write!(f, "List too long at {}:{}",
                       sise::ReprPosValue(pos.line),
                       sise::ReprPosValue(pos.column))
            }
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IllegalChr { .. } => "Illegal character",
            Error::IllegalChrInString { .. } => "Illegal character in string",
            Error::IllegalChrInComment { .. } => "Illegal character in comment",
            Error::UnfinishedString { .. } => "Unfinished string",
            Error::UnexpectedToken { .. } => "Unexpected token",
            Error::ExpectedEof { .. } => "Expected end-of-file",
            Error::LineTooLong { .. } => "Line too long",
            Error::TooManyLines => "Too many lines",
            Error::TooDeep { .. } => "Maximum depth exceeded",
            Error::AtomTooLong { .. } => "Atom too long",
            Error::ListTooLong { .. } => "List too long",
        }
    }
}

/// Parses `data`. If successful, it returns the root node and a position
/// map that allows to fetch the position (line and column) of each node.
///
/// # Example
///
/// ```
/// let data = b"(test (1 2 3))";
/// let limits = sise_decoder::Limits::unlimited();
/// match sise_decoder::parse(data, &limits) {
///     Ok((root_node, pos_map)) => {
///         println!("{:?}", root_node);
///     }
///     Err(e) => {
///         panic!("Error: {}", e);
///     }
/// }
/// ```
pub fn parse(data: &[u8], limits: &Limits) -> Result<(Box<sise::Node>, sise::PosMap), Error> {
    assert!(limits.max_atom_len >= 1);

    let mut lexer = Lexer::new(data.iter().map(|&c| c).peekable());

    enum State {
        Beginning,
        List(sise::Pos, Vec<Box<sise::Node>>),
        Finishing(Box<sise::Node>),
    }

    enum StackItem {
        List(sise::Pos, Vec<Box<sise::Node>>),
    }

    let mut state = State::Beginning;
    let mut stack = Vec::new();
    let mut depth = 0;
    let mut pos_map = sise::PosMap::new();

    loop {
        let (token_pos, token) = lexer.get_token(limits.max_atom_len)?;
        match state {
            State::Beginning => {
                match token {
                    Token::LeftParen => {
                        if depth == limits.max_depth {
                            return Err(Error::TooDeep { pos: token_pos });
                        }
                        state = State::List(token_pos, Vec::new());
                        depth += 1;
                    }
                    Token::Atom(atom) => {
                        let root_node = Box::new(sise::Node::Atom(atom));
                        pos_map.set_pos(&root_node, token_pos);
                        state = State::Finishing(root_node);
                    }
                    token => {
                        return Err(Error::UnexpectedToken { pos: token_pos, token: token });
                    }
                }
            }
            State::List(list_node_pos, mut list) => {
                match token {
                    Token::LeftParen => {
                        if list.len() == limits.max_list_len {
                            return Err(Error::ListTooLong { pos: token_pos });
                        }
                        if depth == limits.max_depth {
                            return Err(Error::TooDeep { pos: token_pos });
                        }

                        stack.push(StackItem::List(list_node_pos, list));
                        state = State::List(token_pos, Vec::new());
                        depth += 1;
                    }
                    Token::RightParen => {
                        let node = Box::new(sise::Node::List(list));
                        pos_map.set_pos(&node, list_node_pos);
                        depth -= 1;
                        match stack.pop() {
                            Some(StackItem::List(parent_node_pos, mut parent_list)) => {
                                parent_list.push(node);
                                state = State::List(parent_node_pos, parent_list);
                            }
                            None => {
                                state = State::Finishing(node);
                            }
                        }
                    }
                    Token::Atom(atom) => {
                        if list.len() == limits.max_list_len {
                            return Err(Error::ListTooLong { pos: token_pos });
                        }

                        let atom_node = Box::new(sise::Node::Atom(atom));
                        pos_map.set_pos(&atom_node, token_pos);
                        list.push(atom_node);
                        state = State::List(list_node_pos, list);
                    }
                    _ => {
                        return Err(Error::UnexpectedToken { pos: token_pos, token: token });
                    }
                }
            }
            State::Finishing(root_node) => {
                assert!(stack.is_empty());
                assert_eq!(depth, 0);
                match token {
                    Token::Eof => {
                        return Ok((root_node, pos_map));
                    }
                    _ => {
                        return Err(Error::ExpectedEof { pos: token_pos });
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token {
    Eof,
    LeftParen,
    RightParen,
    Atom(String),
}

impl Token {
    #[inline]
    pub fn is_eof(&self) -> bool {
        match *self {
            Token::Eof => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_left_paren(&self) -> bool {
        match *self {
            Token::LeftParen => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_right_paren(&self) -> bool {
        match *self {
            Token::RightParen => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_atom(&self) -> bool {
        match *self {
            Token::Atom(..) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn into_atom(self) -> Option<String> {
        match self {
            Token::Atom(atom) => Some(atom),
            _ => None,
        }
    }
}

struct Lexer<I: Iterator<Item=u8>> {
    line: u32,
    column: u32,
    iter: std::iter::Peekable<I>,
}

impl<I: Iterator<Item=u8>> Lexer<I> {
    #[inline]
    fn new(iter: std::iter::Peekable<I>) -> Self {
        Lexer {
            line: 0,
            column: 0,
            iter: iter,
        }
    }

    #[inline]
    fn get_pos(&self) -> sise::Pos {
        sise::Pos::new(self.line, self.column)
    }

    #[inline]
    fn incr_line(&mut self) -> Result<(), Error> {
        self.line = self.line.checked_add(1).ok_or(Error::LineTooLong { line: self.line })?;
        self.column = 0;
        Ok(())
    }

    #[inline]
    fn incr_column(&mut self) -> Result<(), Error> {
        self.column = self.column.checked_add(1).ok_or(Error::TooManyLines)?;
        Ok(())
    }

    fn get_token(&mut self, max_atom_len: usize) -> Result<(sise::Pos, Token), Error> {
        enum State {
            Beginning,
            Comment,
            Atom(sise::Pos, String),
            AtomString(sise::Pos, String),
            AtomStringBackslash(sise::Pos, String),
        }

        let mut state = State::Beginning;

        loop {
            match state {
                State::Beginning => {
                    let chr = self.iter.next();
                    match chr {
                        Some(b' ') | Some(b'\t') => {
                            self.incr_column()?
                        }
                        Some(b'\n') => {
                            self.incr_line()?
                        }
                        Some(b'\r') => {
                            if self.iter.peek().map(|&c| c) == Some(b'\n') {
                                self.iter.next();
                            }
                            self.incr_line()?;
                        }
                        Some(b';') => {
                            self.incr_column()?;
                            state = State::Comment;
                        }
                        Some(b'(') => {
                            let pos = self.get_pos();
                            self.incr_column()?;
                            return Ok((pos, Token::LeftParen));
                        }
                        Some(b')') => {
                            let pos = self.get_pos();
                            self.incr_column()?;
                            return Ok((pos, Token::RightParen));
                        }
                        Some(b'"') => {
                            let pos = self.get_pos();
                            self.incr_column()?;
                            state = State::AtomString(pos, String::from("\""));
                        }
                        Some(c) if sise::is_atom_chr(c) => {
                            let pos = self.get_pos();
                            self.incr_column()?;
                            let mut atom = String::new();
                            atom.push(c as char);
                            state = State::Atom(pos, atom);
                        }
                        Some(c) => {
                            return Err(Error::IllegalChr { chr: c, pos: self.get_pos() });
                        }
                        None => {
                            return Ok((self.get_pos(), Token::Eof));
                        }
                    }
                },
                State::Comment => {
                    let chr = self.iter.next();
                    match chr {
                        Some(b'\n') => {
                            self.incr_line()?;
                            state = State::Beginning;
                        }
                        Some(b'\r') => {
                            if self.iter.peek().map(|&c| c) == Some(b'\n') {
                                self.iter.next();
                            }
                            self.incr_line()?;
                            state = State::Beginning;
                        }
                        Some(c) if c.is_ascii_graphic() || c == b' ' || c == b'\t' => {
                            self.incr_column()?;
                        }
                        Some(c) => {
                            return Err(Error::IllegalChrInComment { chr: c, pos: self.get_pos() });
                        }
                        None => {
                            return Ok((self.get_pos(), Token::Eof));
                        }
                    }
                }
                State::Atom(pos, mut atom) => {
                    let chr = self.iter.peek().map(|&c| c);
                    match chr {
                        Some(b'"') => {
                            if atom.len() == max_atom_len {
                                return Err(Error::AtomTooLong { pos: pos });
                            }

                            self.iter.next();
                            self.incr_column()?;
                            atom.push('"');
                            state = State::AtomString(pos, atom);
                        }
                        Some(c) if sise::is_atom_chr(c) => {
                            if atom.len() == max_atom_len {
                                return Err(Error::AtomTooLong { pos: pos });
                            }

                            self.iter.next();
                            self.incr_column()?;
                            atom.push(c as char);
                            state = State::Atom(pos, atom);
                        }
                        _ => {
                            return Ok((pos, Token::Atom(atom)));
                        }
                    }
                }
                State::AtomString(pos, mut atom) => {
                    let chr = self.iter.next();
                    match chr {
                        Some(b'"') => {
                            if atom.len() == max_atom_len {
                                return Err(Error::AtomTooLong { pos: pos });
                            }

                            self.incr_column()?;
                            atom.push('"');
                            state = State::Atom(pos, atom);
                        }
                        Some(b'\\') => {
                            if atom.len() == max_atom_len {
                                return Err(Error::AtomTooLong { pos: pos });
                            }

                            self.incr_column()?;
                            atom.push('\\');
                            state = State::AtomStringBackslash(pos, atom);
                        }
                        Some(c) if sise::is_atom_string_chr(c) => {
                            if atom.len() == max_atom_len {
                                return Err(Error::AtomTooLong { pos: pos });
                            }

                            self.incr_column()?;
                            atom.push(c as char);
                            state = State::AtomString(pos, atom);
                        }
                        Some(c) => {
                            return Err(Error::IllegalChrInString { chr: c, pos: self.get_pos() });
                        }
                        None => {
                            return Err(Error::UnfinishedString { pos: self.get_pos() });
                        }
                    }
                }
                State::AtomStringBackslash(pos, mut atom) => {
                    let chr = self.iter.next();
                    match chr {
                        Some(c) if sise::is_atom_string_chr(c) || c == b'"' || c == b'\\' => {
                            if atom.len() == max_atom_len {
                                return Err(Error::AtomTooLong { pos: pos });
                            }

                            self.incr_column()?;
                            atom.push(c as char);
                            state = State::AtomString(pos, atom);
                        }
                        Some(c) => {
                            return Err(Error::IllegalChrInString { chr: c, pos: self.get_pos() });
                        }
                        None => {
                            return Err(Error::UnfinishedString { pos: self.get_pos() });
                        }
                    }
                }
            }
        }
    }
}

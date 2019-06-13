// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::Pos;
use crate::ReprPosValue;
use crate::PosTree;
use crate::Node;
use crate::is_atom_chr;
use crate::is_atom_string_chr;

/// Structure to specify parsing limits.
#[derive(Clone, Debug)]
pub struct ParseLimits {
    /// Maximum list nesting depth.
    max_depth: usize,

    /// Maximum atom length.
    max_atom_len: usize,

    /// Maximum number of elements in a list.
    max_list_len: usize,
}

impl ParseLimits {
    /// Creates a `ParseLimits` instance with all fields set to maximum.
    #[inline]
    pub fn unlimited() -> Self {
        ParseLimits {
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
        token: Token,
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

    /// Maximum specified list nesting depth exceeded.
    TooDeep {
        pos: Pos,
    },

    /// Maximum atom length exceeded.
    AtomTooLong {
        pos: Pos,
    },

    /// Maximum number of list elements exceeded.
    ListTooLong {
        pos: Pos,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ParseError::IllegalChr { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} at {}:{}",
                       chr, ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::IllegalChrInString { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} in string at {}:{}",
                       chr, ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::IllegalChrInComment { pos, chr } => {
                write!(f, "Illegal character 0x{:02X} in comment at {}:{}",
                       chr, ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::UnfinishedString { pos } => {
                write!(f, "Unfinished string at {}:{}",
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::UnexpectedToken { pos, ref token } => {
                write!(f, "Unexpected token {:?} at {}:{}",
                       token,
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::ExpectedEof { pos } => {
                write!(f, "Expected end-of-file at {}:{}",
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::LineTooLong { line } => {
                write!(f, "Line {} too long", ReprPosValue(line))
            }
            ParseError::TooManyLines => {
                write!(f, "Too many lines")
            }
            ParseError::TooDeep { pos } => {
                write!(f, "Maximum depth exceeded at {}:{}",
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::AtomTooLong { pos } => {
                write!(f, "Atom too long at {}:{}",
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
            ParseError::ListTooLong { pos } => {
                write!(f, "List too long at {}:{}",
                       ReprPosValue(pos.line),
                       ReprPosValue(pos.column))
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::IllegalChr { .. } => "Illegal character",
            ParseError::IllegalChrInString { .. } => "Illegal character in string",
            ParseError::IllegalChrInComment { .. } => "Illegal character in comment",
            ParseError::UnfinishedString { .. } => "Unfinished string",
            ParseError::UnexpectedToken { .. } => "Unexpected token",
            ParseError::ExpectedEof { .. } => "Expected end-of-file",
            ParseError::LineTooLong { .. } => "Line too long",
            ParseError::TooManyLines => "Too many lines",
            ParseError::TooDeep { .. } => "Maximum depth exceeded",
            ParseError::AtomTooLong { .. } => "Atom too long",
            ParseError::ListTooLong { .. } => "List too long",
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
/// let limits = sise::ParseLimits::unlimited();
/// match sise::parse(data, &limits) {
///     Ok((root_node, pos_tree)) => {
///         println!("{:?}", root_node);
///     }
///     Err(e) => {
///         panic!("Error: {}", e);
///     }
/// }
/// ```
pub fn parse(data: &[u8], limits: &ParseLimits) -> Result<(Node, PosTree), ParseError> {
    assert!(limits.max_atom_len >= 1);

    let mut lexer = Lexer::new(data.iter().cloned().peekable());

    enum State {
        Beginning,
        List(PosTree, Vec<Node>),
        Finishing(PosTree, Node),
    }

    enum StackItem {
        List(PosTree, Vec<Node>),
    }

    let mut state = State::Beginning;
    let mut stack = Vec::new();
    let mut depth = 0;

    loop {
        let (token_pos, token) = lexer.get_token(limits.max_atom_len)?;
        match state {
            State::Beginning => {
                match token {
                    Token::LeftParen => {
                        if depth == limits.max_depth {
                            return Err(ParseError::TooDeep { pos: token_pos });
                        }
                        state = State::List(PosTree::new(token_pos), Vec::new());
                        depth += 1;
                    }
                    Token::Atom(atom) => {
                        let root_node = Node::Atom(atom);
                        state = State::Finishing(PosTree::new(token_pos), root_node);
                    }
                    token => {
                        return Err(ParseError::UnexpectedToken { pos: token_pos, token });
                    }
                }
            }
            State::List(mut list_node_pos_tree, mut list) => {
                match token {
                    Token::LeftParen => {
                        if list.len() == limits.max_list_len {
                            return Err(ParseError::ListTooLong { pos: token_pos });
                        }
                        if depth == limits.max_depth {
                            return Err(ParseError::TooDeep { pos: token_pos });
                        }

                        stack.push(StackItem::List(list_node_pos_tree, list));
                        state = State::List(PosTree::new(token_pos), Vec::new());
                        depth += 1;
                    }
                    Token::RightParen => {
                        let node = Node::List(list);
                        depth -= 1;
                        match stack.pop() {
                            Some(StackItem::List(mut parent_node_pos_tree, mut parent_list)) => {
                                parent_node_pos_tree.children.push(list_node_pos_tree);
                                parent_list.push(node);
                                state = State::List(parent_node_pos_tree, parent_list);
                            }
                            None => {
                                state = State::Finishing(list_node_pos_tree, node);
                            }
                        }
                    }
                    Token::Atom(atom) => {
                        if list.len() == limits.max_list_len {
                            return Err(ParseError::ListTooLong { pos: token_pos });
                        }

                        let atom_node = Node::Atom(atom);
                        list_node_pos_tree.children.push(PosTree::new(token_pos));
                        list.push(atom_node);
                        state = State::List(list_node_pos_tree, list);
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken { pos: token_pos, token });
                    }
                }
            }
            State::Finishing(pos_tree, root_node) => {
                assert!(stack.is_empty());
                assert_eq!(depth, 0);
                match token {
                    Token::Eof => {
                        return Ok((root_node, pos_tree));
                    }
                    _ => {
                        return Err(ParseError::ExpectedEof { pos: token_pos });
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
            iter,
        }
    }

    #[inline]
    fn get_pos(&self) -> Pos {
        Pos::new(self.line, self.column)
    }

    #[inline]
    fn incr_line(&mut self) -> Result<(), ParseError> {
        self.line = self.line.checked_add(1).ok_or(ParseError::LineTooLong { line: self.line })?;
        self.column = 0;
        Ok(())
    }

    #[inline]
    fn incr_column(&mut self) -> Result<(), ParseError> {
        self.column = self.column.checked_add(1).ok_or(ParseError::TooManyLines)?;
        Ok(())
    }

    #[allow(clippy::cognitive_complexity)]
    fn get_token(&mut self, max_atom_len: usize) -> Result<(Pos, Token), ParseError> {
        enum State {
            Beginning,
            Comment,
            Atom(Pos, String),
            AtomString(Pos, String),
            AtomStringBackslash(Pos, String),
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
                            if self.iter.peek().cloned() == Some(b'\n') {
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
                        Some(c) if is_atom_chr(c) => {
                            let pos = self.get_pos();
                            self.incr_column()?;
                            let mut atom = String::new();
                            atom.push(c as char);
                            state = State::Atom(pos, atom);
                        }
                        Some(c) => {
                            return Err(ParseError::IllegalChr { chr: c, pos: self.get_pos() });
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
                            if self.iter.peek().cloned() == Some(b'\n') {
                                self.iter.next();
                            }
                            self.incr_line()?;
                            state = State::Beginning;
                        }
                        Some(c) if c.is_ascii_graphic() || c == b' ' || c == b'\t' => {
                            self.incr_column()?;
                        }
                        Some(c) => {
                            return Err(ParseError::IllegalChrInComment { chr: c, pos: self.get_pos() });
                        }
                        None => {
                            return Ok((self.get_pos(), Token::Eof));
                        }
                    }
                }
                State::Atom(pos, mut atom) => {
                    let chr = self.iter.peek().cloned();
                    match chr {
                        Some(b'"') => {
                            if atom.len() == max_atom_len {
                                return Err(ParseError::AtomTooLong { pos });
                            }

                            self.iter.next();
                            self.incr_column()?;
                            atom.push('"');
                            state = State::AtomString(pos, atom);
                        }
                        Some(c) if is_atom_chr(c) => {
                            if atom.len() == max_atom_len {
                                return Err(ParseError::AtomTooLong { pos });
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
                                return Err(ParseError::AtomTooLong { pos });
                            }

                            self.incr_column()?;
                            atom.push('"');
                            state = State::Atom(pos, atom);
                        }
                        Some(b'\\') => {
                            if atom.len() == max_atom_len {
                                return Err(ParseError::AtomTooLong { pos });
                            }

                            self.incr_column()?;
                            atom.push('\\');
                            state = State::AtomStringBackslash(pos, atom);
                        }
                        Some(c) if is_atom_string_chr(c) => {
                            if atom.len() == max_atom_len {
                                return Err(ParseError::AtomTooLong { pos });
                            }

                            self.incr_column()?;
                            atom.push(c as char);
                            state = State::AtomString(pos, atom);
                        }
                        Some(c) => {
                            return Err(ParseError::IllegalChrInString { chr: c, pos: self.get_pos() });
                        }
                        None => {
                            return Err(ParseError::UnfinishedString { pos: self.get_pos() });
                        }
                    }
                }
                State::AtomStringBackslash(pos, mut atom) => {
                    let chr = self.iter.next();
                    match chr {
                        Some(c) if is_atom_string_chr(c) || c == b'"' || c == b'\\' => {
                            if atom.len() == max_atom_len {
                                return Err(ParseError::AtomTooLong { pos });
                            }

                            self.incr_column()?;
                            atom.push(c as char);
                            state = State::AtomString(pos, atom);
                        }
                        Some(c) => {
                            return Err(ParseError::IllegalChrInString { chr: c, pos: self.get_pos() });
                        }
                        None => {
                            return Err(ParseError::UnfinishedString { pos: self.get_pos() });
                        }
                    }
                }
            }
        }
    }
}

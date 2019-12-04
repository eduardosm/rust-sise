// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::Pos;
use crate::ReadItemKind;
use crate::Reader;
use crate::ReprPosValue;

#[derive(Clone, Debug, PartialEq)]
pub enum ReadUtilError<E, P> {
    ReaderError(E),
    ExpectedAtom { pos: P },
    ExpectedListBeginning { pos: P },
    ExpectedListEnding { pos: P },
    ExpectedNodeInList { pos: P },
    InvalidValue { value_type: String, pos: P },
}

impl<E, P> From<E> for ReadUtilError<E, P> {
    #[inline]
    fn from(e: E) -> Self {
        ReadUtilError::ReaderError(e)
    }
}

impl<E: std::error::Error, P> ReadUtilError<E, P> {
    fn common_description(&self) -> &str {
        match self {
            ReadUtilError::ReaderError(e) => e.description(),
            ReadUtilError::ExpectedAtom { .. } => "expected atom",
            ReadUtilError::ExpectedListBeginning { .. } => "expected list beginning",
            ReadUtilError::ExpectedListEnding { .. } => "expected list ending",
            ReadUtilError::ExpectedNodeInList { .. } => "expected node in list",
            ReadUtilError::InvalidValue { .. } => "invalid value",
        }
    }
}

impl<E: std::fmt::Display> std::fmt::Display for ReadUtilError<E, ()> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadUtilError::ReaderError(e) => write!(f, "reader error: {}", e),
            ReadUtilError::ExpectedAtom { .. } => f.write_str("expected atom"),
            ReadUtilError::ExpectedListBeginning { .. } => f.write_str("expected list beginning"),
            ReadUtilError::ExpectedListEnding { .. } => f.write_str("expected list ending"),
            ReadUtilError::ExpectedNodeInList { .. } => f.write_str("expected node in list"),
            ReadUtilError::InvalidValue { value_type, .. } => {
                write!(f, "invalid value of type {:?}", value_type)
            }
        }
    }
}

impl<E: std::fmt::Display> std::fmt::Display for ReadUtilError<E, Pos> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadUtilError::ReaderError(e) => write!(f, "reader error: {}", e),
            ReadUtilError::ExpectedAtom { pos } => write!(
                f,
                "expected atom at {}:{}",
                ReprPosValue(pos.line),
                ReprPosValue(pos.column)
            ),
            ReadUtilError::ExpectedListBeginning { pos } => write!(
                f,
                "expected list beginning at {}:{}",
                ReprPosValue(pos.line),
                ReprPosValue(pos.column)
            ),
            ReadUtilError::ExpectedListEnding { pos } => write!(
                f,
                "expected list ending at {}:{}",
                ReprPosValue(pos.line),
                ReprPosValue(pos.column)
            ),
            ReadUtilError::ExpectedNodeInList { pos } => write!(
                f,
                "expected node in list at {}:{}",
                ReprPosValue(pos.line),
                ReprPosValue(pos.column)
            ),
            ReadUtilError::InvalidValue { value_type, pos } => write!(
                f,
                "invalid value of type {:?} at {}:{}",
                value_type,
                ReprPosValue(pos.line),
                ReprPosValue(pos.column)
            ),
        }
    }
}

impl<E: std::error::Error> std::error::Error for ReadUtilError<E, ()> {
    fn description(&self) -> &str {
        self.common_description()
    }
}

impl<E: std::error::Error> std::error::Error for ReadUtilError<E, Pos> {
    fn description(&self) -> &str {
        self.common_description()
    }
}

/// Utility to read nodes from a `Reader`.
pub enum NodeReadUtil<'a, R: Reader> {
    Atom(AtomReadUtil<R>),
    List(ListReadUtil<'a, R>),
}

impl<'a, R: Reader> NodeReadUtil<'a, R> {
    /// Reads from `reader` and builds a `ReadUtil::Atom` or
    /// `ReadUtil::List` according to the result. Panics if the
    /// reader returns `ReadItemKind::ListEnding`.
    pub fn new(reader: &'a mut R) -> Result<Self, R::Error> {
        let read_item = reader.read()?;
        match read_item.kind {
            ReadItemKind::Atom(s) => Ok(NodeReadUtil::Atom(AtomReadUtil::new(read_item.pos, s))),
            ReadItemKind::ListBeginning => {
                Ok(NodeReadUtil::List(ListReadUtil::new(read_item.pos, reader)))
            }
            ReadItemKind::ListEnding => panic!("unexpected ReadItemKind::ListEnding"),
        }
    }

    /// Consumes `self` and returns an `AtomReadUtil` if it is an atom,
    /// otherwise it returns a `ReadUtilError::ExpectedAtom`.
    ///
    /// # Example
    ///
    /// ```
    /// let src_data = b"example";
    /// let mut parser = sise::Parser::new(src_data);
    /// let mut node_read_util = sise::NodeReadUtil::new(&mut parser).unwrap();
    /// let atom_read_util = node_read_util.expect_atom().unwrap();
    /// assert_eq!(atom_read_util.into_atom(), "example");
    /// ```
    pub fn expect_atom(self) -> Result<AtomReadUtil<R>, ReadUtilError<R::Error, R::Pos>> {
        match self {
            NodeReadUtil::Atom(atom_read_util) => Ok(atom_read_util),
            NodeReadUtil::List(list_read_util) => Err(ReadUtilError::ExpectedAtom {
                pos: list_read_util.into_beginning_pos(),
            }),
        }
    }

    /// Consumes `self` and returns an `AtomReadUtil` if it is a list,
    /// otherwise it returns a `ReadUtilError::ExpectedListBeginning`.
    ///
    /// # Example
    ///
    /// ```
    /// let src_data = b"()";
    /// let mut parser = sise::Parser::new(src_data);
    /// let node_read_util = sise::NodeReadUtil::new(&mut parser).unwrap();
    /// let mut list_read_util = node_read_util.expect_list().unwrap();
    /// ```
    pub fn expect_list(self) -> Result<ListReadUtil<'a, R>, ReadUtilError<R::Error, R::Pos>> {
        match self {
            NodeReadUtil::Atom(atom_read_util) => Err(ReadUtilError::ExpectedListBeginning {
                pos: atom_read_util.into_pos(),
            }),
            NodeReadUtil::List(list_read_util) => Ok(list_read_util),
        }
    }
}

/// Utility to read atom nodes.
pub struct AtomReadUtil<R: Reader> {
    pos: R::Pos,
    atom: R::String,
}

impl<R: Reader> AtomReadUtil<R> {
    #[inline]
    pub fn new(pos: R::Pos, atom: R::String) -> Self {
        Self { pos, atom }
    }

    /// Returns a reference to the atom position stored in
    /// this utility.
    #[inline]
    pub fn pos(&self) -> &R::Pos {
        &self.pos
    }

    /// Returns a reference to the atom value stored in
    /// this utility.
    #[inline]
    pub fn atom(&self) -> &R::String {
        &self.atom
    }

    /// Consumes `self` and returns the stored atom value.
    #[inline]
    pub fn into_atom(self) -> R::String {
        self.atom
    }

    /// Consumes `self` and returns the stored position.
    #[inline]
    pub fn into_pos(self) -> R::Pos {
        self.pos
    }

    /// Consumes `self` and returns the stored atom position
    /// and value.
    #[inline]
    pub fn into_pos_atom(self) -> (R::Pos, R::String) {
        (self.pos, self.atom)
    }

    /// Decodes the atom using an user-provided function.
    ///
    /// The function shall return `None` if the decodification
    /// fails. In such case, `ReadError::InvalidValue` will be
    /// returned, with the value of `value_type`.
    ///
    /// # Example
    ///
    /// ```
    /// let src_data = b"example";
    /// let mut parser = sise::Parser::new(src_data);
    /// let node_read_util = sise::NodeReadUtil::new(&mut parser).unwrap();
    /// let atom_read_util = node_read_util.expect_atom().unwrap();
    /// let decoded = atom_read_util.decode(|atom| Some(atom.len()), "decode_as_length").unwrap();
    /// assert_eq!(decoded, 7);
    /// ```
    pub fn decode<T, F>(self, f: F, value_type: &str) -> Result<T, ReadUtilError<R::Error, R::Pos>>
    where
        F: FnOnce(&str) -> Option<T>,
    {
        if let Some(value) = f(self.atom.as_ref()) {
            Ok(value)
        } else {
            Err(ReadUtilError::InvalidValue {
                value_type: value_type.to_string(),
                pos: self.pos,
            })
        }
    }
}

/// Read util list
pub struct ListReadUtil<'a, R: Reader> {
    beginning_pos: R::Pos,
    reader: &'a mut R,
}

impl<'a, R: Reader> ListReadUtil<'a, R> {
    #[inline]
    pub fn new(beginning_pos: R::Pos, reader: &'a mut R) -> Self {
        Self {
            beginning_pos,
            reader,
        }
    }

    /// Returns a reference to the stored list beginning position.
    #[inline]
    pub fn beginning_pos(&self) -> &R::Pos {
        &self.beginning_pos
    }

    /// Consumes `self` and returns the stored list beginning position.
    #[inline]
    pub fn into_beginning_pos(self) -> R::Pos {
        self.beginning_pos
    }

    /// Checks if all the nodes in the list have been read.
    /// If not, it returns a `ReadUtilError::ExpectedListEnding` error.
    ///
    /// # Example
    ///
    /// ```
    /// let src_data = b"()";
    /// let mut parser = sise::Parser::new(src_data);
    /// let node_read_util = sise::NodeReadUtil::new(&mut parser).unwrap();
    /// let list_read_util = node_read_util.expect_list().unwrap();
    /// assert!(list_read_util.expect_ending().is_ok());
    /// ```
    pub fn expect_ending(self) -> Result<R::Pos, ReadUtilError<R::Error, R::Pos>> {
        let read_item = self.reader.read()?;
        match read_item.kind {
            ReadItemKind::Atom(_) => Err(ReadUtilError::ExpectedListEnding { pos: read_item.pos }),
            ReadItemKind::ListBeginning => {
                Err(ReadUtilError::ExpectedListEnding { pos: read_item.pos })
            }
            ReadItemKind::ListEnding => Ok(read_item.pos),
        }
    }

    /// Reads the next item in the list. If none are left, it
    /// returns `None`.
    ///
    /// # Example
    ///
    /// ```
    /// let src_data = b"(a b c)";
    /// let mut parser = sise::Parser::new(src_data);
    /// let node_read_util = sise::NodeReadUtil::new(&mut parser).unwrap();
    /// let mut list_read_util = node_read_util.expect_list().unwrap();
    /// assert_eq!(list_read_util.try_next_item().unwrap().unwrap().expect_atom().unwrap().into_atom(), "a");
    /// assert_eq!(list_read_util.try_next_item().unwrap().unwrap().expect_atom().unwrap().into_atom(), "b");
    /// assert_eq!(list_read_util.try_next_item().unwrap().unwrap().expect_atom().unwrap().into_atom(), "c");
    /// assert!(list_read_util.try_next_item().unwrap().is_none());
    /// ```
    pub fn try_next_item(&mut self) -> Result<Option<NodeReadUtil<'_, R>>, R::Error> {
        let read_item = self.reader.read()?;
        match read_item.kind {
            ReadItemKind::Atom(s) => Ok(Some(NodeReadUtil::Atom(AtomReadUtil::new(
                read_item.pos,
                s,
            )))),
            ReadItemKind::ListBeginning => Ok(Some(NodeReadUtil::List(ListReadUtil::new(
                read_item.pos,
                self.reader,
            )))),
            ReadItemKind::ListEnding => Ok(None),
        }
    }

    /// Reads the next item in the list. If none are left, it
    /// returns a `ReadUtilError::ExpectedNodeInList`.
    ///
    /// # Example
    ///
    /// ```
    /// let src_data = b"(a b c)";
    /// let mut parser = sise::Parser::new(src_data);
    /// let node_read_util = sise::NodeReadUtil::new(&mut parser).unwrap();
    /// let mut list_read_util = node_read_util.expect_list().unwrap();
    /// assert_eq!(list_read_util.next_item().unwrap().expect_atom().unwrap().into_atom(), "a");
    /// assert_eq!(list_read_util.next_item().unwrap().expect_atom().unwrap().into_atom(), "b");
    /// assert_eq!(list_read_util.next_item().unwrap().expect_atom().unwrap().into_atom(), "c");
    /// assert!(list_read_util.next_item().is_err());
    /// ```
    pub fn next_item(&mut self) -> Result<NodeReadUtil<'_, R>, ReadUtilError<R::Error, R::Pos>> {
        let read_item = self.reader.read()?;
        match read_item.kind {
            ReadItemKind::Atom(s) => Ok(NodeReadUtil::Atom(AtomReadUtil::new(read_item.pos, s))),
            ReadItemKind::ListBeginning => Ok(NodeReadUtil::List(ListReadUtil::new(
                read_item.pos,
                self.reader,
            ))),
            ReadItemKind::ListEnding => {
                Err(ReadUtilError::ExpectedNodeInList { pos: read_item.pos })
            }
        }
    }

    /// Gets the remaining items from the list, checks if they
    /// are atoms and decodes their value using `f`, returning
    /// a `Vec` with the result.
    ///
    /// # Example
    ///
    /// ```
    /// let src_data = b"(1 12 123)";
    /// let mut parser = sise::Parser::new(src_data);
    /// let node_read_util = sise::NodeReadUtil::new(&mut parser).unwrap();
    /// let mut list_read_util = node_read_util.expect_list().unwrap();
    /// let decoded = list_read_util.decode_atoms(|atom| Some(atom.len()), "decode_as_length", false).unwrap();
    /// assert_eq!(decoded, [1, 2, 3]);
    /// ```
    pub fn decode_atoms<T, F>(
        mut self,
        mut f: F,
        value_type: &str,
        can_be_empty: bool,
    ) -> Result<Vec<T>, ReadUtilError<R::Error, R::Pos>>
    where
        F: FnMut(&str) -> Option<T>,
    {
        let mut result = Vec::new();
        if !can_be_empty {
            result.push(
                self.next_item()?
                    .expect_atom()?
                    .decode(|atom| f(atom), value_type)?,
            );
        }
        while let Some(item) = self.try_next_item()? {
            result.push(item.expect_atom()?.decode(|atom| f(atom), value_type)?);
        }
        Ok(result)
    }
}

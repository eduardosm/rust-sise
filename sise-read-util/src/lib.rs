// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(test)]
mod tests;

extern crate sise;

#[derive(Clone, Debug, PartialEq)]
pub enum ReadError {
    ExpectedAtom {
        pos: Option<sise::Pos>,
    },
    ExpectedList {
        pos: Option<sise::Pos>,
    },
    ExpectedNodeInList {
        list_pos: Option<sise::Pos>,
    },
    UnexpectedNodeInList {
        node_pos: Option<sise::Pos>,
    },
    InvalidValue {
        value_type: String,
        pos: Option<sise::Pos>,
    },
}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReadError::ExpectedAtom { pos } => {
                if let Some(pos) = pos {
                    write!(f, "expected atom at {}:{}",
                           sise::ReprPosValue(pos.line),
                           sise::ReprPosValue(pos.column))
                } else {
                    f.write_str("expected atom")
                }
            }
            ReadError::ExpectedList { pos } => {
                if let Some(pos) = pos {
                    write!(f, "expected list at {}:{}",
                           sise::ReprPosValue(pos.line),
                           sise::ReprPosValue(pos.column))
                } else {
                    f.write_str("expected list")
                }
            }
            ReadError::ExpectedNodeInList { list_pos } => {
                if let Some(list_pos) = list_pos {
                    write!(f, "expected node in list at {}:{}",
                           sise::ReprPosValue(list_pos.line),
                           sise::ReprPosValue(list_pos.column))
                } else {
                    f.write_str("expected node in list")
                }
            }
            ReadError::UnexpectedNodeInList { node_pos } => {
                if let Some(node_pos) = node_pos {
                    write!(f, "unexpected node in list at {}:{}",
                           sise::ReprPosValue(node_pos.line),
                           sise::ReprPosValue(node_pos.column))
                } else {
                    f.write_str("unexpected node in list")
                }
            }
            ReadError::InvalidValue { value_type, pos } => {
                if let Some(pos) = pos {
                    write!(f, "invalid value of type {:?} at {}:{}",
                           value_type,
                           sise::ReprPosValue(pos.line),
                           sise::ReprPosValue(pos.column))
                } else {
                    write!(f, "invalid value of type {:?}", value_type)
                }
            }
        }
    }
}

impl std::error::Error for ReadError {
    fn description(&self) -> &str {
        match self {
            ReadError::ExpectedAtom { .. } => "expected atom",
            ReadError::ExpectedList { .. } => "expected list",
            ReadError::ExpectedNodeInList { .. } => "expected node in list",
            ReadError::UnexpectedNodeInList { .. } => "unexpected node in list",
            ReadError::InvalidValue { .. } => "invalid value",
        }
    }
}


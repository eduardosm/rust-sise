// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::sise_expr;
use crate::Node;
use crate::ReadUtilError;
use crate::NodeReadUtil;

#[test]
fn test_node_as_atom() {
    let node = sise_expr!("test");
    let read_util = NodeReadUtil::new(&node, None);
    assert_eq!(read_util.as_atom().unwrap().atom(), "test");

    let node = sise_expr!([]);
    let read_util = NodeReadUtil::new(&node, None);
    let expected_err = ReadUtilError::ExpectedAtom { pos: None };
    assert_eq!(read_util.as_atom().unwrap_err(), expected_err);
}

#[test]
fn test_node_as_list() {
    let node = sise_expr!([]);
    let read_util = NodeReadUtil::new(&node, None);
    assert_eq!(read_util.as_list().unwrap().list(), [] as [Node; 0]);

    let node = sise_expr!("test");
    let read_util = NodeReadUtil::new(&node, None);
    let expected_err = ReadUtilError::ExpectedList { pos: None };
    assert_eq!(read_util.as_list().unwrap_err(), expected_err);
}

fn decode_as_length(atom: &str) -> Option<usize> {
    if atom == "invalid" {
        None
    } else {
        Some(atom.len())
    }
}

#[test]
fn test_atom_decode() {
    let node = sise_expr!("aa");
    let node_read_util = NodeReadUtil::new(&node, None);
    let atom_read_util = node_read_util.as_atom().unwrap();
    assert_eq!(atom_read_util.decode(decode_as_length, "decode_as_length"), Ok(2));

    let node = sise_expr!("invalid");
    let node_read_util = NodeReadUtil::new(&node, None);
    let atom_read_util = node_read_util.as_atom().unwrap();
    let expected_err = ReadUtilError::InvalidValue {
        pos: None,
        value_type: "decode_as_length".to_string(),
    };
    assert_eq!(atom_read_util.decode(decode_as_length, "decode_as_length"), Err(expected_err));
}

#[test]
fn test_list_expect_end() {
    let node = sise_expr!([]);
    let node_read_util = NodeReadUtil::new(&node, None);
    let list_read_util = node_read_util.as_list().unwrap();
    assert!(list_read_util.expect_end().is_ok());

    let node = sise_expr!(["test"]);
    let node_read_util = NodeReadUtil::new(&node, None);
    let list_read_util = node_read_util.as_list().unwrap();
    let expected_err = ReadUtilError::ExpectedListEnd {
        node_pos: None,
    };
    assert_eq!(list_read_util.expect_end(), Err(expected_err));
}

#[test]
fn test_list_try_next_item() {
    let node = sise_expr!(["test-1", "test-2"]);
    let node_read_util = NodeReadUtil::new(&node, None);
    let mut list_read_util = node_read_util.as_list().unwrap();
    assert_eq!(list_read_util.try_next_item().unwrap().node(), "test-1");
    assert_eq!(list_read_util.try_next_item().unwrap().node(), "test-2");
    assert!(list_read_util.try_next_item().is_none());
}

#[test]
fn test_list_next_item() {
    let node = sise_expr!(["test-1", "test-2"]);
    let node_read_util = NodeReadUtil::new(&node, None);
    let mut list_read_util = node_read_util.as_list().unwrap();
    assert_eq!(list_read_util.next_item().unwrap().node(), "test-1");
    assert_eq!(list_read_util.next_item().unwrap().node(), "test-2");
    let expected_err = ReadUtilError::ExpectedNodeInList {
        list_pos: None,
    };
    assert_eq!(list_read_util.next_item().unwrap_err(), expected_err);
}

#[test]
fn test_list_decode_atoms() {
    let node = sise_expr!(["a", "aa"]);
    let node_read_util = NodeReadUtil::new(&node, None);
    let list_read_util = node_read_util.as_list().unwrap();
    let decoded = list_read_util.decode_atoms(decode_as_length, "decode_as_length", false).unwrap();
    assert_eq!(decoded, [1, 2]);

    let node = sise_expr!([]);
    let node_read_util = NodeReadUtil::new(&node, None);
    let list_read_util = node_read_util.as_list().unwrap();
    let decoded = list_read_util.decode_atoms(decode_as_length, "decode_as_length", true).unwrap();
    assert_eq!(decoded, []);

    let node = sise_expr!([]);
    let node_read_util = NodeReadUtil::new(&node, None);
    let list_read_util = node_read_util.as_list().unwrap();
    let expected_err = ReadUtilError::ExpectedNodeInList {
        list_pos: None,
    };
    let result = list_read_util.decode_atoms(decode_as_length, "decode_as_length", false);
    assert_eq!(result.unwrap_err(), expected_err);
}

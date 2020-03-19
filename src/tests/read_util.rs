// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::BytePos;
use crate::NodeReadUtil;
use crate::Parser;
use crate::ReadUtilError;

#[test]
fn test_node_as_atom() {
    let src_data = "test";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    assert_eq!(node_read_util.expect_atom().unwrap().into_atom(), "test");

    let src_data = "()";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let expected_err = ReadUtilError::ExpectedAtom { pos: BytePos(0) };
    assert_eq!(node_read_util.expect_atom().err().unwrap(), expected_err);
}

#[test]
fn test_node_as_list() {
    let src_data = "()";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    assert!(node_read_util.expect_list().is_ok());

    let src_data = "test";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let expected_err = ReadUtilError::ExpectedListBeginning { pos: BytePos(0) };
    assert_eq!(node_read_util.expect_list().err().unwrap(), expected_err);
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
    let src_data = "aa";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let atom_read_util = node_read_util.expect_atom().unwrap();
    assert_eq!(
        atom_read_util
            .decode(decode_as_length, "decode_as_length")
            .unwrap(),
        2
    );

    let src_data = "invalid";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let atom_read_util = node_read_util.expect_atom().unwrap();
    let expected_err = ReadUtilError::InvalidValue {
        pos: BytePos(0),
        value_type: "decode_as_length".to_string(),
    };
    assert_eq!(
        atom_read_util
            .decode(decode_as_length, "decode_as_length")
            .err()
            .unwrap(),
        expected_err
    );
}

#[test]
fn test_list_expect_end() {
    let src_data = "()";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let list_read_util = node_read_util.expect_list().unwrap();
    assert!(list_read_util.expect_ending().is_ok());

    let src_data = "(test)";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let list_read_util = node_read_util.expect_list().unwrap();
    let expected_err = ReadUtilError::ExpectedListEnding { pos: BytePos(1) };
    assert_eq!(list_read_util.expect_ending().err().unwrap(), expected_err);
}

#[test]
fn test_list_try_next_item() {
    let src_data = "(test-1 test-2)";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let mut list_read_util = node_read_util.expect_list().unwrap();
    assert_eq!(
        list_read_util
            .try_next_item()
            .unwrap()
            .unwrap()
            .expect_atom()
            .unwrap()
            .into_atom(),
        "test-1"
    );
    assert_eq!(
        list_read_util
            .try_next_item()
            .unwrap()
            .unwrap()
            .expect_atom()
            .unwrap()
            .into_atom(),
        "test-2"
    );
    assert!(list_read_util.try_next_item().unwrap().is_none());
}

#[test]
fn test_list_next_item() {
    let src_data = "(test-1 test-2)";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let mut list_read_util = node_read_util.expect_list().unwrap();
    assert_eq!(
        list_read_util
            .next_item()
            .unwrap()
            .expect_atom()
            .unwrap()
            .into_atom(),
        "test-1"
    );
    assert_eq!(
        list_read_util
            .next_item()
            .unwrap()
            .expect_atom()
            .unwrap()
            .into_atom(),
        "test-2"
    );
    let expected_err = ReadUtilError::ExpectedNodeInList { pos: BytePos(14) };
    assert_eq!(list_read_util.next_item().err().unwrap(), expected_err);
}

#[test]
fn test_list_decode_atoms() {
    let src_data = "(a aa)";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let list_read_util = node_read_util.expect_list().unwrap();
    let decoded = list_read_util
        .decode_atoms(decode_as_length, "decode_as_length", false)
        .unwrap();
    assert_eq!(decoded, [1, 2]);

    let src_data = "()";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let list_read_util = node_read_util.expect_list().unwrap();
    let decoded = list_read_util
        .decode_atoms(decode_as_length, "decode_as_length", true)
        .unwrap();
    assert_eq!(decoded, []);

    let src_data = "()";
    let mut parser = Parser::new(src_data);
    let node_read_util = NodeReadUtil::new(&mut parser).unwrap();
    let list_read_util = node_read_util.expect_list().unwrap();
    let expected_err = ReadUtilError::ExpectedNodeInList { pos: BytePos(1) };
    let result = list_read_util.decode_atoms(decode_as_length, "decode_as_length", false);
    assert_eq!(result.err().unwrap(), expected_err);
}

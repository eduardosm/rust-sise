// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use sise::sise_expr;

#[test]
fn test_get_as_atom() {
    let pos_map = sise::PosMap::new();

    let node1 = sise_expr!("test");
    assert_eq!(crate::get_as_atom(&node1, &pos_map), Ok("test"));

    let node2 = sise_expr!([]);
    let expected_err = crate::ReadError::ExpectedAtom { pos: None };
    assert_eq!(crate::get_as_atom(&node2, &pos_map), Err(expected_err));
}

#[test]
fn test_get_as_list() {
    let pos_map = sise::PosMap::new();

    let node1 = sise_expr!(["1", "2"]);
    let expected_list = [sise_expr!("1"), sise_expr!("2")];
    assert_eq!(crate::get_as_list(&node1, &pos_map), Ok(&expected_list as &[_]));

    let node2 = sise_expr!("test");
    let expected_err = crate::ReadError::ExpectedList { pos: None };
    assert_eq!(crate::get_as_list(&node2, &pos_map), Err(expected_err));
}

#[test]
fn test_get_node_from_list() {
    let pos_map = sise::PosMap::new();

    let list_node = sise_expr!(["1", "2"]);
    let mut iter = list_node.as_list().unwrap().iter();

    assert_eq!(crate::get_node_from_list(&mut iter, &list_node, &pos_map), Ok(&*sise_expr!("1")));
    assert_eq!(crate::get_node_from_list(&mut iter, &list_node, &pos_map), Ok(&*sise_expr!("2")));

    let expected_err = crate::ReadError::ExpectedNodeInList { list_pos: None };
    assert_eq!(crate::get_node_from_list(&mut iter, &list_node, &pos_map), Err(expected_err));
}

#[test]
fn test_expect_end_of_list() {
    let pos_map = sise::PosMap::new();

    let list_node = sise_expr!(["1"]);
    let mut iter = list_node.as_list().unwrap().iter();

    let expected_err = crate::ReadError::UnexpectedNodeInList { node_pos: None };
    assert_eq!(crate::expect_end_of_list(&mut iter, &pos_map), Err(expected_err));

    assert!(crate::expect_end_of_list(&mut iter, &pos_map).is_ok());
}

fn decode_as_length(atom: &str) -> Option<usize> {
    if atom == "invalid" {
        None
    } else {
        Some(atom.len())
    }
}

#[test]
fn test_decode_atom() {
    let pos_map = sise::PosMap::new();

    let node1 = sise_expr!("aa");
    assert_eq!(crate::decode_atom(&node1, decode_as_length, "decode_as_length", &pos_map), Ok(2));

    let node2 = sise_expr!("invalid");
    let expected_err = crate::ReadError::InvalidValue {
        pos: None,
        value_type: String::from("decode_as_length"),
    };
    assert_eq!(crate::decode_atom(&node2, decode_as_length, "decode_as_length", &pos_map), Err(expected_err));

    let node3 = sise_expr!([]);
    let expected_err = crate::ReadError::ExpectedAtom { pos: None };
    assert_eq!(crate::decode_atom(&node3, decode_as_length, "decode_as_length", &pos_map), Err(expected_err));
}

#[test]
fn test_get_and_decode_atom_from_list() {
    let pos_map = sise::PosMap::new();

    let list_node = sise_expr!(["aa", "aaa", "invalid"]);
    let mut iter = list_node.as_list().unwrap().iter();

    assert_eq!(crate::get_and_decode_atom_from_list(&mut iter,
                                                    &list_node,
                                                    decode_as_length,
                                                    "decode_as_length",
                                                    &pos_map), Ok(2));
    assert_eq!(crate::get_and_decode_atom_from_list(&mut iter,
                                                    &list_node,
                                                    decode_as_length,
                                                    "decode_as_length",
                                                    &pos_map), Ok(3));

    let expected_err = crate::ReadError::InvalidValue {
        pos: None,
        value_type: String::from("decode_as_length"),
    };
    assert_eq!(crate::get_and_decode_atom_from_list(&mut iter,
                                                    &list_node,
                                                    decode_as_length,
                                                    "decode_as_length",
                                                    &pos_map), Err(expected_err));

    let expected_err = crate::ReadError::ExpectedNodeInList { list_pos: None };
    assert_eq!(crate::get_and_decode_atom_from_list(&mut iter,
                                                    &list_node,
                                                    decode_as_length,
                                                    "decode_as_length",
                                                    &pos_map), Err(expected_err));
}

#[test]
fn test_get_and_decode_atoms_from_list() {
    let pos_map = sise::PosMap::new();

    let list_node1 = sise_expr!(["aa", "aaa"]);
    let mut iter = list_node1.as_list().unwrap().iter();

    assert_eq!(crate::get_and_decode_atoms_from_list(&mut iter,
                                                    &list_node1,
                                                     false,
                                                     decode_as_length,
                                                     "decode_as_length",
                                                     &pos_map), Ok(vec![2, 3]));

    let list_node2 = sise_expr!(["invalid"]);
    let mut iter = list_node2.as_list().unwrap().iter();

    let expected_err = crate::ReadError::InvalidValue {
        pos: None,
        value_type: String::from("decode_as_length"),
    };
    assert_eq!(crate::get_and_decode_atoms_from_list(&mut iter,
                                                     &list_node2,
                                                     false,
                                                     decode_as_length,
                                                     "decode_as_length",
                                                     &pos_map), Err(expected_err));

    let list_node3 = sise_expr!([]);
    let mut iter = list_node3.as_list().unwrap().iter();

    let expected_err = crate::ReadError::ExpectedNodeInList { list_pos: None };
    assert_eq!(crate::get_and_decode_atoms_from_list(&mut iter,
                                                     &list_node2,
                                                     false,
                                                     decode_as_length,
                                                     "decode_as_length",
                                                     &pos_map), Err(expected_err));

}

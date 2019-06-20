// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::sise_expr;
use crate::Node;
use crate::Reader;
use crate::Parser;
use crate::read_into_tree;

struct ReadTreeTest<'a> {
    src_data: &'a [u8],
    expected_tree: Node,
}

impl<'a> ReadTreeTest<'a> {
    fn run(&self) {
        let mut parser = Parser::new(self.src_data);
        let root_node = read_into_tree(&mut parser).unwrap();
        parser.finish().unwrap();
        assert_eq!(root_node, self.expected_tree);
    }
}

#[test]
fn test_empty_list() {
    ReadTreeTest {
        src_data: b"()",
        expected_tree: sise_expr!([]),
    }.run();
}

#[test]
fn test_single_atom() {
    ReadTreeTest {
        src_data: b"atom",
        expected_tree: sise_expr!("atom"),
    }.run();
}

#[test]
fn test_simple_list_1() {
    ReadTreeTest {
        src_data: b"(atom-1)",
        expected_tree: sise_expr!(["atom-1"]),
    }.run();
}

#[test]
fn test_simple_list_2() {
    ReadTreeTest {
        src_data: b"(atom-1 atom-2)",
        expected_tree: sise_expr!(["atom-1", "atom-2"]),
    }.run();
}

#[test]
fn test_nested_list_1() {
    ReadTreeTest {
        src_data: b"(())",
        expected_tree: sise_expr!([[]]),
    }.run();
}

#[test]
fn test_nested_list_2() {
    ReadTreeTest {
        src_data: b"(() ())",
        expected_tree: sise_expr!([[], []]),
    }.run();
}

#[test]
fn test_nested_list_3() {
    ReadTreeTest {
        src_data: b"((atom-1) (atom-2 atom-3))",
        expected_tree: sise_expr!([["atom-1"], ["atom-2", "atom-3"]]),
    }.run();
}

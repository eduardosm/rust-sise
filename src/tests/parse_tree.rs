// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::parse_tree;
use crate::sise_tree;
use crate::Parser;
use crate::TreeNode;

struct ParseTreeTest<'a> {
    src_data: &'a str,
    expected_tree: TreeNode,
}

impl<'a> ParseTreeTest<'a> {
    fn run(&self) {
        let mut parser = Parser::new(self.src_data);
        let root_node = parse_tree(&mut parser).unwrap();
        parser.finish().unwrap();
        assert_eq!(root_node, self.expected_tree);
    }
}

#[test]
fn test_empty_list() {
    ParseTreeTest {
        src_data: "()",
        expected_tree: sise_tree!([]),
    }
    .run();
}

#[test]
fn test_single_atom() {
    ParseTreeTest {
        src_data: "atom",
        expected_tree: sise_tree!("atom"),
    }
    .run();
}

#[test]
fn test_simple_list_1() {
    ParseTreeTest {
        src_data: "(atom-1)",
        expected_tree: sise_tree!(["atom-1"]),
    }
    .run();
}

#[test]
fn test_simple_list_2() {
    ParseTreeTest {
        src_data: "(atom-1 atom-2)",
        expected_tree: sise_tree!(["atom-1", "atom-2"]),
    }
    .run();
}

#[test]
fn test_nested_list_1() {
    ParseTreeTest {
        src_data: "(())",
        expected_tree: sise_tree!([[]]),
    }
    .run();
}

#[test]
fn test_nested_list_2() {
    ParseTreeTest {
        src_data: "(() ())",
        expected_tree: sise_tree!([[], []]),
    }
    .run();
}

#[test]
fn test_nested_list_3() {
    ParseTreeTest {
        src_data: "((atom-1) (atom-2 atom-3))",
        expected_tree: sise_tree!([["atom-1"], ["atom-2", "atom-3"]]),
    }
    .run();
}

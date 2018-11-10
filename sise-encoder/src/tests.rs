// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use sise::sise_expr;

fn compact_style() -> ::CompactStyle {
    ::CompactStyle::new()
}

fn spaced_style() -> ::SpacedStyle {
    spaced_style_with_keep_same_line(::std::collections::HashSet::new())
}

fn spaced_style_with_keep_same_line(keep_same_line: ::std::collections::HashSet<usize>) -> ::SpacedStyle {
    let spacing_config = ::SpacingConfig {
        line_ending: ::LineEnding::Lf,
        indent_len: 1,
        indent_char: ::IndentChar::Tab,
    };
    ::SpacedStyle::new(spacing_config, keep_same_line)
}

#[test]
fn test_empty_list() {
    let root_node = sise_expr!([]);
    let expected_compact = "()";
    let expected_spaced = "()\n";
    let result_compact = ::serialize(&root_node, &mut compact_style());
    let result_spaced = ::serialize(&root_node, &mut spaced_style());
    assert_eq!(result_compact, expected_compact);
    assert_eq!(result_spaced, expected_spaced);
}

#[test]
fn test_single_atom() {
    let root_node = sise_expr!("atom");
    let expected_compact = "atom";
    let expected_spaced = "atom\n";
    let result_compact = ::serialize(&root_node, &mut compact_style());
    let result_spaced = ::serialize(&root_node, &mut spaced_style());
    assert_eq!(result_compact, expected_compact);
    assert_eq!(result_spaced, expected_spaced);
}

#[test]
fn test_list_with_one_atom() {
    let root_node = sise_expr!(["1"]);
    let expected_compact = "(1)";
    let expected_spaced = "(1)\n";
    let result_compact = ::serialize(&root_node, &mut compact_style());
    let result_spaced = ::serialize(&root_node, &mut spaced_style());
    assert_eq!(result_compact, expected_compact);
    assert_eq!(result_spaced, expected_spaced);
}

#[test]
fn test_list_with_two_atoms() {
    let root_node = sise_expr!(["1", "2"]);
    let expected_compact = "(1 2)";
    let expected_spaced = "(1\n\t2\n)\n";
    let result_compact = ::serialize(&root_node, &mut compact_style());
    let result_spaced = ::serialize(&root_node, &mut spaced_style());
    assert_eq!(result_compact, expected_compact);
    assert_eq!(result_spaced, expected_spaced);
}

#[test]
fn test_list_with_three_atoms() {
    let root_node = sise_expr!(["1", "2", "3"]);
    let expected_compact = "(1 2 3)";
    let expected_spaced = "(1\n\t2\n\t3\n)\n";
    let result_compact = ::serialize(&root_node, &mut compact_style());
    let result_spaced = ::serialize(&root_node, &mut spaced_style());
    assert_eq!(result_compact, expected_compact);
    assert_eq!(result_spaced, expected_spaced);
}

#[test]
fn test_nested_list() {
    let root_node = sise_expr!([[]]);
    let expected_compact = "(())";
    let expected_spaced = "(\n\t()\n)\n";
    let result_compact = ::serialize(&root_node, &mut compact_style());
    let result_spaced = ::serialize(&root_node, &mut spaced_style());
    assert_eq!(result_compact, expected_compact);
    assert_eq!(result_spaced, expected_spaced);
}

#[test]
fn test_list_with_atom_and_list() {
    let root_node = sise_expr!(["atom", []]);
    let expected_compact = "(atom ())";
    let expected_spaced = "(atom\n\t()\n)\n";
    let result_compact = ::serialize(&root_node, &mut compact_style());
    let result_spaced = ::serialize(&root_node, &mut spaced_style());
    assert_eq!(result_compact, expected_compact);
    assert_eq!(result_spaced, expected_spaced);
}

#[test]
fn test_spaced_with_keep_line_1() {
    let root_node = sise_expr!(["atom", ["1", "2", "3"], ["a", "b", "c"]]);

    let mut keep_same_line = ::std::collections::HashSet::new();
    keep_same_line.insert(root_node.index_path(&[1, 1]).unwrap().ref_as_usize());
    keep_same_line.insert(root_node.index_path(&[1, 2]).unwrap().ref_as_usize());
    keep_same_line.insert(root_node.index_path(&[2, 1]).unwrap().ref_as_usize());
    keep_same_line.insert(root_node.index_path(&[2, 2]).unwrap().ref_as_usize());

    let result = ::serialize(&root_node, &mut spaced_style_with_keep_same_line(keep_same_line));
    let expected = "(atom\n\t(1 2 3)\n\t(a b c)\n)\n";
    assert_eq!(result, expected);
}

#[test]
fn test_spaced_with_keep_line_2() {
    let root_node = sise_expr!(["atom", ["1", "2", "3"], ["a", "b", "c"]]);

    let mut keep_same_line = ::std::collections::HashSet::new();
    keep_same_line.insert(root_node.index_path(&[1, 1]).unwrap().ref_as_usize());
    keep_same_line.insert(root_node.index_path(&[2, 1]).unwrap().ref_as_usize());

    let result = ::serialize(&root_node, &mut spaced_style_with_keep_same_line(keep_same_line));
    let expected = "(atom\n\t(1 2\n\t\t3\n\t)\n\t(a b\n\t\tc\n\t)\n)\n";
    assert_eq!(result, expected);
}

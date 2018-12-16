// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::sise_expr;
use crate::Pos;
use crate::ParseLimits;
use crate::ParseError;
use crate::parse;
use crate::Token;

macro_rules! pos_tree {
    ($line:expr, $column:expr) => {
        crate::PosTree {
            pos: Pos::new($line, $column),
            children: vec![],
        }
    };
    ($line:expr, $column:expr, [$(($($children:tt)*)),*]) => {
        crate::PosTree {
            pos: Pos::new($line, $column),
            children: vec![$(pos_tree!($($children)*)),*],
        }
    };
}

#[test]
fn test_empty_list() {
    let src_data = b"()";
    let expected_tree = sise_expr!([]);
    let expected_positions = pos_tree!(0, 0);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_single_atom() {
    let src_data = b"atom";
    let expected_tree = sise_expr!("atom");
    let expected_positions = pos_tree!(0, 0);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_simple_list_1() {
    let src_data = b"(atom-1)";
    let expected_tree = sise_expr!(["atom-1"]);
    let expected_positions = pos_tree!(0, 0, [(0, 1)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_simple_list_2() {
    let src_data = b"(atom-1 atom-2)";
    let expected_tree = sise_expr!(["atom-1", "atom-2"]);
    let expected_positions = pos_tree!(0, 0, [(0, 1), (0, 8)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_nested_list_1() {
    let src_data = b"(())";
    let expected_tree = sise_expr!([[]]);
    let expected_positions = pos_tree!(0, 0, [(0, 1)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_nested_list_2() {
    let src_data = b"(() ())";
    let expected_tree = sise_expr!([[], []]);
    let expected_positions = pos_tree!(0, 0, [(0, 1), (0, 4)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_nested_list_3() {
    let src_data = b"((atom-1) (atom-2 atom-3))";
    let expected_tree = sise_expr!([["atom-1"], ["atom-2", "atom-3"]]);
    let expected_positions = pos_tree!(0, 0, [
        (0, 1, [(0, 2)]),
        (0, 10, [(0, 11), (0, 18)])
    ]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_nested_lists() {
    let src_data = b"(((((((((())))))))))";
    let expected_tree = sise_expr!([[[[[[[[[[]]]]]]]]]]);
    let expected_positions = pos_tree!(0, 0, [
        (0, 1, [
            (0, 2, [
                (0, 3, [
                    (0, 4, [
                        (0, 5, [
                            (0, 6, [
                                (0, 7, [
                                    (0, 8, [(0, 9)])
                                ])
                            ])
                        ])
                    ])
                ])
            ])
        ])
    ]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}


#[test]
fn test_mixed() {
    let src_data = b"(atom-1 (atom-2) (atom-3 (atom-4) atom-5) atom-6)";
    let expected_tree = sise_expr!(["atom-1", ["atom-2"], ["atom-3", ["atom-4"], "atom-5"], "atom-6"]);
    let expected_positions = pos_tree!(0, 0, [
        (0, 1),
        (0, 8, [(0, 9)]),
        (0, 17, [
            (0, 18),
            (0, 25, [(0, 26)]),
            (0, 34)
        ]),
        (0, 42)
    ]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_atom_chars() {
    let src_data = b"!#$%&*+-./:<=>?@_~";
    let expected_tree = sise_expr!("!#$%&*+-./:<=>?@_~");
    let expected_positions = pos_tree!(0, 0);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_string_1() {
    let src_data = b"\"atom-1\"";
    let expected_tree = sise_expr!("\"atom-1\"");
    let expected_positions = pos_tree!(0, 0);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_string_2() {
    let src_data = b"prefix\"atom-1\"suffix";
    let expected_tree = sise_expr!("prefix\"atom-1\"suffix");
    let expected_positions = pos_tree!(0, 0);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_string_3() {
    let src_data = b"\" \\\\ \\\" \"";
    let expected_tree = sise_expr!("\" \\\\ \\\" \"");
    let expected_positions = pos_tree!(0, 0);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_multiline_lf() {
    let src_data = b"\n(1 2\n3 4)\n";
    let expected_tree = sise_expr!(["1", "2", "3", "4"]);
    let expected_positions = pos_tree!(1, 0, [(1, 1), (1, 3), (2, 0), (2, 2)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_multiline_crlf() {
    let src_data = b"\r\n(1 2\r\n3 4)\r\n";
    let expected_tree = sise_expr!(["1", "2", "3", "4"]);
    let expected_positions = pos_tree!(1, 0, [(1, 1), (1, 3), (2, 0), (2, 2)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_multiline_cr() {
    let src_data = b"\r(1 2\r3 4)\r";
    let expected_tree = sise_expr!(["1", "2", "3", "4"]);
    let expected_positions = pos_tree!(1, 0, [(1, 1), (1, 3), (2, 0), (2, 2)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_multiline_mixed() {
    let src_data = b"\n\r\r\n(1 2\n\r\r\n3 4)\n\r\r\n";
    let expected_tree = sise_expr!(["1", "2", "3", "4"]);
    let expected_positions = pos_tree!(3, 0, [(3, 1), (3, 3), (6, 0), (6, 2)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_comment_1() {
    let src_data = b"; comment\n(1 2)";
    let expected_tree = sise_expr!(["1", "2"]);
    let expected_positions = pos_tree!(1, 0, [(1, 1), (1, 3)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_comment_2() {
    let src_data = b"(1 2); comment";
    let expected_tree = sise_expr!(["1", "2"]);
    let expected_positions = pos_tree!(0, 0, [(0, 1), (0, 3)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_comment_3() {
    let src_data = b"(1 2); comment";
    let expected_tree = sise_expr!(["1", "2"]);
    let expected_positions = pos_tree!(0, 0, [(0, 1), (0, 3)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_comment_4() {
    let src_data = b"(1; comment\n2)";
    let expected_tree = sise_expr!(["1", "2"]);
    let expected_positions = pos_tree!(0, 0, [(0, 1), (1, 0)]);
    let (parsed_tree, parsed_positions) = parse(src_data, &ParseLimits::unlimited()).unwrap();
    assert_eq!(parsed_tree, expected_tree);
    assert_eq!(parsed_positions, expected_positions);
}

#[test]
fn test_fail_empty() {
    let src_data = b"";
    let expected_error = ParseError::UnexpectedToken {
        pos: Pos::new(0, 0),
        token: Token::Eof,
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited()).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_expected_eof() {
    let src_data = b"() ()";
    let expected_error = ParseError::ExpectedEof {
        pos: Pos::new(0, 3),
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited()).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_unfinished_string() {
    let src_data = b"\"atom-1";
    let expected_error = ParseError::UnfinishedString {
        pos: Pos::new(0, 7),
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited()).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_unclosed_list() {
    let src_data = b"(atom-1";
    let expected_error = ParseError::UnexpectedToken {
        pos: Pos::new(0, 7),
        token: Token::Eof,
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited()).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_unclosed_list_with_comment() {
    let src_data = b"(atom-1 ; comment)";
    let expected_error = ParseError::UnexpectedToken {
        pos: Pos::new(0, 18),
        token: Token::Eof,
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited()).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_illegal_chr() {
    let src_data = b"\xFF";
    let expected_error = ParseError::IllegalChr {
        pos: Pos::new(0, 0),
        chr: 0xFF,
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited()).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_illegal_chr_in_string() {
    let src_data = b"\"\xFF";
    let expected_error = ParseError::IllegalChrInString {
        pos: Pos::new(0, 1),
        chr: 0xFF,
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited()).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_illegal_chr_in_comment() {
    let src_data = b"() ; \xFF";
    let expected_error = ParseError::IllegalChrInComment {
        pos: Pos::new(0, 5),
        chr: 0xFF,
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited()).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_too_deep() {
    let src_data = b"(())";
    let expected_error = ParseError::TooDeep {
        pos: Pos::new(0, 1),
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited().max_depth(1)).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_atom_too_long() {
    let src_data = b"1234";
    let expected_error = ParseError::AtomTooLong {
        pos: Pos::new(0, 0),
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited().max_atom_len(3)).unwrap_err();
    assert_eq!(parse_error, expected_error);
}

#[test]
fn test_fail_list_too_long() {
    let src_data = b"(1 2 3 4)";
    let expected_error = ParseError::ListTooLong {
        pos: Pos::new(0, 7),
    };
    let parse_error = parse(src_data, &ParseLimits::unlimited().max_list_len(3)).unwrap_err();
    assert_eq!(parse_error, expected_error);
}
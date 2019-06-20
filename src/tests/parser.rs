// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::ParseError;
use crate::Parser;
use crate::Pos;
use crate::Reader as _;
use crate::ReadItem;
use crate::ReadItemKind;
use crate::TokenKind;

struct ParserPassTest<'a> {
    src_data: &'a [u8],
    expected_items: &'a [ReadItem<&'a str, Pos>],
}

impl<'a> ParserPassTest<'a> {
    fn run(&self) {
        let mut parser = Parser::new(self.src_data);
        for read_item in self.expected_items.iter() {
            assert_eq!(parser.read().unwrap(), *read_item);
        }
        parser.finish().unwrap();
    }
}

struct ParserFailTest<'a> {
    src_data: &'a [u8],
    expected_items: &'a [ReadItem<&'a str, Pos>],
    error_at_finish: bool,
    expected_error: ParseError,
}

impl<'a> ParserFailTest<'a> {
    fn run(&self) {
        let mut parser = Parser::new(self.src_data);
        for read_item in self.expected_items.iter() {
            assert_eq!(parser.read().unwrap(), *read_item);
        }
        if self.error_at_finish {
            assert_eq!(parser.finish().unwrap_err(), self.expected_error);
        } else {
            assert_eq!(parser.read().unwrap_err(), self.expected_error);
        }
    }
}


#[test]
fn test_empty_list() {
    ParserPassTest {
        src_data: b"()",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_single_atom() {
    ParserPassTest {
        src_data: b"atom",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::Atom("atom"),
            },
        ],
    }.run();
}

#[test]
fn test_simple_list_1() {
    ParserPassTest {
        src_data: b"(atom-1)",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: Pos::new(0, 7),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_simple_list_2() {
    ParserPassTest {
        src_data: b"(atom-1 atom-2)",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: Pos::new(0, 8),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: Pos::new(0, 14),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_nested_list_1() {
    ParserPassTest {
        src_data: b"(())",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 2),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 3),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_nested_list_2() {
    ParserPassTest {
        src_data: b"(() ())",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 2),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 4),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 5),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 6),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_nested_list_3() {
    ParserPassTest {
        src_data: b"((atom-1) (atom-2 atom-3))",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 2),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: Pos::new(0, 8),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 10),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 11),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: Pos::new(0, 18),
                kind: ReadItemKind::Atom("atom-3"),
            },
            ReadItem {
                pos: Pos::new(0, 24),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 25),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_nested_lists() {
    ParserPassTest {
        src_data: b"(((((((((())))))))))",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 2),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 3),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 4),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 5),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 6),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 7),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 8),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 9),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 10),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 11),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 12),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 13),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 14),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 15),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 16),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 17),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 18),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 19),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_mixed() {
    ParserPassTest {
        src_data: b"(atom-1 (atom-2) (atom-3 (atom-4) atom-5) atom-6)",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::Atom("atom-1"),
            },
            ReadItem {
                pos: Pos::new(0, 8),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 9),
                kind: ReadItemKind::Atom("atom-2"),
            },
            ReadItem {
                pos: Pos::new(0, 15),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 17),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 18),
                kind: ReadItemKind::Atom("atom-3"),
            },
            ReadItem {
                pos: Pos::new(0, 25),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 26),
                kind: ReadItemKind::Atom("atom-4"),
            },
            ReadItem {
                pos: Pos::new(0, 32),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 34),
                kind: ReadItemKind::Atom("atom-5"),
            },
            ReadItem {
                pos: Pos::new(0, 40),
                kind: ReadItemKind::ListEnding,
            },
            ReadItem {
                pos: Pos::new(0, 42),
                kind: ReadItemKind::Atom("atom-6"),
            },
            ReadItem {
                pos: Pos::new(0, 48),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_atom_chars() {
    ParserPassTest {
        src_data: b"!#$%&*+-./:<=>?@_~",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::Atom("!#$%&*+-./:<=>?@_~"),
            },
        ],
    }.run();
}

#[test]
fn test_string_1() {
    ParserPassTest {
        src_data: b"\"atom-1\"",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::Atom("\"atom-1\""),
            },
        ],
    }.run();
}

#[test]
fn test_string_2() {
    ParserPassTest {
        src_data: b"prefix\"atom-1\"suffix",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::Atom("prefix\"atom-1\"suffix"),
            },
        ],
    }.run();
}

#[test]
fn test_string_3() {
    ParserPassTest {
        src_data: b"\" \\\\ \\\" \"",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::Atom("\" \\\\ \\\" \""),
            },
        ],
    }.run();
}

#[test]
fn test_multiline_lf() {
    ParserPassTest {
        src_data: b"\n(1 2\n3 4)\n",
        expected_items: &[
            ReadItem {
                pos: Pos::new(1, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(1, 1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: Pos::new(1, 3),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: Pos::new(2, 0),
                kind: ReadItemKind::Atom("3"),
            },
            ReadItem {
                pos: Pos::new(2, 2),
                kind: ReadItemKind::Atom("4"),
            },
            ReadItem {
                pos: Pos::new(2, 3),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_multiline_crlf() {
    ParserPassTest {
        src_data: b"\r\n(1 2\r\n3 4)\r\n",
        expected_items: &[
            ReadItem {
                pos: Pos::new(1, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(1, 1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: Pos::new(1, 3),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: Pos::new(2, 0),
                kind: ReadItemKind::Atom("3"),
            },
            ReadItem {
                pos: Pos::new(2, 2),
                kind: ReadItemKind::Atom("4"),
            },
            ReadItem {
                pos: Pos::new(2, 3),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_multiline_cr() {
    ParserPassTest {
        src_data: b"\r(1 2\r3 4)\r",
        expected_items: &[
            ReadItem {
                pos: Pos::new(1, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(1, 1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: Pos::new(1, 3),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: Pos::new(2, 0),
                kind: ReadItemKind::Atom("3"),
            },
            ReadItem {
                pos: Pos::new(2, 2),
                kind: ReadItemKind::Atom("4"),
            },
            ReadItem {
                pos: Pos::new(2, 3),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_multiline_mixed() {
    ParserPassTest {
        src_data: b"\n\r\r\n(1 2\n\r\r\n3 4)\n\r\r\n",
        expected_items: &[
            ReadItem {
                pos: Pos::new(3, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(3, 1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: Pos::new(3, 3),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: Pos::new(6, 0),
                kind: ReadItemKind::Atom("3"),
            },
            ReadItem {
                pos: Pos::new(6, 2),
                kind: ReadItemKind::Atom("4"),
            },
            ReadItem {
                pos: Pos::new(6, 3),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_comment_1() {
    ParserPassTest {
        src_data: b"; comment\n(1 2)",
        expected_items: &[
            ReadItem {
                pos: Pos::new(1, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(1, 1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: Pos::new(1, 3),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: Pos::new(1, 4),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_comment_2() {
    ParserPassTest {
        src_data: b"(1 2); comment",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: Pos::new(0, 3),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: Pos::new(0, 4),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_comment_3() {
    ParserPassTest {
        src_data: b"(1; comment\n2)",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::Atom("1"),
            },
            ReadItem {
                pos: Pos::new(1, 0),
                kind: ReadItemKind::Atom("2"),
            },
            ReadItem {
                pos: Pos::new(1, 1),
                kind: ReadItemKind::ListEnding,
            },
        ],
    }.run();
}

#[test]
fn test_fail_empty() {
    ParserFailTest {
        src_data: b"",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedToken {
            pos: Pos::new(0, 0),
            token: TokenKind::Eof,
        },
    }.run();
}

#[test]
fn test_fail_expected_eof() {
    ParserFailTest {
        src_data: b"() ()",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::ListEnding,
            },
        ],
        error_at_finish: true,
        expected_error: ParseError::ExpectedEof {
            pos: Pos::new(0, 3),
        },
    }.run();
}

#[test]
fn test_fail_unfinished_string() {
    ParserFailTest {
        src_data: b"\"atom-1",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::UnfinishedString {
            pos: Pos::new(0, 7),
        },
    }.run();
}

#[test]
fn test_fail_unclosed_list() {
    ParserFailTest {
        src_data: b"(atom-1",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::Atom("atom-1"),
            },
        ],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedToken {
            pos: Pos::new(0, 7),
            token: TokenKind::Eof,
        },
    }.run();
}

#[test]
fn test_fail_unexpected_closing() {
    ParserFailTest {
        src_data: b")",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedToken {
            pos: Pos::new(0, 0),
            token: TokenKind::RightParen,
        },
    }.run();
}

#[test]
fn test_fail_unclosed_list_with_comment() {
    ParserFailTest {
        src_data: b"(atom-1 ; comment)",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::Atom("atom-1"),
            },
        ],
        error_at_finish: false,
        expected_error: ParseError::UnexpectedToken {
            pos: Pos::new(0, 18),
            token: TokenKind::Eof,
        },
    }.run();
}

#[test]
fn test_fail_illegal_chr() {
    ParserFailTest {
        src_data: b"\xFF",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::IllegalChr {
            pos: Pos::new(0, 0),
            chr: 0xFF,
        },
    }.run();
}

#[test]
fn test_fail_illegal_chr_in_string() {
    ParserFailTest {
        src_data: b"\"\xFF",
        expected_items: &[],
        error_at_finish: false,
        expected_error: ParseError::IllegalChrInString {
            pos: Pos::new(0, 1),
            chr: 0xFF,
        },
    }.run();
}

#[test]
fn test_fail_illegal_chr_in_comment() {
    ParserFailTest {
        src_data: b"() ; \xFF",
        expected_items: &[
            ReadItem {
                pos: Pos::new(0, 0),
                kind: ReadItemKind::ListBeginning,
            },
            ReadItem {
                pos: Pos::new(0, 1),
                kind: ReadItemKind::ListEnding,
            },
        ],
        error_at_finish: true,
        expected_error: ParseError::IllegalChrInComment {
            pos: Pos::new(0, 5),
            chr: 0xFF,
        },
    }.run();
}

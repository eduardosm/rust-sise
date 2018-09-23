// Copyright 2018 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[test]
pub fn test_check_atom() {
    assert!(::check_atom("1234"));
    assert!(::check_atom("AbCd"));
    assert!(::check_atom("AbCd-1234"));
    assert!(::check_atom("\"\""));
    assert!(::check_atom("\" \\_ \\\" \\\\ \""));
    assert!(::check_atom("prefix\"abcd\"suffix"));
    assert!(::check_atom("!#$%&*+-./:<=>?@_~"));

    assert!(!::check_atom(""));
    assert!(!::check_atom(" "));
    assert!(!::check_atom("'"));
    assert!(!::check_atom("\t"));
    assert!(!::check_atom("\n"));
    assert!(!::check_atom("\r"));
    assert!(!::check_atom("\x00"));
    assert!(!::check_atom("\x7F"));
    assert!(!::check_atom("\u{FFFD}"));
    assert!(!::check_atom("("));
    assert!(!::check_atom(")"));
    assert!(!::check_atom("["));
    assert!(!::check_atom("]"));
    assert!(!::check_atom(";"));
    assert!(!::check_atom("\\"));
    assert!(!::check_atom("\""));
    assert!(!::check_atom("abcd\""));
    assert!(!::check_atom("\"abcd"));
    assert!(!::check_atom("\"\\\""));
}

// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[allow(clippy::cognitive_complexity)]
#[test]
fn test_check_atom() {
    assert!(crate::check_atom("1234"));
    assert!(crate::check_atom("AbCd"));
    assert!(crate::check_atom("AbCd-1234"));
    assert!(crate::check_atom("\"\""));
    assert!(crate::check_atom("\" \\_ \\\" \\\\ \""));
    assert!(crate::check_atom("prefix\"abcd\"suffix"));
    assert!(crate::check_atom("!#$%&*+-./:<=>?@_~"));

    assert!(!crate::check_atom(""));
    assert!(!crate::check_atom(" "));
    assert!(!crate::check_atom("'"));
    assert!(!crate::check_atom("\t"));
    assert!(!crate::check_atom("\n"));
    assert!(!crate::check_atom("\r"));
    assert!(!crate::check_atom("\x00"));
    assert!(!crate::check_atom("\x7F"));
    assert!(!crate::check_atom("\u{FFFD}"));
    assert!(!crate::check_atom("("));
    assert!(!crate::check_atom(")"));
    assert!(!crate::check_atom("["));
    assert!(!crate::check_atom("]"));
    assert!(!crate::check_atom(";"));
    assert!(!crate::check_atom("\\"));
    assert!(!crate::check_atom("\""));
    assert!(!crate::check_atom("abcd\""));
    assert!(!crate::check_atom("\"abcd"));
    assert!(!crate::check_atom("\"\\\""));
}

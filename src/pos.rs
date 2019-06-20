// Copyright 2019 Eduardo Sánchez Muñoz
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

/// Represents a position in a text file.
///
/// Lines and columns begin to count with zero.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub line: u32,
    pub column: u32,
}

impl Pos {
    #[inline]
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }
}

/// Wrapper whose `Display` implementation prints
/// `self.0 + 1`, taking care of overflow.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReprPosValue(pub u32);

impl std::fmt::Display for ReprPosValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.checked_add(1) {
            Some(value) => std::fmt::Display::fmt(&value, f),
            None => f.write_str("4294967296"),
        }
    }
}

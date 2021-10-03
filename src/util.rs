/// Returns whether `chr` is a valid atom character outside a
/// string (i.e. one of `:atomchar:` documented at `TreeNode::Atom`).
#[inline]
pub fn is_atom_chr(chr: char) -> bool {
    matches!(
        chr,
        '!' | '#'
            | '$'
            | '%'
            | '&'
            | '*'
            | '+'
            | '-'
            | '.'
            | '/'
            | '0'..='9'
            | ':'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | 'A'..='Z'
            | '_'
            | 'a'..='z'
            | '~'
    )
}

/// Returns whether `chr` is a valid atom character inside a
/// string, excluding `"` and `\` (i.e. one of `:stringchar:`
/// documented at `TreeNode::Atom`).
#[inline]
pub fn is_atom_string_chr(chr: char) -> bool {
    matches!(chr, ' '..='~' if chr != '"' && chr != '\\')
}

/// Checks whether `atom` is a valid atom (i.e. matches the regular
/// expression documented at `TreeNode::Atom`).
pub fn check_atom(atom: &str) -> bool {
    enum State {
        Beginning,
        Normal,
        String,
        StringBackslash,
    }

    let mut state = State::Beginning;
    let mut iter = atom.chars();
    loop {
        let chr = iter.next();
        match state {
            State::Beginning => {
                match chr {
                    Some('"') => {
                        state = State::String;
                    }
                    Some(chr) if is_atom_chr(chr) => {
                        state = State::Normal;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Empty atom
                        return false;
                    }
                }
            }
            State::Normal => {
                match chr {
                    Some('"') => {
                        state = State::String;
                    }
                    Some(chr) if is_atom_chr(chr) => {
                        state = State::Normal;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Valid atom
                        return true;
                    }
                }
            }
            State::String => {
                match chr {
                    Some('"') => {
                        state = State::Normal;
                    }
                    Some('\\') => {
                        state = State::StringBackslash;
                    }
                    Some(chr) if is_atom_string_chr(chr) => {
                        state = State::String;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Incomplete string
                        return false;
                    }
                }
            }
            State::StringBackslash => {
                match chr {
                    Some(chr) if is_atom_string_chr(chr) || chr == '"' || chr == '\\' => {
                        state = State::String;
                    }
                    Some(_) => {
                        // Illegal character
                        return false;
                    }
                    None => {
                        // Incomplete string
                        return false;
                    }
                }
            }
        }
    }
}

use std::str::Chars;

use crate::handlers;
use crate::tokens;

pub fn scan_char(c: char, chars: &mut Chars, output: &mut String) {
    match c {
        '#' => handlers::heading::handler(chars, output),
        '_' => handlers::basic::handler(&tokens::ITALIC_TOKEN, chars, output),
        '*' => handlers::basic::handler(&tokens::BOLD_TOKEN, chars, output),
        _ => output.push(c),
    }
}

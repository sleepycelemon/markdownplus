use std::str::Chars;

use crate::handlers;
use crate::tokens;

pub fn scan_char(c: char, chars: &mut Chars, output: &mut String) {
    match c {
        '#' => handlers::basic_text_handler(&tokens::HEADING_ONE_TOKEN, chars, output),
        '_' => handlers::basic_text_handler(&tokens::ITALIC_TOKEN, chars, output),
        _ => output.push(c),
    }
}


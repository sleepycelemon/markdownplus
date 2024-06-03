use crate::scanner;
use crate::tokens;

use std::str::Chars;

pub fn basic_text_handler(token: &tokens::Token, chars: &mut Chars, output: &mut String) {
    output.push_str(token.open_tag);

    while let Some(c) = chars.next() {
        match c {
            _ if c == token.termination_char => {
                output.push_str(token.close_tag);
                return;
            }
            _ => scanner::scan_char(c, chars, output),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokens;

    #[test]
    fn handles_heading() {
        let want = "<h1>hello!</h1>\n";

        let mut got = String::new();

        super::basic_text_handler(
            &tokens::HEADING_ONE_TOKEN,
            &mut "hello!\n".chars(),
            &mut got,
        );

        assert_eq!(got, want);
    }
}

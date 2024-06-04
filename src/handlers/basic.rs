use std::str::Chars;

use crate::{scanner, tokens};

pub fn handler(token: &tokens::Token, chars: &mut Chars, output: &mut String) {
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
    fn handles_italics() {
        let want = "<em>hello!</em>";

        let mut got = String::new();

        super::handler(&tokens::ITALIC_TOKEN, &mut "hello!_".chars(), &mut got);

        assert_eq!(want, got);
    }

    #[test]
    fn handles_bold() {
        let want = "<b>hello!</b>";

        let mut got = String::new();

        super::handler(&tokens::BOLD_TOKEN, &mut "hello!*".chars(), &mut got);

        assert_eq!(want, got);
    }
}

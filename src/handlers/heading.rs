use std::str::Chars;

use crate::{scanner, tokens};

fn get_variant(chars: &mut Chars) -> &'static tokens::Token {
    let mut variant = 1;

    // Count the number of consecutive '#' characters
    while let Some(c) = chars.as_str().chars().next() {
        if c == '#' {
            variant += 1;
            chars.next(); // Advance the iterator
        } else {
            break;
        }
    }

    match variant {
        1 => &tokens::HEADING_ONE_TOKEN,
        2 => &tokens::HEADING_TWO_TOKEN,
        3 => &tokens::HEADING_THREE_TOKEN,
        4 => &tokens::HEADING_FOUR_TOKEN,
        5 => &tokens::HEADING_FIVE_TOKEN,
        6 => &tokens::HEADING_SIX_TOKEN,
        _ => &tokens::HEADING_SIX_TOKEN,
    }
}

pub fn handler(chars: &mut Chars, output: &mut String) {
    let variant = get_variant(chars);

    output.push_str(variant.open_tag);

    while let Some(c) = chars.next() {
        match c {
            _ if c == variant.termination_char => {
                output.push_str(variant.close_tag);
                return;
            }
            _ => scanner::scan_char(c, chars, output),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{get_variant, handler};
    #[test]
    fn handles_heading_one() {
        let want = "<h1>hello!</h1>\n";

        let mut got = String::new();

        super::handler(&mut "hello!\n".chars(), &mut got);

        assert_eq!(got, want);
    }

    #[test]
    fn handles_heading_two() {
        let want = "<h2>hello!</h2>\n";

        let mut got = String::new();

        super::handler(&mut "#hello!\n".chars(), &mut got);

        assert_eq!(got, want);
    }

    #[test]
    fn get_variant_returns_correct_variant() {
        let mut input = "## hello".chars();

        let variant = get_variant(&mut input);

        assert_eq!(variant.open_tag, "<h3>");
        assert_eq!(input.collect::<String>(), " hello");
    }
}

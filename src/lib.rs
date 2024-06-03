mod handlers;
mod scanner;
mod tokens;

pub fn parse(input: &str) -> String {
    let mut chars = input.chars();
    let mut output = String::new();

    while let Some(c) = chars.next() {
        scanner::scan_char(c, &mut chars, &mut output)
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::parse;

    #[test]
    fn it_works() {
        let input = "#hello! _what's up_\n*DAMN BRO*";
        let want = "<h1>hello! <em>what's up</em></h1>\n<b>DAMN BRO</b>";

        let got = parse(input);

        assert_eq!(got, want);
    }
}

pub struct Token {
    pub start_char: char,
    pub open_tag: &'static str,
    pub close_tag: &'static str,
    pub termination_char: char,
}

pub static HEADING_ONE_TOKEN: Token = Token {
    start_char: '#',
    open_tag: "<h1>",
    close_tag: "</h1>",
    termination_char: '\n',
};

pub static ITALIC_TOKEN: Token = Token {
    start_char: '_',
    open_tag: "<em>",
    close_tag: "</em>",
    termination_char: '_'
};

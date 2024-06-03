pub struct Token {
    pub start_char: char,
    pub open_tag: &'static str,
    pub close_tag: &'static str,
    pub termination_char: char,
}

pub static HEADING_ONE_TOKEN: Token = Token {
    start_char: '#',
    open_tag: "<h1>",
    close_tag: "</h1>\n",
    termination_char: '\n',
};

pub static ITALIC_TOKEN: Token = Token {
    start_char: '_',
    open_tag: "<em>",
    close_tag: "</em>",
    termination_char: '_',
};

pub static BOLD_TOKEN: Token = Token {
    start_char: '*',
    open_tag: "<b>",
    close_tag: "</b>",
    termination_char: '*'
};

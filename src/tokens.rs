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

pub static HEADING_TWO_TOKEN: Token = Token {
    start_char: '#',
    open_tag: "<h2>",
    close_tag: "</h2>\n",
    termination_char: '\n',
};

pub static HEADING_THREE_TOKEN: Token = Token {
    start_char: '#',
    open_tag: "<h3>",
    close_tag: "</h3>\n",
    termination_char: '\n',
};

pub static HEADING_FOUR_TOKEN: Token = Token {
    start_char: '#',
    open_tag: "<h4>",
    close_tag: "</h4>\n",
    termination_char: '\n',
};

pub static HEADING_FIVE_TOKEN: Token = Token {
    start_char: '#',
    open_tag: "<h5>",
    close_tag: "</h5>\n",
    termination_char: '\n',
};

pub static HEADING_SIX_TOKEN: Token = Token {
    start_char: '#',
    open_tag: "<h6>",
    close_tag: "</h6>\n",
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
    termination_char: '*',
};

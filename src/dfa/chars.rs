pub struct CharIdentifier {
    c: char,
}


impl CharIdentifier {
    pub fn new(c: char) -> CharIdentifier {
        CharIdentifier{c}
    }

    pub fn is_hexa(&self) -> bool {
        self.c.is_ascii_hexdigit()
    }

    pub fn is_e(&self) -> bool {
        self.c == 'e'
    }

    pub fn is_equal(&self) -> bool {
        self.c == '='
    }

    pub fn is_before_eq(&self) -> bool {
        vec!['+', '-', '*', '/', '<', '>', '=', '&', '|'].iter().any(|&ch| ch == self.c)
    }

    pub fn is_escape(&self) -> bool {
        self.c == '\\'
    }

    pub fn is_point(&self) -> bool {
        self.c == '.'
    }

    pub fn is_neg(&self) -> bool {
        self.c == '-'
    }

    pub fn is_plus(&self) -> bool {
        self.c == '+'
    }

    pub fn is_double_quote(&self) -> bool {
        self.c == '"'
    }

    pub fn is_simple_quote(&self) -> bool {
        self.c == '\''
    }

    pub fn is_hash(&self) -> bool {
        self.c == '#'
    }

    pub fn is_del_character(&self) -> bool {
        vec!['*', '+', '-', '(', ')', '[', ']', '{', '}', ':', '=', '.', ',', '>', '<', '!', ';'].iter().any(|&ch| ch == self.c)
    }

    pub fn is_tab(&self) -> bool {
        self.c == '\t'
    }

    pub fn is_c_r(&self) -> bool {
        self.c == '\r'
    }

    pub fn is_newline(&self) -> bool {
        self.c == '\n'
    }

    pub fn is_space(&self) -> bool {
        self.c == ' '
    }

    pub fn is_letter(&self) -> bool {
        self.c.is_ascii_alphabetic()
    }

    pub fn is_digit(&self) -> bool {
        self.c.is_ascii_digit()
    }

    pub fn is_first_id_char(&self) -> bool {
        self.is_letter() || self.c == '_'
    }

    pub fn is_id_char(&self) -> bool {
        self.is_first_id_char() || self.is_digit()
    }

    pub fn is_zero(&self) -> bool {
        self.c == '0'
    }

    pub fn is_x(&self) -> bool {
        self.c == 'x'
    }

}
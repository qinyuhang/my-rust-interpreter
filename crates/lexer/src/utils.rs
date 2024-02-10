// FIXME: 不支持 unicode 的中文等等字码
pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

pub fn is_valid_identifier_char(ch: char) -> bool {
    ch != '\0' && ch != '"'
}

#[allow(unused)]
pub fn is_valid_variable_prefix(ch: char) -> bool {
    // 不等于 0-9,
    true
}

pub fn is_digits(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

pub fn is_dot(ch: char) -> bool {
    ch == '.'
}

pub fn is_hex(ch: char) -> bool {
    '0' <= ch && ch <= '9' || 'a' <= ch && ch <= 'f' || 'A' <= ch && ch <= 'F'
}

pub fn is_not_decimal_symbol(ch: char) -> bool {
    ch == 'x' || ch == 'X' || ch == 'o' || ch == 'O' || ch == 'b' || ch == 'B'
}

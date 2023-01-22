// FIXME: 不支持 unicode 的中文等等字码
pub fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

pub fn is_digits(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

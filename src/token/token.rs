use std::collections::HashMap;
pub type TokenType = &'static str;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: EOF,
            literal: "".into(),
        }
    }
}

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";

pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";

pub const LT: TokenType = "<";
pub const GT: TokenType = ">";

pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";

pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FLASE";
pub const IF: TokenType = "if";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURE";

pub const EQ: TokenType = "==";
pub const NOT_EQ: TokenType = "!=";

// pub const KEYWORDS: HashMap<String, TokenType> = HashMap::new();
pub fn lookup_ident(ident: &String) -> TokenType {
    let keywords: HashMap<String, TokenType> = HashMap::from([
        ("let".into(), LET),
        ("fn".into(), FUNCTION),
        ("true".into(), TRUE),
        ("false".into(), FALSE),
        ("if".into(), IF),
        ("else".into(), ELSE),
        ("return".into(), RETURN),
    ]);

    if let Some(&t) = keywords.get(ident) {
        return t;
    }
    IDENT
}

use std::collections::HashMap;
pub type TokenType = &'static str;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Token {} {}>", self.token_type, &self.literal,)
    }
}
impl Default for Token {
    fn default() -> Self {
        Token {
            token_type: EOF,
            literal: "".into(),
        }
    }
}

pub struct ToBeToken {
    token_type: Option<TokenType>,
    literal: Option<String>,
}

impl ToBeToken {
    pub fn from_t(t: TokenType) -> Token {
        Token {
            token_type: t,
            literal: t.to_lowercase().to_string(),
        }
    }

    pub fn from_tt(t: TokenType) -> Self {
        Self {
            token_type: Some(t),
            literal: None,
        }
    }
}

impl From<ToBeToken> for Token {
    fn from(value: ToBeToken) -> Self {
        // token_type & literal 是一样的
        if let Some(ref l) = value.literal {
            let t = match l.as_str() {
                ASSIGN => Some(ASSIGN),
                PLUS => Some(PLUS),
                MINUS => Some(MINUS),
                BANG => Some(BANG),
                ASTERISK => Some(ASTERISK),
                SLASH => Some(SLASH),
                BITAND => Some(BITAND),
                BITXOR => Some(BITXOR),
                BITOR => Some(BITOR),
                POW => Some(POW),
                LOGICOR => Some(LOGICOR),
                LOGICAND => Some(LOGICAND),

                LT => Some(LT),
                GT => Some(GT),

                COMMA => Some(COMMA),
                SEMICOLON => Some(SEMICOLON),

                LPAREN => Some(LPAREN),
                RPAREN => Some(RPAREN),
                LBRACE => Some(LBRACE),
                RBRACE => Some(RBRACE),
                LBRACKET => Some(LBRACKET),
                RBRACKET => Some(RBRACKET),

                FUNCTION => Some(FUNCTION),
                LET => Some(LET),
                TRUE => Some(TRUE),
                FALSE => Some(FALSE),
                IF => Some(IF),
                ELSE => Some(ELSE),
                RETURN => Some(RETURN),

                EQ => Some(EQ),
                NOT_EQ => Some(NOT_EQ),
                STRING => Some(STRING),
                COLON => Some(COLON),
                _ => None,
            };
            if let Some(t) = t {
                return Self {
                    token_type: t,
                    literal: t.to_lowercase().to_string(),
                };
            }
        }
        // token_type 和 literal 不一样的
        if let Some(t) = value.token_type {
            if let Some(r) = match t {
                INT => value.literal.clone(),
                IDENT => value.literal.clone(),
                STRING => value.literal.clone(),
                _ => None,
            } {
                return Self {
                    token_type: t,
                    literal: r,
                };
            }
        }
        Default::default()
    }
}

pub const ILLEGAL: TokenType = "ILLEGAL";
pub const EOF: TokenType = "EOF";

pub const IDENT: TokenType = "IDENT";
pub const INT: TokenType = "INT";
pub const FLOAT: TokenType = "FLOAT";

pub const ASSIGN: TokenType = "=";
pub const PLUS: TokenType = "+";
pub const MINUS: TokenType = "-";
pub const BANG: TokenType = "!";
pub const ASTERISK: TokenType = "*";
pub const SLASH: TokenType = "/";
pub const BITAND: TokenType = "&";
pub const BITXOR: TokenType = "^";
pub const BITOR: TokenType = "|";
pub const POW: TokenType = "^^";
pub const LOGICOR: TokenType = "||";
pub const LOGICAND: TokenType = "&&";

pub const LT: TokenType = "<";
pub const GT: TokenType = ">";

pub const COMMA: TokenType = ",";
pub const SEMICOLON: TokenType = ";";

pub const LPAREN: TokenType = "(";
pub const RPAREN: TokenType = ")";
pub const LBRACE: TokenType = "{";
pub const RBRACE: TokenType = "}";
pub const LBRACKET: TokenType = "[";
pub const RBRACKET: TokenType = "]";

pub const FUNCTION: TokenType = "FUNCTION";
pub const LET: TokenType = "LET";
pub const TRUE: TokenType = "TRUE";
pub const FALSE: TokenType = "FALSE";
pub const IF: TokenType = "if";
pub const ELSE: TokenType = "ELSE";
pub const RETURN: TokenType = "RETURN";

pub const EQ: TokenType = "==";
pub const NOT_EQ: TokenType = "!=";
pub const STRING: TokenType = "STRING";
pub const COLON: TokenType = ":";
pub const WHILE: TokenType = "WHILE";
pub const FOR: TokenType = "FOR";
pub const DO: TokenType = "DO";
pub const SWITCH: TokenType = "SWITCH";
pub const CASE: TokenType = "CASE";
pub const BREAK: TokenType = "BREAK";
pub const DEFAULT: TokenType = "DEFAULT";
pub const PLUSEQ: TokenType = "+=";
pub const MINEQ: TokenType = "-=";
pub const INCREASE: TokenType = "++";
pub const DECREASE: TokenType = "--";
pub const DIVEQ: TokenType = "/=";
pub const MULEQ: TokenType = "*=";

thread_local! {
    static KEYWORDS: HashMap<String, TokenType> = HashMap::from([
        ("let".into(), LET),
        ("fn".into(), FUNCTION),
        ("true".into(), TRUE),
        ("false".into(), FALSE),
        ("if".into(), IF),
        ("else".into(), ELSE),
        ("return".into(), RETURN),
        ("while".into(), WHILE),
        ("for".into(), FOR),
        ("do".into(), DO),
        ("switch".into(), SWITCH),
        ("case".into(), CASE),
        ("break".into(), BREAK),
        ("default".into(), DEFAULT),
    ])
}
pub fn lookup_ident(ident: &String) -> TokenType {
    if let Some(&t) = KEYWORDS.with(|k| k.clone()).get(ident) {
        return t;
    }
    IDENT
}

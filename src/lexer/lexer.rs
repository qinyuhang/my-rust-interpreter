use crate::token::{self, *};
use crate::utils::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::vec::Vec;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Lexer {
    input_chars: Vec<char>,
    input: String,
    position: Cell<usize>,
    read_position: Cell<usize>,
    ch: Rc<RefCell<char>>,
}

/// ```
/// use my_rust_interpreter::lexer::Lexer;
/// let input = r#"let a = 1;"#;
/// let l = Lexer::new(input);
/// l.next_token();
/// ```
impl Lexer {
    pub fn new<T: Into<String> + Clone>(input: T) -> Self {
        let l = Lexer {
            input: input.clone().into(),
            input_chars: input.into().chars().into_iter().collect(),
            position: Cell::new(0),
            read_position: Cell::new(0),
            ch: Rc::new(RefCell::new('0')),
        };
        l.read_char();
        l
    }
    pub fn next_token(&self) -> Token {
        self.skip_white_space();
        let mut should_read_one_more = true;
        let t = *self.ch.borrow();
        let mut token_type = match t {
            '=' => {
                if self.peek_char() == "=" {
                    self.read_char();
                    token::EQ
                } else {
                    token::ASSIGN
                }
            }
            ';' => token::SEMICOLON,
            '(' => token::LPAREN,
            ')' => token::RPAREN,
            '{' => token::LBRACE,
            '}' => token::RBRACE,
            ',' => token::COMMA,
            '+' => token::PLUS,
            '-' => token::MINUS,
            '!' => {
                if self.peek_char() == "=" {
                    self.read_char();
                    token::NOT_EQ
                } else {
                    token::BANG
                }
            }
            '^' => {
                if self.peek_char() == "^" {
                    self.read_char();
                    token::POW
                } else {
                    token::BITXOR
                }
            }
            '|' => {
                if self.peek_char() == "|" {
                    self.read_char();
                    token::LOGICOR
                } else {
                    token::BITOR
                }
            }
            '&' => {
                if self.peek_char() == "&" {
                    self.read_char();
                    token::LOGICAND
                } else {
                    token::BITAND
                }
            }
            '/' => token::SLASH,
            '*' => token::ASTERISK,
            '<' => token::LT,
            '>' => token::GT,
            '"' => {
                self.read_char();
                token::STRING
            }
            '[' => token::LBRACKET,
            ']' => token::RBRACKET,
            '\0' => token::EOF,
            ':' => token::COLON,
            _ => token::IDENT,
        };
        let ch = match token_type {
            token::NOT_EQ => "!=".into(),
            token::EQ => "==".into(),
            token::POW => "^^".into(),
            token::LOGICOR => "||".into(),
            token::LOGICAND => "&&".into(),
            token::STRING => self.read_string(),
            token::LBRACKET => "[".into(),
            token::RBRACKET => "]".into(),
            token::IDENT => {
                if is_letter(*self.ch.borrow()) {
                    should_read_one_more = false;
                    let idf = self.read_identifier();
                    let lidf = token::lookup_ident(&idf);
                    match lidf {
                        token::IDENT => (),
                        _ => token_type = lidf.into(),
                    };
                    idf
                } else if is_digits(*self.ch.borrow()) {
                    // let idf = self.read_identifier();
                    should_read_one_more = false;
                    token_type = token::INT;
                    self.read_number()
                } else {
                    token_type = token::ILLEGAL;
                    self.ch.borrow().clone().into()
                }
            }
            token::EOF => '\0'.into(),
            _ => self.ch.borrow().clone().into(),
        };

        if should_read_one_more {
            self.read_char();
        }

        Token {
            token_type,
            literal: ch,
        }
    }
    pub fn read_char(&self) {
        *self.ch.borrow_mut() = if self.read_position.get() >= self.input_chars.len() {
            '\0'
        } else {
            self.input_chars[self.read_position.get()]
        };
        // if *self.ch.borrow() != '\0' {
        self.position.set(self.read_position.get());
        self.read_position.set(self.read_position.get() + 1);
        // }
    }
    pub fn peek_char(&self) -> String {
        if self.read_position.get() > self.input_chars.len() {
            "".into()
        } else {
            self.input_chars[self.read_position.get()].clone().into()
        }
    }
    pub fn read_identifier(&self) -> String {
        let position = self.position.get();
        self.read_char();
        // 先读一个，后续的可以判断做
        while is_letter(*self.ch.borrow()) {
            self.read_char();
        }
        self.input_chars[position..self.position.get()]
            .iter()
            .map(|c| c.clone().to_string())
            .collect::<String>()
    }
    pub fn read_number(&self) -> String {
        let position = self.position.get();
        let is_read_hex =
            *self.ch.borrow() == '0' && (self.peek_char() == "x" || self.peek_char() == "X");
        while is_digits(*self.ch.borrow())
            || is_not_decimal_symbol(*self.ch.borrow())
            || (is_read_hex && is_hex(*self.ch.borrow()))
            || (*self.ch.borrow()) == '_'
        {
            self.read_char();
        }
        self.input_chars[position..self.position.get()]
            .iter()
            .map(|c| c.clone().to_string())
            // 不知道这里去掉之后是否合适？
            .filter(|val| val != "_")
            .collect::<String>()
    }
    pub fn skip_white_space(&self) {
        while *self.ch.borrow() == ' '
            || *self.ch.borrow() == '\t'
            || *self.ch.borrow() == '\n'
            || *self.ch.borrow() == '\r'
        {
            self.read_char();
        }
    }
    pub fn read_string(&self) -> String {
        let position = self.position.get();
        self.read_char();
        while is_valid_identifier_char(*self.ch.borrow()) {
            self.read_char();
        }
        self.input_chars[position..self.position.get()]
            .iter()
            .map(|c| c.clone().to_string())
            .collect::<String>()
    }
}

use crate::ast::*;
use crate::lexer::*;
use crate::parser::*;
use crate::token::*;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

/// 运算符的优先级
/// 后续用来解析表达式的时候会用到
/// 比如 5 + 5 * 2，应该先运算 5 * 2
/// 或者 5 * 2 + abc(1) 应该先运算函数再运算乘法最后运算加法
#[allow(unused)]
#[derive(Clone, Copy, Debug)]
pub enum ExpressionConst {
    LOWEST = 1,  // what is this?
    EQUALS,      // =
    LESSGREATER, // > or <
    SUM,         // +￼
    PRODUCT,     // "*￼
    PREFIX,      // -X or !X￼
    CALL,        // function
}

/// 把代码中的符号与优先级关联起来了
thread_local! {
    #[allow(unused)]
    pub static PRECEDENCES: HashMap<TokenType, ExpressionConst> = HashMap::from([
        (EQ, ExpressionConst::EQUALS),
        (NOT_EQ, ExpressionConst::EQUALS),
        (LT, ExpressionConst::LESSGREATER),
        (GT, ExpressionConst::LESSGREATER),
        (PLUS, ExpressionConst::SUM),
        (MINUS, ExpressionConst::SUM),
        (SLASH, ExpressionConst::PRODUCT),
        (ASTERISK, ExpressionConst::PRODUCT),
    ]);
}

// #[derive(Clone)]
pub struct Parser {
    l: Box<Lexer>,
    cur_token: Rc<RefCell<Token>>,
    peek_token: Rc<RefCell<Token>>,
    errors: Rc<RefCell<Vec<String>>>,

    prefix_parse_fns: Rc<RefCell<HashMap<TokenType, Rc<PrefixParseFn>>>>,
    infix_parse_fns: Rc<RefCell<HashMap<TokenType, Rc<InfixParseFn>>>>,
}

impl Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            r#"Parser: {{
    l: {:?},
    cur_token: {:?},
    peek_token: {:?},
    errors: {:?},
    prefix_parse_fns: unable to stringify,
    infix_parse_fns: unable to stringify,
}}"#,
            self.l, self.cur_token, self.peek_token, self.errors
        ))
    }
}

impl Parser {
    pub fn new(l: Lexer) -> Rc<Self> {
        let p = Parser {
            l: Box::new(l),
            cur_token: Rc::new(RefCell::new(Token::default())),
            peek_token: Rc::new(RefCell::new(Token::default())),
            errors: Rc::new(RefCell::new(vec![])),
            prefix_parse_fns: Rc::new(RefCell::new(HashMap::new())),
            infix_parse_fns: Rc::new(RefCell::new(HashMap::new())),
        };
        let pc = Rc::new(p);
        let pd = pc.clone();
        pc.register_prefix(IDENT, Rc::new(move || pd.parse_identifier()));
        let pd = pc.clone();
        pc.register_prefix(INT, Rc::new(move || pd.parse_integer_literal()));

        let pd = pc.clone();
        pc.register_prefix(BANG, Rc::new(move || pd.parse_prefix_expression()));
        let pd = pc.clone();
        pc.register_prefix(MINUS, Rc::new(move || pd.parse_prefix_expression()));
        // println!("{:?}", &p.parse_identifier);

        PRECEDENCES.with(|ps| {
            // println!("before register infix_parse: {:?}", ps);
            ps.iter().for_each(|(&token, ec)| {
                let pd = pc.clone();
                // println!("register infix parser for {:?}", token);
                pc.register_infix(token, Rc::new(move |left| pd.parse_infix_expression(left)));
            });
        });

        pc.next_token();
        pc.next_token();
        pc
    }
    pub fn next_token(&self) {
        self.cur_token.replace(self.peek_token.take());
        self.peek_token.replace(self.l.next_token());
    }
    #[allow(unused)]
    pub fn parse_program(&self) -> Option<Program> {
        let mut stm = vec![];

        // println!("\nParser::parse_program {:?} {:?}\n", self.cur_token, self.peek_token);

        let mut ctk_type = self.cur_token.borrow().token_type;
        while ctk_type != EOF {
            let stmt = self.parse_statement();
            if stmt.is_some() {
                let stmt = stmt.unwrap();
                // println!("get push: {:?}", stmt);
                stm.push(stmt);
            }
            self.next_token();
            ctk_type = self.cur_token.borrow().token_type;
        }
        // println!("parse_program: {:?}", stm);
        let program = Program { statement: stm };
        Some(program)
    }
    pub fn parse_statement(&self) -> Option<Rc<dyn Statement>> {
        let cur_type = self.cur_token.borrow().token_type.clone();
        match cur_type {
            LET => self.parse_let_statement(),
            RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    pub fn parse_let_statement(&self) -> Option<Rc<dyn Statement>> {
        let cur_token = Rc::new(RefCell::new((*self.cur_token.borrow()).clone()));

        // println!("\nParser::parse_let_statement {:?} {:?}\n", self.cur_token, self.peek_token);

        if !self.expect_peek(IDENT) {
            return None;
        }
        let ct = (*self.cur_token.borrow()).clone();
        let name = Identifier {
            token: Rc::new(RefCell::new(ct.clone())),
            value: ct.literal.clone(),
        };
        // println!("\nParser::parse_let_statement2 {:?} {:?} {:?}\n", cur_token, self.peek_token, name);
        if !self.expect_peek(ASSIGN) {
            return None;
        }
        // FIXME: 这里是想给 parse 后面的 expression 的
        let mut expression = None;

        self.next_token();
        expression = self.parse_expression(ExpressionConst::LOWEST);

        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }

        // println!("\nParser::parse_let_statement3 {:?} {:?} {:?}\n", cur_token, self.peek_token, name);
        // println!("parse_let_statement return: {:?} {:?}", cur_token, name);

        Some(Rc::new(LetStatement {
            token: cur_token.clone(),
            name: Box::new(name),
            value: Box::new(ExpressionStatement {
                token: cur_token,
                // FIXME: LetStatement expression
                expression, // self.parse_expression(ExpressionConst::LOWEST), //None,
            }),
            // value: Box::new(*self.parse_expression_statement().unwrap())
        }))
    }

    pub fn parse_return_statement(&self) -> Option<Rc<dyn Statement>> {
        let cur_token = Rc::new(RefCell::new((*self.cur_token.borrow()).clone()));
        let mut expression = None;

        self.next_token();
        expression = self.parse_expression(ExpressionConst::LOWEST);

        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Rc::new(ReturnStatement {
            token: cur_token,
            return_value: expression,
        }))
    }

    fn parse_expression_statement(&self) -> Option<Rc<dyn Statement>> {
        let token = Rc::new(RefCell::new((*self.cur_token.borrow()).clone()));
        if token.borrow().literal == ";" {
            println!("parse_expression_statement")
        }
        let stm = ExpressionStatement {
            token,
            expression: self.parse_expression(ExpressionConst::LOWEST),
        };
        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Rc::new(stm))
    }
    fn parse_expression(&self, precedence: ExpressionConst) -> Option<Rc<dyn Expression>> {
        let tp = self.cur_token.borrow().token_type.to_string();
        if self.prefix_parse_fns.borrow().get(&*tp).is_none() {
            self.no_prefix_parse_fn_error();
            return None;
        }
        let mut left = None;
        if let Some(pf) = self.prefix_parse_fns.borrow().get(&*tp) {
            left = pf();
        }
        println!("before parse_infix: {:?}", left);
        while !self.peek_token_is(SEMICOLON)
            && (precedence as isize) < (self.peek_precedence() as isize)
        {
            let pktp = self.peek_token.borrow().token_type.to_string();
            if let Some(infix) = self
                .infix_parse_fns
                .borrow()
                .get(&*pktp)
            {
                self.next_token();
                left = infix(left.unwrap());
            }
        }
        return left;
        // None
    }
    // fixme: return Option is better?
    pub fn parse_identifier(&self) -> Option<Rc<dyn Expression>> {
        let token = Rc::new(RefCell::new((*self.cur_token.borrow()).clone()));
        let value = self.cur_token.borrow().literal.to_string();
        Some(Rc::new(Identifier { token, value }))
    }
    pub fn parse_integer_literal(&self) -> Option<Rc<dyn Expression>> {
        let literal = self.cur_token.borrow().literal.clone();
        if let Ok(v) = IntegerLiteral::try_from(literal) {
            Some(Rc::new(v))
        } else {
            None
        }
        // IntegerLiteral::try_from(self.cur_token.borrow().Literal.clone())
        //     .map_or_else(|_| None, move |v| Some(Rc::new(v)))
    }

    pub fn parse_prefix_expression(&self) -> Option<Rc<dyn Expression>> {
        let token = Rc::new(RefCell::new((*self.cur_token.borrow()).clone()));
        let operator = self.cur_token.borrow().literal.clone();
        let mut ex = PrefixExpression {
            token,
            operator,
            right: None,
        };
        self.next_token();
        // None
        ex.right = self.parse_expression(ExpressionConst::PREFIX);
        Some(Rc::new(ex))
    }
    pub fn parse_infix_expression(&self, left: Rc<dyn Expression>) -> Option<Rc<dyn Expression>> {
        let token = Rc::new(RefCell::new((*self.cur_token.borrow()).clone()));
        let operator = self.cur_token.borrow().literal.clone().into();

        let precedence = self.cur_precedence();
        self.next_token();

        let expression = InfixExpression {
            token,
            operator,
            left: Some(left),
            right: self.parse_expression(precedence),
        };
        Some(Rc::new(expression))
    }
    pub fn expect_peek(&self, token: TokenType) -> bool {
        let r = self.peek_token_is(token);
        if r {
            self.next_token();
        } else {
            self.peek_error(token);
        }
        r
    }
    pub fn cur_token_is(&self, token: TokenType) -> bool {
        let t = self.cur_token.borrow().token_type.clone();
        t == token
    }
    pub fn peek_token_is(&self, token: TokenType) -> bool {
        let t = self.peek_token.borrow().token_type.clone();
        t == token
    }
    pub fn errors(&self) -> Rc<RefCell<Vec<String>>> {
        self.errors.clone()
    }
    pub fn peek_error(&self, t: TokenType) {
        let msg = self.peek_token.borrow().token_type;
        let msg = format!("expect next token to be {}, got {} instead", t, msg);
        self.errors.borrow_mut().push(msg);
    }
    pub fn register_prefix(&self, token: TokenType, the_fn: Rc<PrefixParseFn>) {
        self.prefix_parse_fns.borrow_mut().insert(token, the_fn);
    }
    pub fn register_infix(&self, token: TokenType, the_fn: Rc<InfixParseFn>) {
        self.infix_parse_fns.borrow_mut().insert(token, the_fn);
    }
    pub fn no_prefix_parse_fn_error(&self) {
        self.errors.borrow_mut().push(format!(
            "Cannot found prefix_parse_fn for {}",
            self.cur_token.borrow().token_type.to_string()
        ));
    }
    pub fn peek_precedence(&self) -> ExpressionConst {
        let mut r = ExpressionConst::LOWEST;
        PRECEDENCES.with(|val| {
            let tp = self.peek_token.borrow().token_type.clone();
            if let Some(rs) = val.get(tp) {
                r = *rs;
            }
        });
        r
    }
    pub fn cur_precedence(&self) -> ExpressionConst {
        let mut r = ExpressionConst::LOWEST;
        PRECEDENCES.with(|val| {
            let tp = self.cur_token.borrow().token_type.clone();
            if let Some(rs) = val.get(tp) {
                r = *rs;
            }
        });
        r
    }
}

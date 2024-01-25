use crate::ExpressionConst::LOWEST;
use crate::InfixParseFn;
use crate::PrefixParseFn;
use ::lexer::*;
use ::token::*;
use ast::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// 运算符的优先级
/// 后续用来解析表达式的时候会用到
/// 比如 5 + 5 * 2，应该先运算 5 * 2
/// 或者 5 * 2 + abc(1) 应该先运算函数再运算乘法最后运算加法
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum ExpressionConst {
    LOWEST = 1,
    // what is this?
    EQUALS,
    // =
    LESSGREATER,
    // > or <
    SUM,
    // +
    PRODUCT,
    // "*
    PREFIX,
    // -X or !X
    BITOP,
    // ^ or | or &
    LOGICOP,
    // && or ||
    POW,
    // ^^
    CALL,  // function
    INDEX, // a[1]
}

impl From<isize> for ExpressionConst {
    fn from(value: isize) -> Self {
        match value {
            1 => ExpressionConst::LOWEST,      // what is this?
            2 => ExpressionConst::EQUALS,      // =
            3 => ExpressionConst::LESSGREATER, // > or <
            4 => ExpressionConst::SUM,         // +
            5 => ExpressionConst::PRODUCT,     // "*
            6 => ExpressionConst::PREFIX,      // -X or !X
            7 => ExpressionConst::BITOP,       // ^ or | or &
            8 => ExpressionConst::LOGICOP,     // && or ||
            9 => ExpressionConst::POW,         // ^^
            10 => ExpressionConst::CALL,       // function
            11 => ExpressionConst::INDEX,      // a[1]
            _ => ExpressionConst::LOWEST,
        }
    }
}

// 把代码中的符号与优先级关联起来了
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

        (BITAND, ExpressionConst::BITOP),
        (BITOR, ExpressionConst::BITOP),
        (BITXOR, ExpressionConst::BITOP),

        (LOGICAND, ExpressionConst::LOGICOP),
        (LOGICOR, ExpressionConst::LOGICOP),

        (POW, ExpressionConst::POW),

        (ASTERISK, ExpressionConst::PRODUCT),
        (LPAREN, ExpressionConst::CALL),
        (LBRACKET, ExpressionConst::INDEX),
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

impl std::fmt::Display for Parser {
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
        pc.register_prefix(FLOAT, Rc::new(move || pd.parse_float_literal()));

        let pd = pc.clone();
        pc.register_prefix(BANG, Rc::new(move || pd.parse_prefix_expression()));
        let pd = pc.clone();
        pc.register_prefix(MINUS, Rc::new(move || pd.parse_prefix_expression()));

        let pd = pc.clone();
        pc.register_prefix(BITAND, Rc::new(move || pd.parse_prefix_expression()));
        let pd = pc.clone();
        pc.register_prefix(BITOR, Rc::new(move || pd.parse_prefix_expression()));
        let pd = pc.clone();
        pc.register_prefix(POW, Rc::new(move || pd.parse_prefix_expression()));

        // println!("{:?}", &p.parse_identifier);
        let pd = pc.clone();
        pc.register_prefix(FUNCTION, Rc::new(move || pd.parse_function_literal()));
        let pd = pc.clone();
        pc.register_prefix(TRUE, Rc::new(move || pd.parse_boolean()));
        let pd = pc.clone();
        pc.register_prefix(FALSE, Rc::new(move || pd.parse_boolean()));
        let pd = pc.clone();
        pc.register_prefix(LPAREN, Rc::new(move || pd.parse_grouped_expression()));
        let pd = pc.clone();
        pc.register_prefix(IF, Rc::new(move || pd.parse_if_expression()));
        let pd = pc.clone();
        pc.register_prefix(STRING, Rc::new(move || pd.parse_string_literal()));

        let pd = pc.clone();
        pc.register_prefix(LBRACKET, Rc::new(move || pd.parse_array_literal()));
        let pd = pc.clone();
        pc.register_prefix(LBRACE, Rc::new(move || pd.parse_hash_literal()));

        let pd = pc.clone();
        pc.register_infix(LPAREN, Rc::new(move |val| pd.parse_call_expression(val)));

        let pd = pc.clone();
        pc.register_infix(LBRACKET, Rc::new(move |val| pd.parse_index_expression(val)));

        // let pd = pc.clone();
        // pc.register_prefix(IF, Rc::new(move || pd.parse_block_statement()));

        PRECEDENCES.with(|ps| {
            // println!("before register infix_parse: {:?}", ps);
            #[allow(unused_variables)]
            ps.iter().for_each(|(&token, ec)| {
                if token == LPAREN {
                    return;
                }
                if token == LBRACKET {
                    return;
                }
                let pd = pc.clone();
                // println!("register infix lang_parser for {:?}", token);
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
    pub fn parse_statement(&self) -> Option<Rc<AstExpression>> {
        let cur_type = self.cur_token.borrow().token_type;
        match cur_type {
            LET => self.parse_let_statement(),
            RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    pub fn parse_let_statement(&self) -> Option<Rc<AstExpression>> {
        let cur_token = (*self.cur_token.borrow()).clone();

        // println!("\nParser::parse_let_statement {:?} {:?}\n", self.cur_token, self.peek_token);

        if !self.expect_peek(IDENT) {
            return None;
        }
        let ct = (*self.cur_token.borrow()).clone();
        let name = Identifier {
            token: ct.clone(),
            value: ct.literal.clone(),
        };
        // println!("\nParser::parse_let_statement2 {:?} {:?} {:?}\n", cur_token, self.peek_token, name);
        if !self.expect_peek(ASSIGN) {
            return None;
        }
        // FIXME: 这里是想给 parse 后面的 expression 的
        #[allow(unused_assignments)]
        let mut expression = None;

        self.next_token();
        expression = self.parse_expression(ExpressionConst::LOWEST);

        // while !self.cur_token_is(SEMICOLON) {
        //     self.next_token();
        // }
        if !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }

        // println!("\nParser::parse_let_statement3 {:?} {:?} {:?}\n", cur_token, self.peek_token, name);
        // println!("parse_let_statement return: {:?} {:?}", cur_token, name);

        Some(Rc::new(AstExpression::LetStatement(LetStatement {
            token: cur_token.clone(),
            name: Rc::new(name),

            // FIXME: make it clone
            value: Some(Rc::new(AstExpression::ExpressionStatement(
                ExpressionStatement {
                    token: cur_token,
                    // FIXME: LetStatement expression
                    expression, // self.parse_expression(ExpressionConst::LOWEST), //None,
                },
            ))),
            // value: Box::new(*self.parse_expression_statement().unwrap())
        })))
    }

    pub fn parse_return_statement(&self) -> Option<Rc<AstExpression>> {
        let cur_token = (*self.cur_token.borrow()).clone();
        #[allow(unused_assignments)]
        let mut expression = None;

        self.next_token();
        expression = self.parse_expression(ExpressionConst::LOWEST);

        while !self.cur_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Rc::new(AstExpression::ReturnStatement(ReturnStatement {
            token: cur_token,
            return_value: expression,
        })))
    }

    fn parse_expression_statement(&self) -> Option<Rc<AstExpression>> {
        let token = (*self.cur_token.borrow()).clone();
        if token.literal == ";" {
            // println!("parse_expression_statement")
        }
        let stm = ExpressionStatement {
            token,
            expression: self.parse_expression(ExpressionConst::LOWEST),
        };
        if self.peek_token_is(SEMICOLON) {
            self.next_token();
        }
        Some(Rc::new(AstExpression::ExpressionStatement(stm)))
    }
    fn parse_expression(&self, precedence: ExpressionConst) -> Option<Rc<AstExpression>> {
        let tp = self.cur_token.borrow().token_type.to_string();
        // println!("parse_expression {}", &self.cur_token.borrow());
        let pfs = self.prefix_parse_fns.borrow();
        let pf = pfs.get(&*tp);
        if pf.is_none() {
            self.no_prefix_parse_fn_error();
            return None;
        }
        let pf = pf.unwrap();
        let mut left = pf();
        // println!("before parse_infix: {:?}", left);
        while !self.peek_token_is(SEMICOLON) && precedence < self.peek_precedence() {
            let pktp = self.peek_token.borrow().token_type.to_string();
            if let Some(infix) = self.infix_parse_fns.borrow().get(&*pktp) {
                self.next_token();
                left = infix(left.unwrap());
            } else {
                return left;
            }
        }
        // println!("after parse_infix: {:?}", left);
        return left;
        // None
    }
    // fixme: return Option is better?
    pub fn parse_identifier(&self) -> Option<Rc<AstExpression>> {
        let token = (*self.cur_token.borrow()).clone();
        let value = self.cur_token.borrow().literal.to_string();
        Some(Rc::new(AstExpression::Identifier(Identifier {
            token,
            value,
        })))
    }
    pub fn parse_integer_literal(&self) -> Option<Rc<AstExpression>> {
        let literal = self.cur_token.borrow().literal.clone();
        if let Ok(v) = IntegerLiteral::try_from(literal) {
            Some(Rc::new(AstExpression::IntegerLiteral(v)))
        } else {
            None
        }
        // IntegerLiteral::try_from(self.cur_token.borrow().Literal.clone())
        //     .map_or_else(|_| None, move |v| Some(Rc::new(v)))
    }

    pub fn parse_float_literal(&self) -> Option<Rc<AstExpression>> {
        let literal = self.cur_token.borrow().literal.clone();
        if let Ok(v) = FloatLiteral::try_from(literal) {
            Some(Rc::new(AstExpression::FloatLiteral(v)))
        } else {
            None
        }
    }

    pub fn parse_prefix_expression(&self) -> Option<Rc<AstExpression>> {
        let token = (*self.cur_token.borrow()).clone();
        let operator = self.cur_token.borrow().literal.clone();
        let mut ex = PrefixExpression {
            token,
            operator,
            right: None,
        };
        self.next_token();
        // None
        ex.right = self.parse_expression(ExpressionConst::PREFIX);
        Some(Rc::new(AstExpression::PrefixExpression(ex)))
    }
    pub fn parse_infix_expression(&self, left: Rc<AstExpression>) -> Option<Rc<AstExpression>> {
        let token = (*self.cur_token.borrow()).clone();
        let operator = self.cur_token.borrow().literal.clone().into();

        let precedence = self.cur_precedence();
        self.next_token();

        #[allow(unused_mut)]
        let mut right = self.parse_expression(precedence);
        let left = Some(left);

        let expression = InfixExpression {
            token,
            operator,
            left,
            right,
        };
        // println!("parse_infix_expression result: {:?}", expression);
        Some(Rc::new(AstExpression::InfixExpression(expression)))
    }
    pub fn parse_boolean(&self) -> Option<Rc<AstExpression>> {
        Some(Rc::new(AstExpression::BooleanLiteral(BooleanLiteral {
            token: (*self.cur_token.borrow()).clone(),
            value: self.cur_token_is(TRUE),
        })))
    }
    pub fn parse_grouped_expression(&self) -> Option<Rc<AstExpression>> {
        self.next_token();

        let exp = self.parse_expression(ExpressionConst::LOWEST);
        self.expect_peek(RPAREN).then(|| exp.unwrap())
    }
    pub fn parse_if_expression(&self) -> Option<Rc<AstExpression>> {
        let token = (*self.cur_token.borrow()).clone();

        if !self.expect_peek(LPAREN) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(ExpressionConst::LOWEST);
        if condition.is_none() {
            return None;
        }
        if !self.expect_peek(RPAREN) {
            return None;
        }

        if !self.expect_peek(LBRACE) {
            return None;
        }

        let consequence = self.parse_block_statement();
        let mut alternative = None;

        if self.peek_token_is(ELSE) {
            self.next_token();
            self.expect_peek(LBRACE).then(|| {
                alternative = self.parse_block_statement();
            });
        }

        let expression = IfExpression {
            token,
            condition: condition.unwrap(),
            consequence,
            alternative,
        };
        Some(Rc::new(AstExpression::IfExpression(expression)))
    }
    pub fn parse_block_statement(&self) -> Option<Rc<AstExpression>> {
        let mut statement = vec![];
        let token = (*self.cur_token.borrow()).clone();

        self.next_token();

        while !self.cur_token_is(RBRACE) && !self.cur_token_is(EOF) {
            let stm = self.parse_statement();
            stm.map(|val| statement.push(val));
            self.next_token();
        }

        // 这段代码，在书里面没有。我debug之后发现报错没有解析 } 的函数，因此认为这里应该要把这个token消费掉
        // if self.cur_token_is(RBRACE) {
        //     self.next_token();
        // }

        Some(Rc::new(AstExpression::BlockStatement(BlockStatement {
            token,
            statement,
        })))
    }
    pub fn parse_function_literal(&self) -> Option<Rc<AstExpression>> {
        let token = (*self.cur_token.borrow()).clone();

        // function name
        let mut name = None;

        if self.peek_token_is(IDENT) {
            self.next_token();
            name = Some(Rc::new(Identifier {
                token: (*self.cur_token.borrow()).clone(),
                value: self.cur_token.borrow().literal.clone(),
            }));
            // println!("function name: {}", name.as_ref().unwrap());
        }
        if !self.expect_peek(LPAREN) {
            return None;
        }

        let parameters = self.parse_function_parameters();

        if !self.expect_peek(LBRACE) {
            return None;
        }

        let body = self.parse_block_statement();

        let lit = FunctionLiteral {
            token,
            name,
            parameters,
            body,
        };
        Some(Rc::new(AstExpression::FunctionLiteral(lit)))
    }
    pub fn parse_function_parameters(&self) -> Option<Vec<Rc<Identifier>>> {
        let mut identifiers = vec![];

        if self.peek_token_is(RPAREN) {
            self.next_token();
            return Some(identifiers);
        }

        self.next_token();

        let ident = Rc::new(Identifier {
            token: (*self.cur_token.borrow()).clone(),
            value: self.cur_token.borrow().literal.clone(),
        });

        identifiers.push(ident);

        // let token = Rc::new(RefCell::new((*self.cur_token.borrow()).clone()));

        // let value = self.cur_token.borrow().literal.clone();
        // let ident =

        while self.peek_token_is(COMMA) {
            self.next_token();
            self.next_token();
            // fixme fn as p
            let ident = Rc::new(Identifier {
                token: (*self.cur_token.borrow()).clone(),
                value: self.cur_token.borrow().literal.clone(),
            });
            identifiers.push(ident);
        }

        if !self.expect_peek(RPAREN) {
            return None;
        }

        Some(identifiers)
    }
    pub fn parse_call_expression(&self, f: Rc<AstExpression>) -> Option<Rc<AstExpression>> {
        // println!("\n\nparse_call_expression\n\n{:?}", "args");

        let token = (*self.cur_token.borrow()).clone();
        let args = self.parse_expression_list(RPAREN);

        Some(Rc::new(AstExpression::CallExpression(CallExpression {
            token,
            arguments: Some(args),
            function: Some(f.clone()),
        })))
    }
    pub fn parse_string_literal(&self) -> Option<Rc<AstExpression>> {
        let literal = self.cur_token.borrow().literal.clone();
        if let Ok(v) = StringLiteral::try_from(literal) {
            Some(Rc::new(AstExpression::StringLiteral(v)))
        } else {
            None
        }
    }
    pub fn parse_array_literal(&self) -> Option<Rc<AstExpression>> {
        // let mut list = vec![];
        // Some(Rc::new())
        let literal = self.cur_token.borrow().literal.clone();
        let arr = ArrayLiteral {
            token: Token {
                literal,
                token_type: token::LBRACKET,
            },
            elements: self.parse_expression_list(token::RBRACKET),
        };
        Some(Rc::new(AstExpression::ArrayLiteral(arr)))
    }
    pub fn parse_hash_literal(&self) -> Option<Rc<AstExpression>> {
        let literal = self.cur_token.borrow().literal.clone();
        let mut pairs = HashMap::new();

        while !self.peek_token_is(RBRACE) {
            self.next_token();
            let k = self.parse_expression(LOWEST);
            if !self.expect_peek(COLON) {
                return None;
            };
            self.next_token();
            let v = self.parse_expression(LOWEST);
            pairs.insert(k.unwrap(), v.unwrap());

            if !self.peek_token_is(RBRACE) && !self.expect_peek(COMMA) {
                return None;
            }
        }
        if !self.expect_peek(RBRACE) {
            return None;
        }
        Some(Rc::new(AstExpression::HashLiteral(HashLiteral {
            token: Token {
                literal,
                token_type: token::LBRACE,
            },
            pairs: RefCell::new(pairs),
        })))
        // None
    }
    pub fn parse_expression_list(&self, end: TokenType) -> Vec<Rc<AstExpression>> {
        let mut r = vec![];
        if self.peek_token_is(end) {
            self.next_token();
            return r;
        }
        self.next_token();
        r.push(self.parse_expression(LOWEST).unwrap());

        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            if let Some(e) = self.parse_expression(LOWEST) {
                r.push(e);
            }
        }
        if !self.expect_peek(end) {
            return vec![];
        }
        return r;
    }
    pub fn parse_index_expression(&self, left: Rc<AstExpression>) -> Option<Rc<AstExpression>> {
        let literal = self.cur_token.borrow().literal.clone();
        self.next_token();
        let index = self.parse_expression(LOWEST);
        if !self.expect_peek(token::RBRACKET) {
            return None;
        }
        return Some(Rc::new(AstExpression::IndexExpression(IndexExpression {
            token: Token {
                literal,
                token_type: token::LBRACKET,
            },
            left: left.clone(),
            index: index.unwrap(),
        })));
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
        let t = self.cur_token.borrow().token_type;
        t == token
    }
    pub fn peek_token_is(&self, token: TokenType) -> bool {
        let t = self.peek_token.borrow().token_type;
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
    pub fn register_prefix(&self, token: TokenType, f: Rc<PrefixParseFn>) {
        self.prefix_parse_fns.borrow_mut().insert(token, f);
    }
    pub fn register_infix(&self, token: TokenType, f: Rc<InfixParseFn>) {
        self.infix_parse_fns.borrow_mut().insert(token, f);
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
            let tp = self.peek_token.borrow().token_type;
            if let Some(rs) = val.get(tp) {
                r = *rs;
            }
        });
        r
    }
    pub fn cur_precedence(&self) -> ExpressionConst {
        let mut r = ExpressionConst::LOWEST;
        PRECEDENCES.with(|val| {
            let tp = self.cur_token.borrow().token_type;
            if let Some(rs) = val.get(tp) {
                r = *rs;
            }
        });
        r
    }
}

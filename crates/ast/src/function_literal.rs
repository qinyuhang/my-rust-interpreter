use crate::*;
use ::token::*;
use std::rc::Rc;

#[ast_node(Expression)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Option<Vec<Rc<Identifier>>>,
    // blockStatement
    pub body: Option<Rc<AstExpression>>,
    // function name
    pub name: Option<Rc<Identifier>>,
    // pub body: Option<BlockStatement>,
}

impl std::fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{} {}({}) {}",
                self.token_literal(),
                format!(
                    "{}",
                    self.name
                        .as_ref()
                        .map_or("".into(), |val| val.to_string() + " ")
                ),
                (*self
                    .parameters
                    .as_ref()
                    .map_or_else(|| vec![], |v| v.to_vec()))
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", "),
                self.body
                    .as_ref()
                    .map_or_else(|| "".into(), |v| v.to_string())
            )
        )
    }
}

#[cfg(test)]
mod test {
    use ::lexer::*;
    use ::parser::*;

    #[test]
    fn test_function_literal_to_string() {
        let input = r#"fn z (x, y, z) {  }"#;
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        // println!("{}", pr.as_ref().unwrap().to_string());
        assert_eq!(pr.as_ref().unwrap().to_string(), input);
        // let fl = FunctionLiteral {
        //     token: Rc::new(RefCell::new(Token {
        //         token_type: FUNCTION,
        //         literal: "fn".into(),
        //     })),
        //     parameters: Rc::new(RefCell::new(vec![])),
        //     body: Rc::new(RefCell::new(BlockStatement {
        //         token: Rc::new(RefCell::new(Token {
        //             token_type: EOF,
        //             literal: '\0'.into(),
        //         })),
        //         statement: Rc::new(RefCell::new(vec![Rc::new(LetStatement {
        //             token: Rc::new(RefCell::new(Token {
        //                 token_type: EOF,
        //                 literal: '\0'.into(),
        //             })),
        //             name: Box::new(Identifier {
        //                 token: Rc::new(RefCell::new(Token {
        //                     token_type: IDENT,
        //                     literal: 'a'.into(),
        //                 })),
        //                 value: 'a'.into(),
        //             }),
        //             value: Box::new(IntegerLiteral {
        //                 token: Rc::new(RefCell::new(Token {
        //                     token_type: EOF,
        //                     literal: '\0'.into(),
        //                 })),
        //                 value: 5,
        //             }),
        //         })])),
        //     })),
        // };
        // println!("function literal: {}", fl);
    }
}

use crate::ast::*;
use crate::token::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Option<Vec<Identifier>>,
    pub body: Option<Rc<dyn Statement>>,
    // pub body: Option<BlockStatement>,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Expression for FunctionLiteral {
    fn expression_node(&self) {
        todo!()
    }
    fn upcast(&self) -> &dyn Node {
        self
    }
}

impl std::fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{}({}) {}",
                self.token_literal(),
                (*self.parameters.as_ref().map_or_else(|| vec![], |v| v.to_vec()))
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                self.body
                    .as_ref()
                    .map_or_else(|| "".into(), |v| v.to_string())
            )
        )
    }
}

mod test {
    #[allow(unused)]
    use {crate::ast::*, crate::token::*, std::cell::RefCell, std::rc::Rc};

    #[test]
    fn test_function_literal_to_string() {
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

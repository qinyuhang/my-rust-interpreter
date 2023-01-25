use crate::ast::*;
use crate::token::Token;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Rc<dyn Expression>>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl TryFrom<Box<&dyn Expression>> for PrefixExpression {
    type Error = String;

    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        let x = value.as_any();
        if x.is::<PrefixExpression>() {
            println!("Object is PrefixExpression {:?}", value);
            let x = x.downcast_ref::<PrefixExpression>().unwrap();
            return Ok(PrefixExpression {
                token: x.token.clone(),
                operator: x.operator.clone(),
                right: x.right.clone(),
            });
        }
        Err(format!("Cannot cast {:?} to PrefixExpression", value))
    }
}
impl std::fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right.as_ref().unwrap())
    }
}

mod test {
    #[allow(unused)]
    use {
        crate::{
            ast::*,
            token::*,
            *,
        },
        std::{cell::RefCell, rc::Rc},
    };

    #[test]
    fn test_prefix_expression_to_string() {
        let s = PrefixExpression {
            token: Token {
                literal: "".into(),
                token_type: "",
            },
            operator: "-".into(),
            right: Some(Rc::new(IntegerLiteral {
                token: Token {
                    literal: "".into(),
                    token_type: "",
                },
                value: 5,
            })),
        };
        // println!("\n\n{}", s);
        assert_eq!(format!("{s}"), "(-5)");
    }
}

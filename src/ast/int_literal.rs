// use std::cell::RefCell;
// use std::rc::Rc;

use crate::ast::{Expression, Node, *};
/// 定义了整数字面量
///
use crate::token::*;

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {
        todo!()
    }
}

impl TryFrom<String> for IntegerLiteral {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(v) = value.clone().parse::<i64>() {
            return Ok(IntegerLiteral {
                token: Token {
                    token_type: INT,
                    literal: value,
                },
                value: v,
            });
        }
        Err(format!("can not parse {} into IntegerLiteral", value))
    }
}

impl TryFrom<Box<&ExpressionStatement>> for IntegerLiteral {
    type Error = String;

    fn try_from(value: Box<&ExpressionStatement>) -> Result<Self, Self::Error> {
        println!("the try from: {:?}", value);
        if value.token.token_type == INT {
            return Ok(IntegerLiteral {
                token: value.token.clone(),
                // 这里过不去是因为 expression_statement 的 try_from 还没完成，expression_statement.expression是 None
                value: value
                    .expression
                    .as_ref()
                    .map(|x| x.as_any().downcast_ref::<Self>().unwrap().value)
                    .unwrap(),
            });
        }
        Err(format!("error cast object {:?}", value))
    }
}

impl TryFrom<Box<&dyn Expression>> for IntegerLiteral {
    type Error = String;
    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        println!("wtf wtf: {:?}", value);
        let x = value.as_any();
        if x.is::<Self>() {
            // println!("x is IntegerLiteral {:?}", x);
            let x = x.downcast_ref::<Self>().unwrap();
            return Ok(IntegerLiteral {
                token: x.token.clone(),
                value: x.value,
            });
        }
        // FIXME: PrefixExpression 可以转换成 IntegerLiteral 吗？
        // if x.is::<PrefixExpression>() {
        //     let x = x.downcast_ref::<PrefixExpression>().unwrap();
        // }
        // println!("{}", format!("Cannot cast {:?} into IntegerLiteral", value));
        Err(format!("Cannot cast {:?} into IntegerLiteral", value))
    }
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

mod test {

    #[allow(unused)]
    use {
        crate::{
            ast::IntegerLiteral,
            token::{Token, INT},
        },
        // std::{cell::RefCell, rc::Rc},
    };

    #[test]
    fn test_int_literal_print() {
        let s = IntegerLiteral {
            token: Token {
                literal: "5".into(),
                token_type: INT,
            },
            value: 5,
        };
        assert_eq!(format!("{s}"), "5");
    }
}

use std::ops::{Add, Mul};
#[allow(dead_code)]
pub(crate) fn test_literal_expression<N>(exp: Box<&dyn Expression>, expected: N) -> bool
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy + Display,
{
    println!("{}{}", exp, expected);
    true
    // return test_integer_literal(exp, expected);
}

// pub(crate) fn 
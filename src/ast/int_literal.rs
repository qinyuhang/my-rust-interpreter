// use std::cell::RefCell;
// use std::rc::Rc;

use crate::ast::{Expression, Node, *};
/// 定义了整数字面量
///
use crate::token::*;

#[ast_node(Expression)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl TryFrom<String> for IntegerLiteral {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut radix = 10;
        let mut v = value.clone();
        if value.starts_with("0x") || value.starts_with("0X") {
            radix = 16;
            v = value.replace("0x", "").replace("0X", "");
        }
        if value.starts_with("0b") || value.starts_with("0B") {
            radix = 2;
            v = value.replace("0b", "").replace("0B", "");
        }
        if value.starts_with("0o") || value.starts_with("0O") {
            radix = 8;
            v = value.replace("0o", "").replace("0O", "");
        }

        if let Ok(v) = i64::from_str_radix(&v, radix) {
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
        //     if x.operator == "-" || x.operator == "+" {}
        // }
        println!("{}", format!("Cannot cast {:?} into IntegerLiteral", value));
        Err(format!("Cannot cast {:?} into IntegerLiteral", value))
    }
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::IntegerLiteral,
        token::{Token, INT},
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

    #[test]
    fn test_int_literal_try_from() {
        let cases = vec![("1", 1), ("-1", -1)];
        cases.iter().for_each(|&(input, out)| {
            let r = IntegerLiteral::try_from(input.to_string());
            assert!(r.is_ok());
            let r = r.unwrap();
            // dbg!(&r);
            assert_eq!(r.value, out);
        });
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

use crate::*;
use ::token::*;
use std::rc::Rc;

#[ast_node(Expression)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
// #[derive(PartialEq, Eq, Hash)]
pub struct StringLiteral {
    pub token: Rc<Token>,
    pub value: Rc<String>,
}

impl TryFrom<String> for StringLiteral {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = Rc::new(value);
        Ok(Self {
            token: Rc::new(Token {
                token_type: STRING,
                literal: value.clone(),
            }),
            value,
        })
    }
}

impl TryFrom<Rc<String>> for StringLiteral {
    type Error = String;

    fn try_from(value: Rc<String>) -> Result<Self, Self::Error> {
        let value = value.clone();
        Ok(Self {
            token: Rc::new(Token {
                token_type: STRING,
                literal: value.clone(),
            }),
            value,
        })
    }
}

impl TryFrom<Box<&ExpressionStatement>> for StringLiteral {
    type Error = String;

    fn try_from(value: Box<&ExpressionStatement>) -> Result<Self, Self::Error> {
        // println!("the try from: {:?}", value);
        if value.token.token_type == STRING {
            return Ok(Self {
                token: value.token.clone(),
                // 这里过不去是因为 expression_statement 的 try_from 还没完成，expression_statement.expression是 None
                value: value
                    .expression
                    .as_ref()
                    .map(|x| x.as_any().downcast_ref::<Self>().unwrap().value.clone())
                    .unwrap(),
            });
        }
        Err(format!("error cast object {:?}", value))
    }
}

impl std::fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use ::token::*;
    use std::rc::Rc;

    #[test]
    fn test_int_literal_print() {
        let s = StringLiteral {
            token: Rc::new(Token {
                literal: Rc::new("5".into()),
                token_type: STRING,
            }),
            value: Rc::new("5".into()),
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

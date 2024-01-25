use crate::*;
use ::token::*;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[ast_node(Expression)]
#[derive(Hash)]
pub struct FloatLiteral {
    pub token: Token,
    pub value: WrapF64,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct WrapF64(pub f64);
impl Hash for WrapF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert the f64 to bits and hash the bits
        self.0.to_bits().hash(state);
    }
}

impl Eq for WrapF64 {}

impl std::fmt::Display for WrapF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for FloatLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl TryFrom<Box<&AstExpression>> for FloatLiteral {
    type Error = String;
    fn try_from(value: Box<&AstExpression>) -> Result<Self, Self::Error> {
        return match *value {
            AstExpression::FloatLiteral(value) => {
                let x = value.as_any();
                if x.is::<Self>() {
                    // println!("x is IntegerLiteral {:?}", x);
                    let x = x.downcast_ref::<Self>().unwrap();
                    return Ok(Self {
                        token: x.token.clone(),
                        value: x.value.clone(),
                    });
                }
                Err(format!("Cannot cast {:?} into IntegerLiteral", value))
            }
            AstExpression::ExpressionStatement(value) => {
                if let Some(value) = value.expression.clone() {
                    return Self::try_from(Box::new(&value.as_ref().clone()));
                }
                Err(format!("error cast object {:?}", value))
            }
            _ => Err(format!("Cannot cast {:?} into IntegerLiteral", value)),
        };
    }
}
impl TryFrom<String> for FloatLiteral {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Ok(v) = f64::from_str(&value) {
            return Ok(FloatLiteral {
                token: Token {
                    token_type: FLOAT,
                    literal: value,
                },
                value: float_literal::WrapF64(v),
            });
        }
        Err(format!("can not parse {} into FloatLiteral", value))
    }
}

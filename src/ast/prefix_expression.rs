use crate::ast::*;
use crate::token::*;

use std::rc::Rc;

#[ast_node(Expression)]
#[derive(Hash)]
// #[derive(Eq, PartialEq, Hash)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Rc<AstExpression>>,
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

impl TryFrom<Box<&AstExpression>> for PrefixExpression {
    type Error = String;

    fn try_from(value: Box<&AstExpression>) -> Result<Self, Self::Error> {
        return match *value {
            AstExpression::PrefixExpression(value) => {
                let x = value.as_any();
                if x.is::<Self>() {
                    let x = x.downcast_ref::<Self>().unwrap();
                    return Ok(PrefixExpression {
                        token: x.token.clone(),
                        operator: x.operator.clone(),
                        right: x.right.clone(),
                    });
                }
                Err(format!("Cannot cast {:?} to PrefixExpression", value))
            }
            AstExpression::ExpressionStatement(value) => {
                if let Some(value) = value.expression.clone() {
                    return Self::try_from(Box::new(&value.as_ref().clone()));
                }
                Err(format!("Cannot cast {:?} to PrefixExpression", value))
            }
            _ => Err(format!("Cannot cast {:?} to PrefixExpression", value)),
        };
    }
}
impl std::fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}{})", self.operator, self.right.as_ref().unwrap())
    }
}

#[cfg(test)]
mod test {
    use {crate::*, std::rc::Rc};

    #[test]
    fn test_prefix_expression_to_string() {
        let s = PrefixExpression {
            token: Token {
                literal: "".into(),
                token_type: "",
            },
            operator: "-".into(),
            right: Some(Rc::new(AstExpression::IntegerLiteral(IntegerLiteral {
                token: Token {
                    literal: "".into(),
                    token_type: "",
                },
                value: 5,
            }))),
        };
        // println!("\n\n{}", s);
        assert_eq!(format!("{s}"), "(-5)");
    }
}

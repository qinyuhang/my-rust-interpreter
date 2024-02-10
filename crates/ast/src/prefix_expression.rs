use crate::*;
use ::token::*;

use std::rc::Rc;

#[ast_node(Expression)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
// #[derive(Eq, PartialEq, Hash)]
pub struct PrefixExpression {
    pub token: Rc<Token>,
    pub operator: Rc<String>,
    pub right: Option<Rc<AstExpression>>,
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
    use token::Token;
    use {crate::*, std::rc::Rc};

    #[test]
    fn test_prefix_expression_to_string() {
        let s = PrefixExpression {
            token: Rc::new(Token {
                literal: Rc::new("".into()),
                token_type: "",
            }),
            operator: Rc::new("-".into()),
            right: Some(Rc::new(AstExpression::IntegerLiteral(IntegerLiteral {
                token: Rc::new(Token {
                    literal: Rc::new("".into()),
                    token_type: "",
                }),
                value: 5,
            }))),
        };
        // println!("\n\n{}", s);
        assert_eq!(format!("{s}"), "(-5)");
    }
}

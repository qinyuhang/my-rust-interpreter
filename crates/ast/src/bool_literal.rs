use crate::*;
use ::token::*;
use std::any::Any;
use std::rc::Rc;

#[ast_node(Expression)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
pub struct BooleanLiteral {
    pub token: Rc<Token>,
    pub value: bool,
}

// impl std::fmt::Display for BooleanLiteral {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.token.literal.clone())
//     }
// }
impl TryFrom<&str> for BooleanLiteral {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "true" => Ok(BooleanLiteral {
                token: ToBeToken::from_ttt(TRUE),
                value: true,
            }),
            "false" => Ok(BooleanLiteral {
                token: ToBeToken::from_ttt(FALSE),
                value: false,
            }),
            _ => Err("can't cast {} into BoolLiteral".into()),
        }
    }
}

impl TryFrom<Box<&AstExpression>> for BooleanLiteral {
    type Error = String;

    fn try_from(value: Box<&AstExpression>) -> Result<Self, Self::Error> {
        if let AstExpression::BooleanLiteral(value) = *value {
            let v_any = value.as_any();
            if v_any.is::<Self>() {
                if let Some(val) = v_any.downcast_ref::<Self>() {
                    return Ok(Self {
                        token: val.token.clone(),
                        value: val.value,
                    });
                }
            }
        }
        Err(format!("Cannot cast {:?} to BooleanLiteral", value))
    }
}
impl std::fmt::Display for BooleanLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal.clone())
    }
}

#[cfg(test)]
mod test {
    use {crate::BooleanLiteral, ::token::*};
    use std::rc::Rc;

    #[test]
    fn test_bool_literal() {
        let s = BooleanLiteral {
            token: Rc::new(Token {
                token_type: TRUE,
                literal: Rc::new("true".into()),
            }),
            value: true,
        };
        assert_eq!(format!("{}", s), "true");
        assert_eq!(s.value, true);
    }
}

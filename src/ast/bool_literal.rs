use crate::ast::*;
use crate::token::*;
use std::any::Any;

#[ast_node(Expression)]
pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl TryFrom<Box<&dyn Expression>> for BooleanLiteral {
    type Error = String;

    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        let v_any = value.as_any();
        if v_any.is::<BooleanLiteral>() {
            if let Some(val) = v_any.downcast_ref::<BooleanLiteral>() {
                return Ok(BooleanLiteral {
                    token: val.token.clone(),
                    value: val.value,
                });
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
    use {crate::ast::BooleanLiteral, crate::token::Token, crate::token::TRUE};

    #[test]
    fn test_bool_literal() {
        let s = BooleanLiteral {
            token: Token {
                token_type: TRUE,
                literal: "true".into(),
            },
            value: true,
        };
        assert_eq!(format!("{}", s), "true");
        assert_eq!(s.value, true);
    }
}

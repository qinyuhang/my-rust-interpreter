use crate::*;
use ::token::*;

use std::rc::Rc;

#[ast_node(Statement)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Rc<AstExpression>>,
}

impl TryFrom<Box<&dyn Statement>> for ReturnStatement {
    type Error = String;
    fn try_from(value: Box<&dyn Statement>) -> Result<Self, Self::Error> {
        if let Some(v) = value.as_any().downcast_ref::<Self>() {
            return Ok((*v).clone());
        }
        Err(format!("error cast object {:?}", value))
    }
}

impl TryFrom<Box<&AstExpression>> for ReturnStatement {
    type Error = String;
    fn try_from(value: Box<&AstExpression>) -> Result<Self, Self::Error> {
        if let AstExpression::ReturnStatement(v) = *value {
            return Ok((*v).clone());
        }
        Err(format!("error cast object {:?}", value))
    }
}
impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {};",
            self.token_literal(),
            self.return_value
                .as_ref()
                .map_or("".into(), |v| v.to_string())
        )
    }
}

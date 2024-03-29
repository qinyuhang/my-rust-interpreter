use crate::ast::*;
use crate::token::Token;

use std::rc::Rc;

#[ast_node(Statement)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Option<Rc<dyn Expression>>,
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

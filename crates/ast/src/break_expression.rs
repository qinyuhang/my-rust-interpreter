use crate::*;
use std::any::Any;
use token::Token;

#[ast_node(Expression)]
#[derive(Hash)]
pub struct Break {
    pub token: Token,
}

impl Display for Break {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "break")
    }
}

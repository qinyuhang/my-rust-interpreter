use crate::*;
use std::any::Any;
use token::Token;
use std::rc::Rc;

#[ast_node(Expression)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
pub struct Break {
    pub token: Rc<Token>,
}

impl Display for Break {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "break")
    }
}

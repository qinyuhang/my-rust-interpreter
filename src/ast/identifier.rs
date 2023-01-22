use crate::ast::{*, Node};
use crate::token::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: Rc<RefCell<Token>>,

    // FIXME: 这里现在结果是 IDENT , 我觉得预期应该与 token.literal 一样
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.borrow().literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Expression for Identifier {
    fn expression_node(&self) {
        todo!()
    }
}

impl TryFrom<Box<&dyn Expression>> for Identifier {
    type Error = String;

    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        if let Some(value) = value.as_any().downcast_ref::<ExpressionStatement>() {
            if value.token.borrow().token_type == IDENT {
                return Ok(Identifier { token: Rc::new(RefCell::new((*value.token.borrow()).clone())), value: value.token.borrow().literal.clone() });
            }
            // fixme: how to make a new
            // return Ok(Identifier { token: v.token.clone(), value: v.value.clone() });
        }
        Err(format!("error cast object {:?}", value))
    }
}

impl TryFrom<Box<&ExpressionStatement>> for Identifier {
    type Error = String;

    fn try_from(value: Box<&ExpressionStatement>) -> Result<Self, Self::Error> {
        if value.token.borrow().token_type == IDENT {
            return Ok(Identifier { token: Rc::new(RefCell::new((*value.token.borrow()).clone())), value: value.token.borrow().literal.clone() });
        }
        Err(format!("error cast object {:?}", value))
    }
}
impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
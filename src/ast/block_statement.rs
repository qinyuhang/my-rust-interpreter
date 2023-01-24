use crate::ast::*;
use crate::token::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub token: Rc<RefCell<Token>>,
    pub statement: Rc<RefCell<Vec<Rc<dyn Statement>>>>,
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        todo!()
    }

    fn as_any(&self) -> &dyn Any {
        todo!()
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {
        todo!()
    }
}

impl std::fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.statement
                .borrow()
                .iter()
                .fold("".into(), |acc, x| format!("{}{}", acc, x))
        )
    }
}

impl TryFrom<Box<&dyn Expression>> for BlockStatement {
    type Error = String;

    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        if let Some(value) = value.as_any().downcast_ref::<BlockStatement>() {
            
        }
        Err(format!("error cast object {:?}", value))
    }
}

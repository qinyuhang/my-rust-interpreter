use crate::ast::identifier::Identifier;
use crate::ast::*;
use crate::token::Token;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Box<Identifier>,
    pub value: Box<dyn Expression>,
}
impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Statement for LetStatement {
    fn statement_node(&self) {
        todo!()
    }
}
impl Expression for LetStatement {
    fn expression_node(&self) {
        todo!()
    }
}
impl TryFrom<Box<&dyn Statement>> for LetStatement {
    type Error = String;
    fn try_from(value: Box<&dyn Statement>) -> Result<Self, Self::Error> {
        if let Some(v) = value.as_any().downcast_ref::<Self>() {
            return Ok(LetStatement {
                token: v.token.clone(),
                name: v.name.clone(),
                value: Box::new(ExpressionStatement {
                    token: ExpressionStatement::try_from(Box::new(v.value.as_ref()))
                        .unwrap()
                        .token
                        .clone(),
                    // FIXME: not None
                    expression: None,
                }),
            });
        }
        Err(format!("error cast object {:?}", value))
    }
}
impl TryFrom<Box<&dyn Expression>> for LetStatement {
    type Error = String;
    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        let x = (value).as_any();
        if x.is::<LetStatement>() {
            let x = x.downcast_ref::<LetStatement>();
            if x.is_some() {
                let x = x.unwrap();
                return Ok(LetStatement {
                    token: x.token.clone(),
                    name: x.name.clone(),
                    value: Box::new(LetStatement::try_from(Box::new(x.value.as_ref())).unwrap()),//x.value.clone(),
                });
            } else {
                return Err(format!(""));
            }
        }
        Err(format!(""))
    }
}
impl std::fmt::Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME:
        // write!(f, "{} {} = {};", self.token_literal(), self.name, self.value.unwrap_or(""))
        write!(f, "{} {} = {};", self.token_literal(), self.name, self.value)
    }
}
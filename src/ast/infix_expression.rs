use crate::ast::*;
use crate::token::Token;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct InfixExpression {
    pub token: Rc<RefCell<Token>>,
    pub operator: String,
    pub left: Option<Rc<dyn Expression>>,
    pub right: Option<Rc<dyn Expression>>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.borrow().literal.clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {
        todo!()
    }
}

impl TryFrom<Box<&dyn Expression>> for InfixExpression {
    type Error = String;

    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        let x = value.as_any();
        if x.is::<InfixExpression>() {
            println!("Object is InfixExpression {:?}", value);
            let x = x.downcast_ref::<InfixExpression>().unwrap();
            return Ok(InfixExpression {
                token: x.token.clone(),
                operator: x.operator.clone(),
                right: x.right.clone(),
                left: x.left.clone(),
            });
        }
        Err(format!("Cannot cast {:?} to InfixExpression", value))
    }
}

impl std::fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left.as_ref().unwrap(), self.operator, self.right.as_ref().unwrap())
    }
}

mod test {
    #[test]
    fn test_infix_display() {
        
    }
}
use crate::ast::*;
use crate::token::Token;

use std::rc::Rc;

#[ast_node(Expression)]
pub struct InfixExpression {
    pub token: Token,
    pub operator: String,
    pub left: Option<Rc<dyn Expression>>,
    pub right: Option<Rc<dyn Expression>>,
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
        write!(
            f,
            "({} {} {})",
            self.left.as_ref().unwrap(),
            self.operator,
            self.right.as_ref().unwrap()
        )
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_infix_display() {}
}

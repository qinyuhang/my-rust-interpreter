use crate::*;
use ::token::*;

use std::rc::Rc;

#[ast_node(Expression)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
pub struct UpdateExpression {
    pub token: Token,
    pub operator: String,
    pub name: Option<Rc<Identifier>>,
    pub right: Option<Rc<AstExpression>>,
}

impl TryFrom<Box<&AstExpression>> for UpdateExpression {
    type Error = String;

    fn try_from(value: Box<&AstExpression>) -> Result<Self, Self::Error> {
        return match *value {
            AstExpression::UpdateExpression(value) => {
                let x = value.as_any();
                if x.is::<Self>() {
                    // println!("Object is InfixExpression {:?}", value);
                    let x = x.downcast_ref::<Self>().unwrap();
                    return Ok(x.clone());
                }
                Err(format!("Cannot cast {:?} to UpdateExpression", value))
            }
            AstExpression::ExpressionStatement(value) => {
                if let Some(value) = value.expression.clone() {
                    return Self::try_from(Box::new(&value.as_ref().clone()));
                }
                Err(format!("Cannot cast {:?} to UpdateExpression", value))
            }
            _ => Err(format!("Cannot cast {:?} to UpdateExpression", value)),
        };
    }
}

impl std::fmt::Display for UpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.name.as_ref().unwrap(),
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

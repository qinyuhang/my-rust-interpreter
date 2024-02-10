use crate::*;
use ::token::*;
use std::rc::Rc;

#[ast_node(Statement)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
pub struct BlockStatement {
    pub token: Rc<Token>, // { token
    // FIXME: change Rc RefCell
    pub statement: Vec<Rc<AstExpression>>,
}

impl std::fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.statement
                .iter()
                .fold("".into(), |acc, x| format!("{}{}", acc, x))
        )
    }
}

impl TryFrom<Rc<&dyn Statement>> for BlockStatement {
    type Error = String;

    fn try_from(value: Rc<&dyn Statement>) -> Result<Self, Self::Error> {
        if let Some(val) = value.as_any().downcast_ref::<Self>() {
            return Ok(val.clone());
        }
        Err(format!("error cast object {:?}", value))
    }
}

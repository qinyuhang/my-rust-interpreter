use crate::ast::*;
use crate::token::*;
use std::rc::Rc;

#[ast_node(Expression)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Rc<dyn Expression>,

    // FIXME: BlockStatement
    pub consequence: Option<Rc<dyn Statement /* BlockStatement */>>,
    // FIXME: BlockStatement
    pub alternative: Option<Rc<dyn Statement /* BlockStatement */>>,
}

impl std::fmt::Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "if {} {} {}",
                self.condition,
                self.consequence.as_ref().unwrap(),
                self.alternative.as_ref().map_or_else(
                    || "".into(),
                    |val| format!("else {}", val.as_ref().to_string())
                )
            )
        )
    }
}
impl TryFrom<Box<&dyn Expression>> for IfExpression {
    type Error = String;

    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        if let Some(value) = value.as_any().downcast_ref::<IfExpression>() {
            return Ok(value.clone());
        }
        Err(format!("error cast object {:?}", value))
    }
}

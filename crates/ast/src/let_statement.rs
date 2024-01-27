use crate::*;
use ::token::*;
use std::rc::Rc;

#[ast_node(Statement)]
#[ast_node_with_try_from(Expression)]
#[derive(Hash)]
pub struct LetStatement {
    pub token: Token,
    pub name: Rc<Identifier>,
    //         Option<Rc<dyn Expression>>;
    pub value: Option<Rc<AstExpression>>,
}

impl TryFrom<Box<&dyn Statement>> for LetStatement {
    type Error = String;
    fn try_from(value: Box<&dyn Statement>) -> Result<Self, Self::Error> {
        if let Some(v) = value.as_any().downcast_ref::<Self>() {
            return Ok(LetStatement {
                token: v.token.clone(),
                name: v.name.clone(),
                value: Some(v.value.as_ref().unwrap().clone()),
            });
        }
        Err(format!("error cast object {:?}", value))
    }
}

impl TryFrom<Box<&AstExpression>> for LetStatement {
    type Error = String;
    fn try_from(value: Box<&AstExpression>) -> Result<Self, Self::Error> {
        if let AstExpression::IfExpression(value) = *value {
            let x = (value).as_any();
            if x.is::<LetStatement>() {
                let x = x.downcast_ref::<LetStatement>();
                if x.is_some() {
                    let x = x.unwrap();
                    return Ok(LetStatement {
                        token: x.token.clone(),
                        name: x.name.clone(),
                        value: Some(Rc::new(AstExpression::LetStatement(
                            LetStatement::try_from(Box::new(&*x.value.as_ref().unwrap().clone()))
                                .unwrap(),
                        ))), //x.value.clone(),
                    });
                } else {
                    return Err(format!(""));
                }
            }
        }

        Err(format!(""))
    }
}
impl std::fmt::Display for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // FIXME:
        // write!(f, "{} {} = {};", self.token_literal(), self.name, self.value.unwrap_or(""))
        write!(
            f,
            "{} {} = {};",
            self.token_literal(),
            self.name,
            self.value.as_ref().unwrap()
        )
    }
}

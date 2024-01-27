use crate::*;
use ::token::*;

#[ast_node(Expression)]
#[ast_node_with_try_from(Expression)]
// #[derive(Hash, Eq, PartialEq)]
#[derive(Hash)]
pub struct Identifier {
    pub token: Token,

    pub value: String,
}

impl TryFrom<Box<&ExpressionStatement>> for Identifier {
    type Error = String;

    fn try_from(value: Box<&ExpressionStatement>) -> Result<Self, Self::Error> {
        if value.token.token_type == IDENT {
            return Ok(Identifier {
                token: value.token.clone(),
                value: value.token.literal.clone(),
            });
        }
        Err(format!("error cast object {:?}", value))
    }
}
impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[allow(unused)]
pub fn test_identifier_expression(exp: Box<&dyn Statement>, value: String) -> bool {
    let stm = ExpressionStatement::try_from(exp);

    assert!(stm.is_ok());

    let stm = stm.unwrap();

    let id = Identifier::try_from(Box::new(&stm));

    assert!(id.is_ok());

    let id = id.unwrap();

    assert_eq!(id.value, value);

    assert_eq!(id.token_literal(), value);

    true
}

#[allow(unused)]
pub fn test_identifier_enum(exp: Box<&AstExpression>, value: String) -> bool {
    let stm = ExpressionStatement::try_from(exp);

    assert!(stm.is_ok());

    let stm = stm.unwrap();

    let id = Identifier::try_from(Box::new(&stm));

    assert!(id.is_ok());

    let id = id.unwrap();

    assert_eq!(id.value, value);

    assert_eq!(id.token_literal(), value);

    true
}

use crate::ast::*;
use crate::token::*;
use std::rc::Rc;

#[ast_node(Expression)]
#[derive(Hash)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Rc<AstExpression>,
    pub index: Rc<AstExpression>,
}

impl std::fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}

#[cfg(test)]
mod test {
    use crate::{AstExpression, Identifier, IndexExpression, Token, LPAREN};
    use std::rc::Rc;

    #[test]
    fn test_index_expression_print() {
        let x = IndexExpression {
            token: Token {
                token_type: LPAREN,
                literal: "".into(),
            },
            left: Rc::new(AstExpression::Identifier(Identifier {
                token: Token {
                    token_type: LPAREN,
                    literal: "a".into(),
                },
                value: "a".to_string(),
            })),
            index: Rc::new(AstExpression::Identifier(Identifier {
                token: Token {
                    token_type: LPAREN,
                    literal: "a".into(),
                },
                value: "a".to_string(),
            })),
        };
        dbg!(&x);
        assert_eq!(format!("{}", x), "(a[a])");
    }
}

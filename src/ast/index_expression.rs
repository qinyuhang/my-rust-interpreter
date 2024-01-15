use crate::ast::*;
use crate::token::*;
use std::rc::Rc;

#[ast_node(Expression)]
pub struct IndexExpression {
    pub token: Token,
    pub left: Rc<dyn Expression>,
    pub index: Rc<dyn Expression>,
}

impl std::fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}

#[cfg(test)]
mod test {
    use crate::{Identifier, IndexExpression, Token, LPAREN};
    use std::rc::Rc;

    #[test]
    fn test_index_expression_print() {
        let x = IndexExpression {
            token: Token {
                token_type: LPAREN,
                literal: "".into(),
            },
            left: Rc::new(Identifier {
                token: Token {
                    token_type: LPAREN,
                    literal: "a".into(),
                },
                value: "a".to_string(),
            }),
            index: Rc::new(Identifier {
                token: Token {
                    token_type: LPAREN,
                    literal: "a".into(),
                },
                value: "a".to_string(),
            }),
        };
        dbg!(&x);
        assert_eq!(format!("{}", x), "(a[a])");
    }
}

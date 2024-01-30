use crate::*;
use ::token::*;
use std::rc::Rc;

#[ast_node(Expression)]
#[ast_node_with_try_from(Expression)]
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
    use crate::*;
    use ::token::*;
    use std::rc::Rc;

    #[test]
    fn test_index_expression_print() {
        let x = IndexExpression {
            token: Token {
                token_type: LPAREN,
                literal: Rc::new("".into()),
            },
            left: Rc::new(AstExpression::Identifier(Identifier {
                token: Token {
                    token_type: LPAREN,
                    literal: Rc::new("a".into()),
                },
                value: Rc::new("a".to_string()),
            })),
            index: Rc::new(AstExpression::Identifier(Identifier {
                token: Token {
                    token_type: LPAREN,
                    literal: Rc::new("a".into()),
                },
                value: Rc::new("a".to_string()),
            })),
        };
        dbg!(&x);
        assert_eq!(format!("{}", x), "(a[a])");
    }
}

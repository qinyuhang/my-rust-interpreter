use crate::AstExpression;
use crate::*;
use ast_macro::ast_node;
use std::rc::Rc;
use token::Token;

#[ast_node(Expression)]
#[derive(Hash)]
pub struct WhileLoopLiteral {
    pub token: Token,
    pub condition: Rc<AstExpression>,
    // blockStatement
    pub body: Option<Rc<AstExpression>>,
}

impl std::fmt::Display for WhileLoopLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                r#"while ({}) {{ {} }}"#,
                self.condition,
                self.body
                    .as_ref()
                    .map_or("".to_string(), |v| format!("{v}"))
            )
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use token::*;
    #[test]
    fn test_fmt_while_loop_literal() {
        let exp = WhileLoopLiteral {
            token: Default::default(),
            condition: Rc::new(AstExpression::BooleanLiteral(BooleanLiteral {
                token: Token::from(TRUE),
                value: true,
            })),
            body: Some(Rc::new(AstExpression::BooleanLiteral(BooleanLiteral {
                token: Token::from(TRUE),
                value: true,
            }))),
        };

        dbg!(format!("{exp}"));
    }
}
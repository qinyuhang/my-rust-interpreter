use crate::ast::*;
use crate::token::*;
use std::rc::Rc;

#[ast_node(Expression)]
#[derive(Hash)]
pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Rc<AstExpression>>,
}

impl std::fmt::Display for ArrayLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.elements
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::rc::Rc;

    #[test]
    fn test_array_literal_display() {
        let input = "[1, 2]";
        let i = ArrayLiteral {
            token: Token {
                token_type: LBRACKET,
                literal: "[".into(),
            },
            elements: vec![
                Rc::new(AstExpression::IntegerLiteral(
                    IntegerLiteral::try_from("1".to_string()).unwrap(),
                )),
                Rc::new(AstExpression::IntegerLiteral(
                    IntegerLiteral::try_from("2".to_string()).unwrap(),
                )),
            ],
        };
        // dbg!(&i);
        // println!("{}", &i);

        assert_eq!(format!("{}", i), input);
    }
}

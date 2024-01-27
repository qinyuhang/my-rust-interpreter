use crate::*;
use ::ast::*;
use ast_macro::{object, object_with_try_from};
use std::rc::Rc;

#[object(FUNCTION_OBJECT)]
#[object_with_try_from(FUNCTION_OBJECT)]
pub struct FunctionObject {
    pub context: Rc<Context>,

    // TODO change to RC
    pub parameters: Option<Vec<Rc<Identifier>>>,
    // blockStatement
    pub body: Option<Rc<AstExpression>>,
    // pub body: Option<BlockStatement>,
}

impl ObjectInspect for FunctionObject {
    fn _inspect(&self) -> String {
        format!("fn ({}) {{ {} }}", 1, 2)
    }
}

#[cfg(test)]
mod test {
    use ::lexer::*;
    use ::token::*;
    #[test]
    fn test_function() {
        let input = r#"fn() {}"#;

        let tests = vec![
            (FUNCTION, "fn"),
            (LPAREN, "("),
            (RPAREN, ")"),
            (LBRACE, "{"),
            (RBRACE, "}"),
            (EOF, "\0"),
        ];

        let lex = Lexer::new(input);

        tests.iter().for_each(|test| {
            let p_token = lex.next_token();
            // println!("Running Test: {:?}, lexer.next_token: {:?}", test, p_token);
            assert_eq!(p_token.token_type, test.0);
            assert_eq!(p_token.literal, test.1);
        });
    }
}

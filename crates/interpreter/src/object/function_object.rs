use crate::object::*;
use crate::*;
use ast_macro::object;
use std::rc::Rc;
use lang_parser::*;

#[object(FUNCTION_OBJECT)]
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

impl TryFrom<Rc<dyn Object>> for FunctionObject {
    type Error = String;

    fn try_from(_value: Rc<dyn Object>) -> Result<Self, Self::Error> {
        // let val = value.as_any();
        // if val.is::<Integer>() {
        //     if let Some(v) = val.downcast_ref::<Integer>() {
        //         return Ok((*v).clone());
        //     }
        // }
        Err("Str".into())
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use lang_parser::*;
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

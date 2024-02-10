#[cfg(test)]
mod test {
    use ::lexer::*;
    use ::token::*;
    use {crate::*, std::rc::Rc};

    #[test]
    fn test_program() {
        let input = r#"let a = 10;"#;
        #[allow(unused)]
        let lex = Lexer::new(input);

        // let p = Program { statement: vec![] };
        // println!("{:?}", p.token_literal());
    }

    #[test]
    fn test_let_statement() {
        let input = r#"let a = 10;"#;
        #[allow(unused)]
        let lex = Lexer::new(input);
        // let s = LetStatement {};
        // println!("{}", s.token_literal());
        // assert_eq!(s.token_literal(), "");
    }

    #[test]
    fn test_prefix_expression() {
        let pe = PrefixExpression {
            token: Rc::new(Token {
                token_type: INT,
                literal: Rc::new("1".into()),
            }),
            operator: Rc::new("-".into()),
            right: Some(Rc::new(AstExpression::IntegerLiteral(IntegerLiteral {
                token: Rc::new(Token {
                    token_type: INT,
                    literal: Rc::new("1".into()),
                }),
                value: 1,
            }))),
        };
        assert_eq!(format!("{}", pe), "(-1)");
    }

    #[test]
    fn test_new_int_literal() {
        #[allow(unused)]
        let i = IntegerLiteral {
            value: 5,
            token: Rc::new(Token {
                token_type: INT,
                literal: Rc::new("5".to_string()),
            }),
        };
    }

    #[test]
    fn test_from_str() {
        let input = "5";
        let i = IntegerLiteral::try_from(input.to_string());
        assert!(i.is_ok());
        let i = i.unwrap();
        assert_eq!(*i.token_literal(), input.to_string());
        assert_eq!(5, i.value);
    }
}

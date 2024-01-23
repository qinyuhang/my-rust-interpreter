#[cfg(test)]
mod test {
    use ::token::*;
    use ast::{AstExpression, *};
    use lexer::Lexer;
    use {crate::*, std::rc::Rc};

    #[test]
    fn test_parser() {
        let input = r#"let x = 5;
let y = 10;
let foobar = 838383;
"#;
        let lex = Lexer::new(input);
        let p = Parser::new(lex);
        // println!("{p}");
        let pr = p.parse_program();
        // println!("{p}");
        test_parser_errors(&p, None);

        assert!(pr.is_some());

        let pr = pr.unwrap();
        assert_eq!(pr.statement.len(), input.lines().count());

        // for st in pr.statement.iter() {
        //     println!("Statements: {:?}", st);
        // }

        let tests = vec![("x"), ("y"), ("foobar")];

        for (i, &v) in tests.iter().enumerate() {
            let stmt = pr.statement[i].clone();
            if let AstExpression::LetStatement(l) = &*stmt {
                test_let_statement(l, v.into());
            } else {
                assert!(false);
            }
            // test_let_statement(&*stmt, v.into());
        }
    }

    #[allow(unused)]
    fn test_let_statement(s: &dyn Statement, name: String) -> bool {
        assert_eq!(s.token_literal(), "let".to_string());
        // assert_eq!()

        let s = LetStatement::try_from(Box::new(s));

        assert!(s.is_ok());
        let s = s.unwrap();
        assert_eq!(s.name.value, name);
        //
        // assert_eq!(s.statement_node());
        assert_eq!(s.name.token_literal(), name);
        true
    }

    #[allow(unused)]
    fn test_parser_errors(p: &Parser, expect_error_count: Option<usize>) {
        let errors = p.errors();
        let err_count = errors.borrow().len();
        let expect_error_count = expect_error_count.unwrap_or(0);
        if err_count != expect_error_count {
            errors.borrow().iter().for_each(|e| {
                println!("Get parse error: {}", e);
            })
        }
        assert_eq!(err_count, expect_error_count);
        if err_count == expect_error_count {
            return;
        }
        println!("lang_parser has {} errors", err_count);
    }

    #[test]
    fn test_parser_return_statement() {
        let input = r#"return 5;
return 50;
return 500;
return;
"#;
        let lex = Lexer::new(input);
        let p = Parser::new(lex);
        let pr = p.parse_program();
        test_parser_errors(&p, Some(1));

        assert!(pr.is_some());
        let pr = pr.unwrap();

        assert_eq!(pr.statement.len(), input.lines().count());

        for st in pr.statement.iter() {
            println!("Statements: {:?}", st);

            let s = ReturnStatement::try_from(Box::new(st.as_ref()));
            assert!(s.is_ok());

            let s = s.unwrap();
            assert_eq!(s.token_literal(), "return".to_string());
        }
    }

    #[test]
    fn test_register_parse_fns() {
        let input = r#"return 5;
return 50;
return 500;
"#;
        let lex = Lexer::new(input);
        let p = Parser::new(lex);
        fn the_fn() -> Option<Rc<AstExpression>> {
            Some(Rc::new(AstExpression::ExpressionStatement(
                ExpressionStatement {
                    token: Token {
                        literal: EOF.into(),
                        token_type: EOF,
                    },
                    expression: None,
                },
            )))
        }
        fn the_fn1() -> Option<Rc<AstExpression>> {
            Some(Rc::new(AstExpression::ExpressionStatement(
                ExpressionStatement {
                    token: Token {
                        literal: EOF.into(),
                        token_type: EOF,
                    },
                    expression: None,
                },
            )))
        }
        p.register_prefix(EOF, Rc::new(the_fn));
        p.register_prefix(EOF, Rc::new(the_fn1));
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lex = Lexer::new(input);
        let p = Parser::new(lex);
        // println!("{p}");
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());

        let pr = pr.unwrap();

        assert_eq!(pr.statement.len(), input.lines().count());

        test_identifier_enum(Box::new(&*pr.statement[0].clone()), "foobar".into());
    }

    #[test]
    fn test_int_expression() {
        let input = "5;";
        let lex = Lexer::new(input);
        let p = Parser::new(lex);
        // println!("{p}");
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());

        let pr = pr.unwrap();

        assert_eq!(pr.statement.len(), input.lines().count());

        // println!("{:?}", pr.statement);

        let stm = ExpressionStatement::try_from(Box::new(&*pr.statement[0].clone()));

        assert!(stm.is_ok());

        let stm = stm.unwrap();

        // println!("\n\nwtf: {:?}\n\n", stm);

        let il = IntegerLiteral::try_from(Box::new(&*stm.expression.unwrap()));

        assert!(il.is_ok());

        let il = il.unwrap();

        assert_eq!(il.value, 5);
        assert_eq!(il.token_literal(), "5");
        assert_eq!(il.token.literal, "5");
        assert_eq!(il.token.token_type, INT);
    }

    #[test]
    fn test_int_statement() {
        let input = "let a = 5;";
        let lex = Lexer::new(input);
        let p = Parser::new(lex);
        // println!("{p}");
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());

        let pr = pr.unwrap();

        assert_eq!(pr.statement.len(), input.lines().count());

        println!("{:?}", pr);
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let prefix_tests = vec![("!5", "!", 5i64), ("-15", "-", 15i64)];

        prefix_tests
            .iter()
            .for_each(|&(input, operator, integer_value)| {
                let l = Lexer::new(input);
                let p = Parser::new(l);
                println!("Start Test: {} {} {}", input, operator, integer_value);
                let pr = p.parse_program();
                test_parser_errors(&p, None);

                assert!(pr.is_some());
                let pr = pr.unwrap();

                assert_eq!(pr.statement.len(), input.lines().count());
                println!("Start Test: {} {} {}", input, operator, integer_value);
                let stm = ExpressionStatement::try_from(Box::new(&*pr.statement[0].clone()));

                assert!(stm.is_ok());

                let stm = stm.unwrap();

                // println!("\n\nwtf: {:?}\n\n", stm);

                let il = PrefixExpression::try_from(Box::new(&*stm.expression.unwrap()));

                assert!(il.is_ok());

                let il = il.unwrap();
                println!("{:?}", il);
                assert_eq!(il.operator, operator);
                assert_eq!(
                    IntegerLiteral::try_from(Box::new(&*il.right.unwrap()))
                        .unwrap()
                        .value,
                    integer_value
                );
            });
    }

    #[test]
    fn test_parsing_infix_expression() {
        let tests = vec![
            ("5 + 5", 5, "+", 5),
            ("5 - 5", 5, "-", 5),
            ("5 * 5", 5, "*", 5),
            ("5 / 5", 5, "/", 5),
            ("5 > 5", 5, ">", 5),
            ("5 < 5", 5, "<", 5),
            ("5 == 5", 5, "==", 5),
            ("5 != 5", 5, "!=", 5),
            ("5 ^ 5", 5, "^", 5),
            ("5 | 5", 5, "|", 5),
            ("5 ^^ 5", 5, "^^", 5),
            ("5 && 5", 5, "&&", 5),
            ("5 || 5", 5, "||", 5),
        ];

        tests.iter().for_each(|&(input, le, operator, re)| {
            let l = Lexer::new(input);
            let p = Parser::new(l);
            println!("Start Test: {} {}", input, operator);
            let pr = p.parse_program();
            test_parser_errors(&p, None);

            assert!(pr.is_some());
            let pr = pr.unwrap();

            // println!(
            //     "\n\n\ntest_parsing_infix_expression: {:?}\n\n\n",
            //     pr.statement
            // );

            assert_eq!(pr.statement.len(), input.lines().count());

            let stm = ExpressionStatement::try_from(Box::new(&*pr.statement[0].clone()));

            assert!(stm.is_ok());

            let stm = stm.unwrap();

            // println!("\n\nwtf: {:?}\n\n", stm);

            let il = InfixExpression::try_from(Box::new(&*stm.expression.unwrap()));

            assert!(il.is_ok());

            let il = il.unwrap();
            // println!("{:?}", il);
            assert_eq!(il.operator, operator);
            assert_eq!(
                IntegerLiteral::try_from(Box::new(&*il.left.unwrap()))
                    .unwrap()
                    .value,
                le
            );
            assert_eq!(
                IntegerLiteral::try_from(Box::new(&*il.right.unwrap()))
                    .unwrap()
                    .value,
                re
            );
        });

        let tests = vec![
            ("true == true", true, "==", true),
            ("true != false", true, "!=", false),
            ("false == false", false, "==", false),
        ];

        tests.iter().for_each(|&(input, le, operator, re)| {
            let l = Lexer::new(input);
            let p = Parser::new(l);
            // println!("Start Test: {} {}", input, operator);
            let pr = p.parse_program();
            test_parser_errors(&p, None);

            assert!(pr.is_some());
            let pr = pr.unwrap();

            // println!(
            //     "\n\n\ntest_parsing_infix_expression: {:?}\n\n\n",
            //     pr.statement
            // );

            assert_eq!(pr.statement.len(), input.lines().count());

            let stm = ExpressionStatement::try_from(Box::new(&*pr.statement[0].clone()));

            assert!(stm.is_ok());

            let stm = stm.unwrap();

            // println!("\n\nwtf: {:?}\n\n", stm);

            let il = InfixExpression::try_from(Box::new(&*stm.expression.unwrap()));

            assert!(il.is_ok());

            let il = il.unwrap();
            // println!("{:?}", il);
            assert_eq!(il.operator, operator);
            assert_eq!(
                BooleanLiteral::try_from(Box::new(&*il.left.unwrap()))
                    .unwrap()
                    .value,
                le
            );
            assert_eq!(
                BooleanLiteral::try_from(Box::new(&*il.right.unwrap()))
                    .unwrap()
                    .value,
                re
            );
        });
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("true", "true"),
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
            ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
            (
                "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
                "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
            ),
            (
                "add(a + b + c * d / f + g)",
                "add((((a + b) + ((c * d) / f)) + g))",
            ),
            (
                "a * [1, 2, 3][b * c] * d",
                "((a * ([1, 2, 3][(b * c)])) * d)",
            ),
            (
                "add(a * b[2], b[1], 2 * [1, 2][1])",
                "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
            ),
        ];

        #[allow(unused)]
        tests.iter().for_each(|(input, expected)| {
            let l = Lexer::new(*input);
            let p = Parser::new(l);
            let pr = p.parse_program();
            test_parser_errors(&p, None);

            assert!(pr.is_some());
            let pr = pr.unwrap();

            println!(
                "\n\ntest_operator_precedence_parsing\nDebug: {:?}\nToString\n{}\n\n",
                pr, pr
            );
            println!("\n\ntest_operator_precedence_parsing\nToString\n{}\n\n", pr);

            assert_eq!(format!("{}", &pr), *expected);

            // FIXME: 这里无法完成，
            println!("\n\ntest_operator_precedence_parsing: {}", pr.statement[0]);

            println!("\n\nntest_operator_precedence_parsing pr: {}", pr);
        });
    }

    #[test]
    fn test_parse_hash_literal() {
        let input = r#"{"one": 1, "two": 2, "three": 3 }"#;
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());
        let pr = pr.unwrap();
        assert_eq!(
            pr.statement[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .clone()
                .unwrap()
                .as_any()
                .downcast_ref::<HashLiteral>()
                .unwrap()
                .pairs
                .borrow()
                .len(),
            3
        );

        // get 0 of pr
        // check is hashObject
        // check hashObject.pair.len() == 3
        // check every key and value is match the key&value
    }

    #[test]
    fn test_empty_hash_literal() {
        let input = r#"{}"#;
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());
        let pr = pr.unwrap();

        assert_eq!(
            pr.statement[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .clone()
                .unwrap()
                .as_any()
                .downcast_ref::<HashLiteral>()
                .unwrap()
                .pairs
                .borrow()
                .len(),
            0
        );
    }

    #[test]
    fn test_hash_literal_with_expression() {
        let input = r#"{"one": 0 + 1, "two": 10 - 8, "three": 15 / 5 }"#;
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());
        let pr = pr.unwrap();

        // get 0 of pr
        // check is hashObject
        // check hashObject.pair.len() == 3
        // check every key and value is match the key&value
        assert_eq!(
            pr.statement[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .clone()
                .unwrap()
                .as_any()
                .downcast_ref::<HashLiteral>()
                .unwrap()
                .pairs
                .borrow()
                .len(),
            3
        );

        dbg!(
            &pr.statement[0]
                .as_any()
                .downcast_ref::<ExpressionStatement>()
                .unwrap()
                .expression
                .clone()
                .unwrap()
                .as_any()
                .downcast_ref::<HashLiteral>()
                .unwrap()
                .pairs
        );
    }
    #[test]
    fn test_mixed_to_string() {
        let input = r#"let x = 1;
let y = -10;
-a;
!a;
return 1;
return;
5 + 5;
"#;
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        assert!(pr.is_some());
        let pr = pr.unwrap();
        assert_eq!(pr.statement.len(), input.lines().count());

        println!(
            "\n\ntest_mixed_to_string {} {}\n\n",
            &pr,
            &pr.statement.len()
        );
        pr.statement.iter().for_each(|st| {
            println!("st: {:?}", st);
        });
    }

    #[test]
    fn test_if_expression() {
        let input = r#"if (x < y) { x }"#;
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());
        let pr = pr.unwrap();

        assert_eq!(pr.statement.len(), input.lines().count());

        let stm = ExpressionStatement::try_from(Box::new(&*pr.statement[0].clone()));

        assert!(stm.is_ok());

        let stm = stm.unwrap();

        println!("\n\nwtf: {:?}\n\n", stm);

        let il = IfExpression::try_from(Box::new(&*stm.expression.unwrap()));

        assert!(il.is_ok());
        #[allow(unused_variables)]
        let il = il.unwrap();
    }

    #[test]
    fn test_if_else_expression() {
        let input = r#"if (x < y) { x } else { y }"#;
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());
        let pr = pr.unwrap();

        assert_eq!(pr.statement.len(), input.lines().count());

        let stm = ExpressionStatement::try_from(Box::new(&*pr.statement[0].clone()));

        assert!(stm.is_ok());

        let stm = stm.unwrap();

        // println!("\n\nwtf: {:?}\n\n", stm);

        let il = IfExpression::try_from(Box::new(&*stm.expression.unwrap()));

        assert!(il.is_ok());
        #[allow(unused_variables)]
        let il = il.unwrap();
    }

    #[test]
    fn test_function_literal() {
        let input = r#"fn (x, y) { return x + y; };
fn() { return x + y; };
fn() { return fn(x, y) { return x > y; } };
let mf = fn(x, y) { return x + y; };"#;
        // mf(x, y, fn(x, y) { return x > y });"#;
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        test_parser_errors(&p, None);

        assert!(pr.is_some());
        let pr = pr.unwrap();

        // assert_eq!(pr.statement.len(), input.lines().count());
        println!("pr: {:?}", pr);
        println!("pr: {}", pr);
    }

    #[test]
    fn test_call_expression() {
        let input = r#"add(1, 2);"#;

        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        // test_parser_errors(&p, None);

        assert!(pr.is_some());
        let pr = pr.unwrap();
        println!("test_call_expression: pr: {:?}", pr);
    }

    #[test]
    fn test_hex_binary_string() {
        let input = "0x01; 0b10";
        let l = Lexer::new(input);

        let p = Parser::new(l);

        let pr = p.parse_program();

        assert!(pr.is_some());

        let pr = pr.unwrap();
        println!("test_hex_binary_string: pr: {:?}", pr);
    }

    #[test]
    fn test_string_literal() {
        let input = r#""foobar""#;
        let l = Lexer::new(input);

        let p = Parser::new(l);

        let pr = p.parse_program();

        assert!(pr.is_some());

        let pr = pr.unwrap();
        println!("test_string_literal: pr: {:?}", pr);

        // if let Program { ref statement } = pr {
        //     dbg!(statement[0].as_ref().expression);
        //     let x = statement[0].as_ref().as_any();
        //     dbg!(&x);
        //     assert!(x.is::<StringLiteral>());
        // }
    }

    #[test]
    fn test_array_literal() {
        let input = r#"[1,2,3]"#;
        let l = Lexer::new(input);

        let p = Parser::new(l);

        let pr = p.parse_program();

        assert!(pr.is_some());

        let pr = pr.unwrap();
        println!("test_array_literal: pr: {:?}", pr);

        let input = r#"[1,2,3,]"#;
        let l = Lexer::new(input);

        let p = Parser::new(l);

        let pr = p.parse_program();

        assert!(pr.is_some());

        let pr = pr.unwrap();
        println!("test_array_literal: pr: {:?}", pr);
    }

    #[test]
    fn test_index_literal() {
        let cases = vec![("[1, 2, 3][0]")];
        cases.iter().for_each(|&input| {
            let l = Lexer::new(input);

            let p = Parser::new(l);

            let pr = p.parse_program();

            assert!(pr.is_some());
        });
    }
}

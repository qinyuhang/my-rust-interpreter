mod test {
    #[allow(unused)]
    use {crate::evaluator::*, crate::lexer::*, crate::object::*, crate::parser::*};

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];

        tests.iter().for_each(|&(input, expected)| {
            let evaluated = test_eval(input);
            assert!(test_integer_object(evaluated, expected));
        });
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 < 1", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
        ];

        tests.iter().for_each(|&(input, expected)| {
            let evaluated = test_eval(input);
            assert!(test_boolean_object(evaluated, expected));
        });
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
            // ("!null", true),
        ];
        tests.iter().for_each(|&(input, expected)| {
            let evaluated = test_eval(input);
            assert!(test_boolean_object(evaluated, expected));
        });
    }

    #[test]
    fn test_if_else_expressions() {
        let tests = vec![
            ("if (true) { 10 }", Some(10)),
            ("if (false) { 10 }", None),
            ("if (1) { 10 }", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 }", None),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];

        tests.iter().for_each(|&(input, value)| {
            let evaluated = test_eval(input);
            assert!(evaluated.is_some());

            if let Some(int_val) = value {
                test_integer_object(evaluated, int_val);
            } else {
                test_null_object(&evaluated);
            }
        })
    }

    #[test]
    fn test_hex_binary_string() {
        let tests = vec![
            ("0x01", 1),
            ("0xf", 15),
            ("0b1", 1),
            ("0x1_000", 0x1_000),
            ("0x1_000_000", 0x1_000_000),
            ("0x1_000_", 0x1_000_),
        ];

        tests.iter().for_each(|&(input, value)| {
            let evaluated = test_eval(input);
            assert!(evaluated.is_some());

            assert_eq!(Integer::try_from(evaluated.unwrap()).unwrap().value, value);

            // println!("{}", evaluated.unwrap());
        });
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", 10),
            ("return 10; 9;", 10),
            ("return 5 * 2; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                r#"if (10 > 1) { 
                       if (10 > 1) {  return 10;  }
                       return 1; 
                   }"#,
                10,
            ),
        ];

        tests.iter().for_each(|&(input, expected)| {
            let evaluated = test_eval(input);
            assert!(evaluated.is_some());
            test_integer_object(evaluated, expected);
        });
    }

    #[test]
    fn test_function_declaration() {
        let tests = vec![
            ("let b = 5; let a = fn() { b }; a();", 5),
            ("let b = 5; let a = fn() { b }; let b = 10; a();", 10),
            ("fn () { 1; }();", 1),
            // ("fn a() {}", None::<Rc<dyn Object>>),
            // ("fn a(i) {}", None::<Rc<dyn Object>>),
            // ("fn a(x, y) {}", None::<Rc<dyn Object>>),
            ("let add = fn (x, y) { x + y; }; add (1, 1);", 2),
            (
                "let add = fn (x, y) { x + y; }; add(1, 1);",
                2,
            ),
            (
                "let a = fn a(x, y) { return x + y; }; a(1,1);",
                2,
            ),
            // (
            //     "let a = fn a(x, y) { return x + y; }; return a(1,1);",
            //     2,
            // ),
            ("fn (x) { x; }(5)", 5),
            (r#"
            let add = fn(a, b) { a + b; };
            let sub = fn(a, b) { a - b; };
            let applyFunc = fn(a, b, func) { func(a, b) };
            applyFunc(2, 2, add);
            "#, 4),
        ];
        tests.iter().for_each(|&(input, expected)| {
            let parsed = test_parse(input);
            assert!(parsed.is_some());
            let evaluated = test_eval(input);
            test_integer_object(evaluated, expected);
            // assert_eq!(evaluated, expected);
            // if expected.is_none() {
            //     assert!(evaluated.is_none());
            // } else {
            //     assert!(evaluated.is_some());
            // }
        });
    }

    #[test]
    fn test_function_eval() {
        let tests = vec![
            // ("fn () {}", None::<Rc<dyn Object>>),
            // ("fn a() {}; a();", None::<Rc<dyn Object>>),
            // ("fn a(i) {}; a();", None::<Rc<dyn Object>>),
            // ("fn a(x, y) {}; a();", None::<Rc<dyn Object>>),
            ("fn a(x, y) { return x + y; }; a(1, 2);", 3),
            (
                "let a = fn a(x, y) { return x + y; }; a(1, 2);",
                3,
            ),
        ];
        tests.iter().for_each(|&(input, expected)| {
            let parsed = test_parse(input);
            assert!(parsed.is_some());
            let evaluated = test_eval(input);
            dbg!(&evaluated);
            test_integer_object(evaluated, expected);
            // assert_eq!(evaluated, expected);
            // if expected.is_none() {
            //     assert!(evaluated.is_none());
            // }
        });
    }

    #[test]
    fn test_error_object() {
        let test_cases = vec![
            ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
            ("5 + true; 5", "type mismatch: INTEGER + BOOLEAN"),
            ("-true", "unknown operator: -BOOLEAN"),
            ("true + false", "unknown operator: BOOLEAN + BOOLEAN"),
            ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            (
                "if (10 > 1) { true + false; }; return 1;",
                "unknown operator: BOOLEAN + BOOLEAN",
            ),
            ("foobar", "identifier not found: foobar"),
        ];
        test_cases.iter().for_each(|&(case, out)| {
            let evaluated = test_eval(case);
            // assert_eq!(format!("{}", evaluated), out);
            assert!(evaluated.is_some());
            let err = ErrorObject::try_from(evaluated.clone().unwrap());
            assert!(err.is_ok());
            // println!("{:?}", err.unwrap());
            assert_eq!(err.unwrap().message, out);
        });
    }

    #[test]
    fn test_let_state() {
        let test_cases = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];
        test_cases.iter().for_each(|&(case, expected)| {
            let evaluated = test_eval(case);
            // assert_eq!(format!("{}", evaluated), out);
            assert!(evaluated.is_some());
            assert!(test_integer_object(evaluated, expected));
        });
    }

    #[allow(unused)]
    fn test_null_object(obj: &Option<Rc<dyn Object>>) {
        assert!(obj.is_some());
        println!("test null object: {}", obj.as_ref().unwrap());
        let x = obj.as_ref().unwrap().as_any();
        assert!(x.downcast_ref::<Null>().is_some());
    }

    #[allow(unused)]
    fn test_parse(input: &str) -> Option<Program> {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        return pr;
    }

    #[allow(unused)]
    fn test_eval(input: &str) -> Option<Rc<dyn Object>> {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        assert!(pr.is_some());
        let pr = pr.unwrap();
        let context = Context::new();
        return eval(&pr, Rc::new(context));
    }

    #[allow(unused)]
    fn test_integer_object(obj: Option<Rc<dyn Object>>, expected: i64) -> bool {
        println!("test_integer_object {:?}", obj);
        let i = Integer::try_from(obj.unwrap());
        assert!(i.is_ok());
        let i = i.unwrap();
        assert_eq!(i.value, expected);
        true
    }

    #[allow(unused)]
    fn test_boolean_object(obj: Option<Rc<dyn Object>>, expected: bool) -> bool {
        let i = Boolean::try_from(obj.unwrap());
        assert!(i.is_ok());
        let i = i.unwrap();
        assert_eq!(i.value, expected);
        true
    }
}

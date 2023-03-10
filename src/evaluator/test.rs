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

    #[allow(unused)]
    fn test_null_object(obj: &Option<Rc<dyn Object>>) {
        assert!(obj.is_some());
        println!("test null object: {}", obj.as_ref().unwrap());
        let x = obj.as_ref().unwrap().as_any();
        assert!(x.downcast_ref::<Null>().is_some());
    }

    #[allow(unused)]
    fn test_eval(input: &str) -> Option<Rc<dyn Object>> {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        assert!(pr.is_some());
        let pr = pr.unwrap();
        return eval(&pr);
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

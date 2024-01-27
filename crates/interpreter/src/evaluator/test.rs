#[cfg(test)]
mod test {
    use crate::*;
    use ::ast::*;
    use ::testing::{testing_result, TestingResult};
    use std::collections::HashMap;
    use testing_object::*;

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", testing_result!(Int, 5)),
            ("10", testing_result!(Int, 10)),
            ("-5", testing_result!(Int, -5)),
            ("-10", testing_result!(Int, -10)),
            ("5 + 5 + 5 + 5 - 10", testing_result!(Int, 10)),
            ("2 * 2 * 2 * 2 * 2", testing_result!(Int, 32)),
            ("-50 + 100 + -50", testing_result!(Int, 0)),
            ("5 * 2 + 10", testing_result!(Int, 20)),
            ("5 + 2 * 10", testing_result!(Int, 25)),
            ("20 + 2 * -10", testing_result!(Int, 0)),
            ("50 / 2 * 2 + 10", testing_result!(Int, 60)),
            ("2 * (5 + 10)", testing_result!(Int, 30)),
            ("3 * 3 * 3 + 10", testing_result!(Int, 37)),
            ("3 * (3 * 3) + 10", testing_result!(Int, 37)),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", testing_result!(Int, 50)),
        ];

        tests.iter().for_each(|(input, expected)| {
            handle_test_object(input, expected);
        });
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![
            ("true", testing_result!(Bool, true)),
            ("false", testing_result!(Bool, false)),
            ("1 < 2", testing_result!(Bool, true)),
            ("1 > 2", testing_result!(Bool, false)),
            ("1 < 1", testing_result!(Bool, false)),
            ("1 > 1", testing_result!(Bool, false)),
            ("1 == 1", testing_result!(Bool, true)),
            ("1 != 1", testing_result!(Bool, false)),
            ("1 == 2", testing_result!(Bool, false)),
            ("1 != 2", testing_result!(Bool, true)),
            ("true == true", testing_result!(Bool, true)),
            ("false == false", testing_result!(Bool, true)),
            ("true == false", testing_result!(Bool, false)),
            ("true != false", testing_result!(Bool, true)),
            ("false != true", testing_result!(Bool, true)),
            ("(1 < 2) == true", testing_result!(Bool, true)),
            ("(1 < 2) == false", testing_result!(Bool, false)),
            ("(1 > 2) == true", testing_result!(Bool, false)),
            ("(1 > 2) == false", testing_result!(Bool, true)),
        ];

        tests.iter().for_each(|(input, expected)| {
            handle_test_object(input, expected);
        });
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", testing_result!(Bool, false)),
            ("!false", testing_result!(Bool, true)),
            ("!5", testing_result!(Bool, false)),
            ("!!true", testing_result!(Bool, true)),
            ("!!false", testing_result!(Bool, false)),
            ("!!5", testing_result!(Bool, true)),
            // ("!null", f!(Bool, true)),
        ];
        tests.iter().for_each(|(input, expected)| {
            handle_test_object(input, expected);
        });
    }

    #[test]
    fn test_if_else_expressions() {
        let tests = vec![
            ("if (true) { 10 }", testing_result!(Int, 10)),
            ("if (false) { 10 }", testing_result!(Nil)),
            ("if (1) { 10 }", testing_result!(Int, 10)),
            ("if (1 < 2) { 10 }", testing_result!(Int, 10)),
            ("if (1 > 2) { 10 }", testing_result!(Nil)),
            ("if (1 > 2) { 10 } else { 20 }", testing_result!(Int, 20)),
            ("if (1 < 2) { 10 } else { 20 }", testing_result!(Int, 10)),
            ("if (1 > 2) { 10 }; 20", testing_result!(Int, 20)),
        ];

        tests.iter().for_each(|(input, value)| {
            handle_test_object(input, value);
        })
    }

    #[test]
    fn test_hex_binary_string() {
        let tests = vec![
            ("0x01", testing_result!(Int, 1)),
            ("0xf", testing_result!(Int, 15)),
            ("0b1", testing_result!(Int, 1)),
            ("0x1_000", testing_result!(Int, 0x1_000)),
            ("0x1_000_000", testing_result!(Int, 0x1_000_000)),
            ("0x1_000_", testing_result!(Int, 0x1_000_)),
        ];

        tests.iter().for_each(|(input, value)| {
            handle_test_object(input, value);
        });
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", testing_result!(Int, 10)),
            ("return 10; 9;", testing_result!(Int, 10)),
            ("return 5 * 2; 9;", testing_result!(Int, 10)),
            ("9; return 2 * 5; 9;", testing_result!(Int, 10)),
            (
                r#"if (10 > 1) {
                       if (10 > 1) {  return 10;  }
                       return 1;
                   }"#,
                testing_result!(Int, 10),
            ),
        ];

        tests.iter().for_each(|(input, expected)| {
            handle_test_object(input, expected);
        });
    }

    #[test]
    fn test_function_declaration() {
        let tests = vec![
            (
                "let b = 5; let a = fn() { b }; a();",
                testing_result!(Int, 5),
            ),
            (
                "let b = 5; let a = fn() { b }; let b = 10; a();",
                testing_result!(Int, 10),
            ),
            ("fn () { 1; }();", testing_result!(Int, 1)),
            // ("fn a() {}", None::<Rc<dyn Object>>),
            // ("fn a(i) {}", None::<Rc<dyn Object>>),
            // ("fn a(x, y) {}", None::<Rc<dyn Object>>),
            (
                "let add = fn (x, y) { x + y; }; add (1, 1);",
                testing_result!(Int, 2),
            ),
            (
                "let add = fn (x, y) { x + y; }; add(1, 1);",
                testing_result!(Int, 2),
            ),
            (
                "let a = fn a(x, y) { return x + y; }; a(1,1);",
                testing_result!(Int, 2),
            ),
            // (
            //     "let a = fn a(x, y) { return x + y; }; return a(1,1);",
            //     2,
            // ),
            ("fn (x) { x; }(5)", testing_result!(Int, 5)),
            (
                r#"
            let add = fn(a, b) { a + b; };
            let sub = fn(a, b) { a - b; };
            let applyFunc = fn(a, b, func) { func(a, b) };
            applyFunc(2, 2, add);
            "#,
                testing_result!(Int, 4),
            ),
        ];
        tests.iter().for_each(|(input, expected)| {
            handle_test_object(input, expected);
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
            (
                "fn a(x, y) { return x + y; }; a(1, 2);",
                testing_result!(Int, 3),
            ),
            (
                "let a = fn a(x, y) { return x + y; }; a(1, 2);",
                testing_result!(Int, 3),
            ),
        ];
        tests.iter().for_each(|(input, expected)| {
            handle_test_object(input, expected);
        });
    }

    #[test]
    fn test_error_object_eval() {
        let test_cases = vec![
            (
                "5 + true;",
                testing_result!(Err, "type mismatch: INTEGER + BOOLEAN"),
            ),
            (
                "5 + true; 5",
                testing_result!(Err, "type mismatch: INTEGER + BOOLEAN"),
            ),
            ("-true", testing_result!(Err, "unknown operator: -BOOLEAN")),
            (
                "true + false",
                testing_result!(Err, "unknown operator: BOOLEAN + BOOLEAN"),
            ),
            (
                "5; true + false; 5",
                testing_result!(Err, "unknown operator: BOOLEAN + BOOLEAN"),
            ),
            (
                "if (10 > 1) { true + false; }",
                testing_result!(Err, "unknown operator: BOOLEAN + BOOLEAN"),
            ),
            (
                "if (10 > 1) { true + false; }; return 1;",
                testing_result!(Err, "unknown operator: BOOLEAN + BOOLEAN"),
            ),
            (
                "foobar",
                testing_result!(Err, "identifier not found: foobar"),
            ),
        ];
        test_cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_let_state() {
        let test_cases = vec![
            ("let a = 5; a;", testing_result!(Int, 5)),
            ("let a = 5 * 5; a;", testing_result!(Int, 25)),
            ("let a = 5; let b = a; b;", testing_result!(Int, 5)),
            (
                "let a = 5; let b = a; let c = a + b + 5; c;",
                testing_result!(Int, 15),
            ),
        ];
        test_cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_string_literal() {
        let input = r#""hello world""#;
        let evaluated = test_eval(input);
        assert!(evaluated.is_some());
        let evaluated = evaluated.unwrap();
        let x = evaluated.as_any();
        assert!(x.is::<StringObject>());
    }

    #[test]
    fn test_string_opr() {
        let input = r#""hello" + "world""#;
        let evaluated = test_eval(input);
        assert!(evaluated.is_some());
        let evaluated = evaluated.unwrap();
        let x = evaluated.as_any();
        assert!(x.is::<StringObject>());

        let input = r#""hello" == "world""#;
        let evaluated = test_eval(input);
        assert!(evaluated.is_some());
        let evaluated = evaluated.unwrap();
        let x = evaluated.as_any();
        assert!(x.is::<Boolean>());
        assert!(!x.downcast_ref::<Boolean>().unwrap().value);

        let input = r#""hello" != "world""#;
        let evaluated = test_eval(input);
        assert!(evaluated.is_some());
        let evaluated = evaluated.unwrap();
        let x = evaluated.as_any();
        assert!(x.is::<Boolean>());
        assert!(x.downcast_ref::<Boolean>().unwrap().value);

        let input = r#""hello" - "world""#;
        let evaluated = test_eval(input);
        assert!(evaluated.is_some());
        let evaluated = evaluated.unwrap();
        let x = evaluated.as_any();
        assert!(x.is::<ErrorObject>());
        assert_eq!(
            x.downcast_ref::<ErrorObject>().unwrap().message,
            "unknown operator: STRING_OBJECT - STRING_OBJECT"
        );
    }

    #[test]
    fn test_builtin_len_fn() {
        let cases = vec![
            (r#"len("H")"#, testing_result!(Int, 1)),
            (
                r#"len(1)"#,
                TestingResult::Err("argument to `len` not supported, got INTEGER".into()),
            ),
            (
                r#"len("H", "w")"#,
                TestingResult::Err("wrong number of arguments. got=2, want=1".into()),
            ),
            (r#"len([1])"#, TestingResult::Int(1)),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_array_literal() {
        let cases = vec![
            ("[1,2,3];", testing_result!(Vec, vec![1, 2, 3])),
            ("[1,2+1,3];", testing_result!(Vec, vec![1, 3, 3])),
            ("[1,2+5,3];", testing_result!(Vec, vec![1, 7, 3])),
            (
                "let a = fn() { 5 }; [1,a(),3];",
                testing_result!(Vec, vec![1, 5, 3]),
            ),
            (
                "let b = 5; let a = fn() { b }; [1,a(),3];",
                testing_result!(Vec, vec![1, 5, 3]),
            ),
        ];

        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_index_expression() {
        let cases = vec![
            ("[1,2,3][0]", testing_result!(Int, 1)),
            ("[1,2,3][1]", testing_result!(Int, 2)),
            ("[1,2,3][2]", testing_result!(Int, 3)),
            ("let i = 0; [1][i]", testing_result!(Int, 1)),
            ("[1,2,3][1+1]", testing_result!(Int, 3)),
            ("let arr=[1,2,3]; arr[2]", testing_result!(Int, 3)),
            (
                "let arr=[1,2,3]; arr[0] + arr[1] + arr[2]",
                testing_result!(Int, 6),
            ),
            ("[1,2,3][3]", TestingResult::Nil),
            ("[1,2,3][-1]", TestingResult::Nil),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }
    #[test]
    fn test_first_builtin_fn() {
        let cases = vec![
            ("first([1,2,3])", testing_result!(Int, 1)),
            ("first([])", testing_result!(Nil)),
            (
                "let a = [1,2,3]; first([1,2,3]); a",
                testing_result!(Vec, vec![1, 2, 3]),
            ),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_last_builtin_fn() {
        let cases = vec![
            ("last([1,2,3])", testing_result!(Int, 3)),
            ("last([])", testing_result!(Nil)),
            (
                "let a = [1,2,3]; last([1,2,3]); a",
                testing_result!(Vec, vec![1, 2, 3]),
            ),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_rest_builtin_fn() {
        let cases = vec![
            ("rest([1,2,3])", testing_result!(Vec, vec![2, 3])),
            ("rest([])", testing_result!(Vec, vec![])),
            (
                "let a = [1,2,3]; rest(a); a",
                testing_result!(Vec, vec![1, 2, 3]),
            ),
            ("let a = [1,2,3]; rest(a)", testing_result!(Vec, vec![2, 3])),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_push_builtin_fn() {
        let cases = vec![
            ("push([1,2,3], 4)", testing_result!(Vec, vec![1, 2, 3, 4])),
            ("push([], 1)", testing_result!(Vec, vec![1])),
            ("let a = []; push(a, 1); a", testing_result!(Vec, vec![1])),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_compose_builtin_array_fn() {
        let cases = vec![
            (
                r#"
        let map = fn(arr, f) {
            let iter = fn(arr, acc) {
                if (len(arr) == 0) {
                    acc
                } else {
                    iter(rest(arr), push(acc, f(first(arr))));
                }
            };

            iter(arr, []);
        };

        let a = [1,2,3];
        map(a, fn(x) { x * 2 })
"#,
                testing_result!(Vec, vec![2, 4, 6]),
            ),
            (
                r#"
            let reduce = fn(arr, initial, f) {
            let iter = fn(arr, result) {
                if (len(arr) == 0) {
                    result
                } else {
                    iter(rest(arr), f(result, first(arr)));
                }
            };

            iter(arr, initial);

        };

        let a = [1,2,3];
        reduce(a, 0, fn(x,y) { x + y })"#,
                testing_result!(Int, 6),
            ),
            (
                r#"
            let reduce = fn(arr, initial, f) {
            let iter = fn(arr, result) {
                if (len(arr) == 0) {
                    result
                } else {
                    iter(rest(arr), f(result, first(arr)));
                }
            };

            iter(arr, initial);

        };

        let sum = fn(arr) {
            reduce(arr, 0, fn(x, y) { x + y })
        };

        let a = [1,2,3];
        sum(a)"#,
                testing_result!(Int, 6),
            ),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_hash_eval() {
        let cases = vec![
            ("{}", testing_result!(Hash, HashMap::new())),
            (r#"{"one": 1}"#, testing_result!(Hash, HashMap::new())),
            (
                r#"{"one": 1, "two": 1 + 1}"#,
                testing_result!(Hash, HashMap::new()),
            ),
            (
                r#"{"on" + "e": 1, "two": 1 + 1}"#,
                testing_result!(Hash, HashMap::new()),
            ),
        ];
        cases.iter().for_each(|(case, _out)| {
            test_eval(case);
            // handle_test(case, out);
        });
    }

    #[test]
    fn test_hash_index() {
        let cases = vec![
            (r#"{"one": 1}["one"]"#, testing_result!(Int, 1)),
            (r#"{"foo": 5}["bar"]"#, testing_result!(Nil)),
            (
                r#"let key = "foo"; {"foo": 5}[key]"#,
                testing_result!(Int, 5),
            ),
            (r#"{}["foo"]"#, testing_result!(Nil)),
            (r#"{5:5}[5]"#, testing_result!(Int, 5)),
            (r#"{true: 5}[true]"#, testing_result!(Int, 5)),
            (r#"{false: 5}[false]"#, testing_result!(Int, 5)),
            (
                r#"{"name": 5}[fn(x) {x} ]"#,
                testing_result!(Err, "unusable as hash key: FUNCTION_OBJECT"),
            ),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_add_return_value() {
        let cases = vec![
            (r#"fn() { true }() == true;"#, testing_result!(Bool, true)),
            (r#"fn() { 5 }() == 5;"#, testing_result!(Bool, true)),
            (r#"fn() { 5 }() + 5;"#, testing_result!(Int, 10)),
            (r#"fn() { 5 }() - 5;"#, testing_result!(Int, 0)),
            (r#"fn() { 5 }() * 5;"#, testing_result!(Int, 25)),
            (r#"fn() { 5 }() / 5;"#, testing_result!(Int, 1)),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_float_value() {
        let cases = vec![
            (r#"3.14"#, testing_result!(Float, 3.14)),
            (r#"3.0 + 5.1"#, testing_result!(Float, 8.1)),
            (r#"3.0 - 5.1"#, testing_result!(Float, 3.0 - 5.1)),
            (r#"3.0 * 5.1"#, testing_result!(Float, 3.0 * 5.1)),
            (r#"3.0 / 5.1"#, testing_result!(Float, 3.0 / 5.1)),
            (r#"3 + 5.1"#, testing_result!(Float, 3f64 + 5.1)),
            (r#"3 - 5.1"#, testing_result!(Float, 3f64 - 5.1)),
            (r#"3 * 5.1"#, testing_result!(Float, 3f64 * 5.1)),
            (r#"3 / 5.1"#, testing_result!(Float, 3f64 / 5.1)),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_assign() {
        let cases = vec![
            (r#"let a = 0; a = 1; a;"#, testing_result!(Int, 1)),
            (
                r#"let a = 0; if (true) { a = 1;}; a;"#,
                testing_result!(Int, 1),
            ),
            (
                r#"let a = 0; let b = fn() { a = 1;}; b(); a;"#,
                testing_result!(Int, 1),
            ),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_update() {
        let cases = vec![
            (r#"let a = 0; a += 1; a;"#, testing_result!(Int, 1)),
            (
                r#"let a = 0; if (true) { a += 1;}; a;"#,
                testing_result!(Int, 1),
            ),
            (
                r#"let a = 0; let b = fn() { a += 1;}; b(); a;"#,
                testing_result!(Int, 1),
            ),
            (r#"let a = 0; a -= 1; a;"#, testing_result!(Int, -1)),
            (
                r#"let a = 0; if (true) { a -= 1;}; a;"#,
                testing_result!(Int, -1),
            ),
            (
                r#"let a = 0; let b = fn() { a -= 1;}; b(); a;"#,
                testing_result!(Int, -1),
            ),
            (r#"let a = 2; a *= 5; a;"#, testing_result!(Int, 10)),
            (
                r#"let a = 2; if (true) { a *= 5;}; a;"#,
                testing_result!(Int, 10),
            ),
            (
                r#"let a = 2; let b = fn() { a *= 5;}; b(); a;"#,
                testing_result!(Int, 10),
            ),
            (r#"let a = 10; a /= 2; a;"#, testing_result!(Int, 5)),
            (
                r#"let a = 10; if (true) { a /= 2;}; a;"#,
                testing_result!(Int, 5),
            ),
            (
                r#"let a = 10; let b = fn() { a /= 2;}; b(); a;"#,
                testing_result!(Int, 5),
            ),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }

    #[test]
    fn test_while_loop() {
        let cases = vec![
            (
                r#"let a = 0; while (a < 1) { a += 1; break; }; a;"#,
                testing_result!(Int, 1),
            ),
            (
                r#"let a = 0; while (a < 1) { break; }; a;"#,
                testing_result!(Int, 0),
            ),
            (
                r#"
let a = 0;
let b = fn() {
  while (a < 5) { a += 1; };
  return a;
};
b();
            "#,
                testing_result!(Int, 5),
            ),
            // (r#"3.0 - 5.1"#, testing_result!(Float, 3.0 - 5.1)),
            // (r#"3.0 * 5.1"#, testing_result!(Float, 3.0 * 5.1)),
            // (r#"3.0 / 5.1"#, testing_result!(Float, 3.0 / 5.1)),
            // (r#"3 + 5.1"#, testing_result!(Float, 3f64 + 5.1)),
            // (r#"3 - 5.1"#, testing_result!(Float, 3f64 - 5.1)),
            // (r#"3 * 5.1"#, testing_result!(Float, 3f64 * 5.1)),
            // (r#"3 / 5.1"#, testing_result!(Float, 3f64 / 5.1)),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test_object(case, out);
        });
    }
}

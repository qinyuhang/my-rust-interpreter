#[cfg(test)]
mod test {
    use crate::*;
    use ::ast::*;
    use ::lexer::*;
    use ::parser::*;
    use ::token::*;
    use std::collections::HashMap;

    #[allow(dead_code)]
    enum FinalResult {
        STRING(String),
        Int(i64),
        Bool(bool),
        Vec(Vec<i64>),
        Err(String),
        Nil,
        Hash(HashMap<Rc<String>, Box<FinalResult>>),
    }

    macro_rules! f {
        (String, $e:expr) => {
            FinalResult::String($e.to_string())
        };
        (Int, $e:expr) => {
            FinalResult::Int($e)
        };
        (Bool, $e:expr) => {
            FinalResult::Bool($e)
        };
        (Vec, $e:expr) => {
            FinalResult::Vec($e)
        };
        (Err, $e:expr) => {
            FinalResult::Err($e.to_string())
        };
        (Nil) => {
            FinalResult::Nil
        };
        (Hash, $e:expr) => {
            FinalResult::Hash($e)
        };
    }

    macro_rules! my_hash {
        () => {};
    }

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", f!(Int, 5)),
            ("10", f!(Int, 10)),
            ("-5", f!(Int, -5)),
            ("-10", f!(Int, -10)),
            ("5 + 5 + 5 + 5 - 10", f!(Int, 10)),
            ("2 * 2 * 2 * 2 * 2", f!(Int, 32)),
            ("-50 + 100 + -50", f!(Int, 0)),
            ("5 * 2 + 10", f!(Int, 20)),
            ("5 + 2 * 10", f!(Int, 25)),
            ("20 + 2 * -10", f!(Int, 0)),
            ("50 / 2 * 2 + 10", f!(Int, 60)),
            ("2 * (5 + 10)", f!(Int, 30)),
            ("3 * 3 * 3 + 10", f!(Int, 37)),
            ("3 * (3 * 3) + 10", f!(Int, 37)),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", f!(Int, 50)),
        ];

        tests.iter().for_each(|(input, expected)| {
            handle_test(input, expected);
        });
    }

    #[test]
    fn test_boolean_expression() {
        let tests = vec![
            ("true", f!(Bool, true)),
            ("false", f!(Bool, false)),
            ("1 < 2", f!(Bool, true)),
            ("1 > 2", f!(Bool, false)),
            ("1 < 1", f!(Bool, false)),
            ("1 > 1", f!(Bool, false)),
            ("1 == 1", f!(Bool, true)),
            ("1 != 1", f!(Bool, false)),
            ("1 == 2", f!(Bool, false)),
            ("1 != 2", f!(Bool, true)),
            ("true == true", f!(Bool, true)),
            ("false == false", f!(Bool, true)),
            ("true == false", f!(Bool, false)),
            ("true != false", f!(Bool, true)),
            ("false != true", f!(Bool, true)),
            ("(1 < 2) == true", f!(Bool, true)),
            ("(1 < 2) == false", f!(Bool, false)),
            ("(1 > 2) == true", f!(Bool, false)),
            ("(1 > 2) == false", f!(Bool, true)),
        ];

        tests.iter().for_each(|(input, expected)| {
            handle_test(input, expected);
        });
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", f!(Bool, false)),
            ("!false", f!(Bool, true)),
            ("!5", f!(Bool, false)),
            ("!!true", f!(Bool, true)),
            ("!!false", f!(Bool, false)),
            ("!!5", f!(Bool, true)),
            // ("!null", f!(Bool, true)),
        ];
        tests.iter().for_each(|(input, expected)| {
            handle_test(input, expected);
        });
    }

    #[test]
    fn test_if_else_expressions() {
        let tests = vec![
            ("if (true) { 10 }", f!(Int, 10)),
            ("if (false) { 10 }", f!(Nil)),
            ("if (1) { 10 }", f!(Int, 10)),
            ("if (1 < 2) { 10 }", f!(Int, 10)),
            ("if (1 > 2) { 10 }", f!(Nil)),
            ("if (1 > 2) { 10 } else { 20 }", f!(Int, 20)),
            ("if (1 < 2) { 10 } else { 20 }", f!(Int, 10)),
        ];

        tests.iter().for_each(|(input, value)| {
            handle_test(input, value);
        })
    }

    #[test]
    fn test_hex_binary_string() {
        let tests = vec![
            ("0x01", f!(Int, 1)),
            ("0xf", f!(Int, 15)),
            ("0b1", f!(Int, 1)),
            ("0x1_000", f!(Int, 0x1_000)),
            ("0x1_000_000", f!(Int, 0x1_000_000)),
            ("0x1_000_", f!(Int, 0x1_000_)),
        ];

        tests.iter().for_each(|(input, value)| {
            handle_test(input, value);
        });
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", f!(Int, 10)),
            ("return 10; 9;", f!(Int, 10)),
            ("return 5 * 2; 9;", f!(Int, 10)),
            ("9; return 2 * 5; 9;", f!(Int, 10)),
            (
                r#"if (10 > 1) {
                       if (10 > 1) {  return 10;  }
                       return 1;
                   }"#,
                f!(Int, 10),
            ),
        ];

        tests.iter().for_each(|(input, expected)| {
            handle_test(input, expected);
        });
    }

    #[test]
    fn test_function_declaration() {
        let tests = vec![
            ("let b = 5; let a = fn() { b }; a();", f!(Int, 5)),
            (
                "let b = 5; let a = fn() { b }; let b = 10; a();",
                f!(Int, 10),
            ),
            ("fn () { 1; }();", f!(Int, 1)),
            // ("fn a() {}", None::<Rc<dyn Object>>),
            // ("fn a(i) {}", None::<Rc<dyn Object>>),
            // ("fn a(x, y) {}", None::<Rc<dyn Object>>),
            ("let add = fn (x, y) { x + y; }; add (1, 1);", f!(Int, 2)),
            ("let add = fn (x, y) { x + y; }; add(1, 1);", f!(Int, 2)),
            ("let a = fn a(x, y) { return x + y; }; a(1,1);", f!(Int, 2)),
            // (
            //     "let a = fn a(x, y) { return x + y; }; return a(1,1);",
            //     2,
            // ),
            ("fn (x) { x; }(5)", f!(Int, 5)),
            (
                r#"
            let add = fn(a, b) { a + b; };
            let sub = fn(a, b) { a - b; };
            let applyFunc = fn(a, b, func) { func(a, b) };
            applyFunc(2, 2, add);
            "#,
                f!(Int, 4),
            ),
        ];
        tests.iter().for_each(|(input, expected)| {
            handle_test(input, expected);
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
            ("fn a(x, y) { return x + y; }; a(1, 2);", f!(Int, 3)),
            ("let a = fn a(x, y) { return x + y; }; a(1, 2);", f!(Int, 3)),
        ];
        tests.iter().for_each(|(input, expected)| {
            handle_test(input, expected);
        });
    }

    #[test]
    fn test_error_object_eval() {
        let test_cases = vec![
            ("5 + true;", f!(Err, "type mismatch: INTEGER + BOOLEAN")),
            ("5 + true; 5", f!(Err, "type mismatch: INTEGER + BOOLEAN")),
            ("-true", f!(Err, "unknown operator: -BOOLEAN")),
            (
                "true + false",
                f!(Err, "unknown operator: BOOLEAN + BOOLEAN"),
            ),
            (
                "5; true + false; 5",
                f!(Err, "unknown operator: BOOLEAN + BOOLEAN"),
            ),
            (
                "if (10 > 1) { true + false; }",
                f!(Err, "unknown operator: BOOLEAN + BOOLEAN"),
            ),
            (
                "if (10 > 1) { true + false; }; return 1;",
                f!(Err, "unknown operator: BOOLEAN + BOOLEAN"),
            ),
            ("foobar", f!(Err, "identifier not found: foobar")),
        ];
        test_cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[test]
    fn test_let_state() {
        let test_cases = vec![
            ("let a = 5; a;", f!(Int, 5)),
            ("let a = 5 * 5; a;", f!(Int, 25)),
            ("let a = 5; let b = a; b;", f!(Int, 5)),
            ("let a = 5; let b = a; let c = a + b + 5; c;", f!(Int, 15)),
        ];
        test_cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
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
            (r#"len("H")"#, f!(Int, 1)),
            (
                r#"len(1)"#,
                FinalResult::Err("argument to `len` not supported, got INTEGER".into()),
            ),
            (
                r#"len("H", "w")"#,
                FinalResult::Err("wrong number of arguments. got=2, want=1".into()),
            ),
            (r#"len([1])"#, FinalResult::Int(1)),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[test]
    fn test_array_literal() {
        let cases = vec![
            ("[1,2,3];", f!(Vec, vec![1, 2, 3])),
            ("[1,2+1,3];", f!(Vec, vec![1, 3, 3])),
            ("[1,2+5,3];", f!(Vec, vec![1, 7, 3])),
            ("let a = fn() { 5 }; [1,a(),3];", f!(Vec, vec![1, 5, 3])),
            (
                "let b = 5; let a = fn() { b }; [1,a(),3];",
                f!(Vec, vec![1, 5, 3]),
            ),
        ];

        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[test]
    fn test_index_expression() {
        let cases = vec![
            ("[1,2,3][0]", f!(Int, 1)),
            ("[1,2,3][1]", f!(Int, 2)),
            ("[1,2,3][2]", f!(Int, 3)),
            ("let i = 0; [1][i]", f!(Int, 1)),
            ("[1,2,3][1+1]", f!(Int, 3)),
            ("let arr=[1,2,3]; arr[2]", f!(Int, 3)),
            ("let arr=[1,2,3]; arr[0] + arr[1] + arr[2]", f!(Int, 6)),
            ("[1,2,3][3]", FinalResult::Nil),
            ("[1,2,3][-1]", FinalResult::Nil),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }
    #[test]
    fn test_first_builtin_fn() {
        let cases = vec![
            ("first([1,2,3])", f!(Int, 1)),
            ("first([])", f!(Nil)),
            ("let a = [1,2,3]; first([1,2,3]); a", f!(Vec, vec![1, 2, 3])),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[test]
    fn test_last_builtin_fn() {
        let cases = vec![
            ("last([1,2,3])", f!(Int, 3)),
            ("last([])", f!(Nil)),
            ("let a = [1,2,3]; last([1,2,3]); a", f!(Vec, vec![1, 2, 3])),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[test]
    fn test_rest_builtin_fn() {
        let cases = vec![
            ("rest([1,2,3])", f!(Vec, vec![2, 3])),
            ("rest([])", f!(Vec, vec![])),
            ("let a = [1,2,3]; rest(a); a", f!(Vec, vec![1, 2, 3])),
            ("let a = [1,2,3]; rest(a)", f!(Vec, vec![2, 3])),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[test]
    fn test_push_builtin_fn() {
        let cases = vec![
            ("push([1,2,3], 4)", f!(Vec, vec![1, 2, 3, 4])),
            ("push([], 1)", f!(Vec, vec![1])),
            ("let a = []; push(a, 1); a", f!(Vec, vec![1])),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
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
                f!(Vec, vec![2, 4, 6]),
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
                f!(Int, 6),
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
                f!(Int, 6),
            ),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[test]
    fn test_hash_eval() {
        let cases = vec![
            ("{}", f!(Hash, HashMap::new())),
            (r#"{"one": 1}"#, f!(Hash, HashMap::new())),
            (r#"{"one": 1, "two": 1 + 1}"#, f!(Hash, HashMap::new())),
            (r#"{"on" + "e": 1, "two": 1 + 1}"#, f!(Hash, HashMap::new())),
        ];
        cases.iter().for_each(|(case, _out)| {
            test_eval(case);
            // handle_test(case, out);
        });
    }

    #[test]
    fn test_hash_index() {
        let cases = vec![
            (r#"{"one": 1}["one"]"#, f!(Int, 1)),
            (r#"{"foo": 5}["bar"]"#, f!(Nil)),
            (r#"let key = "foo"; {"foo": 5}[key]"#, f!(Int, 5)),
            (r#"{}["foo"]"#, f!(Nil)),
            (r#"{5:5}[5]"#, f!(Int, 5)),
            (r#"{true: 5}[true]"#, f!(Int, 5)),
            (r#"{false: 5}[false]"#, f!(Int, 5)),
            (
                r#"{"name": 5}[fn(x) {x} ]"#,
                f!(Err, "unusable as hash key: FUNCTION_OBJECT"),
            ),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[test]
    fn test_add_return_value() {
        let cases = vec![
            (r#"fn() { true }() == true;"#, f!(Bool, true)),
            (r#"fn() { 5 }() == 5;"#, f!(Bool, true)),
            (r#"fn() { 5 }() + 5;"#, f!(Int, 10)),
            (r#"fn() { 5 }() - 5;"#, f!(Int, 0)),
            (r#"fn() { 5 }() * 5;"#, f!(Int, 25)),
            (r#"fn() { 5 }() / 5;"#, f!(Int, 1)),
        ];
        cases.iter().for_each(|(case, out)| {
            handle_test(case, out);
        });
    }

    #[allow(unused)]
    fn handle_test(case: &str, out: &FinalResult) {
        let input = case;
        let evaluated = test_eval(input);
        assert!(evaluated.is_some());
        dbg!(&evaluated);
        match out {
            FinalResult::STRING(s) => {
                test_string_object(evaluated, s.to_string());
            }
            FinalResult::Int(i) => {
                test_integer_object(evaluated, *i);
            }
            FinalResult::Bool(b) => {
                test_boolean_object(evaluated, *b);
            }
            FinalResult::Vec(v) => {
                v.iter()
                    .zip(
                        evaluated
                            .unwrap()
                            .as_any()
                            .downcast_ref::<ArrayObject>()
                            .unwrap()
                            .elements
                            .clone()
                            .borrow()
                            .iter(),
                    )
                    .for_each(|(expected, ev)| {
                        test_integer_object(Some(ev.clone()), *expected);
                    });
            }
            FinalResult::Err(st) => {
                test_error_object(evaluated, st.to_string());
                // convert to ErrorObject
                // let err = ErrorObject::try_from(evaluated.clone().unwrap());
                // assert!(err.is_ok());
                // // println!("{:?}", err.unwrap());
                // assert_eq!(err.unwrap().message, st.to_string());
            }
            FinalResult::Hash(h) => {}
            FinalResult::Nil => {
                test_null_object(&evaluated);
            }
            _ => assert!(false),
        }
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
    fn test_string_object(obj: Option<Rc<dyn Object>>, expected: String) -> bool {
        let i = StringObject::try_from(obj.unwrap());
        assert!(i.is_ok());
        let i = i.unwrap();
        assert_eq!(*i.value, expected);
        true
    }
    #[allow(unused)]
    fn test_error_object(object: Option<Rc<dyn Object>>, expected: String) -> bool {
        let err = ErrorObject::try_from(object.clone().unwrap());
        assert!(err.is_ok());
        // println!("{:?}", err.unwrap());
        assert_eq!(err.unwrap().message, expected.to_string());
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

#[cfg(test)]
mod vm_test {
    use crate::VM;
    use ::ast::Program;
    use ::compiler::*;
    use ::lexer::*;
    use ::parser::*;
    use ::testing::*;
    use ast::{AstExpression, IntegerLiteral};
    use interpreter::testing_object::*;
    use std::collections::HashMap;
    use std::panic::{self, AssertUnwindSafe};
    use std::rc::Rc;

    fn parse(input: &str) -> Option<Program> {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        p.parse_program()
    }

    // FIXME: expected?
    fn run_vm_test(cases: &Vec<(/* input */ &str, /* expected */ TestingResult)>) {
        cases
            .iter()
            .enumerate()
            .for_each(|(index, (input, expected))| {
                let pr = parse(input).unwrap();
                let comp = Compiler::new();
                let c = comp.compile(&pr);
                assert!(
                    c.is_ok(),
                    "fatal compiler error: {}, input={}",
                    c.unwrap_err(),
                    input
                );

                let vm = VM::new(comp.bytecode());

                let r = vm.run();
                if let TestingResult::Throw(str) = expected {
                    assert!(r.is_err());
                    assert_eq!(r.unwrap_err(), str.to_string());
                    return;
                }
                assert!(
                    r.is_ok(),
                    "vm error: {} \nVM Instructions: {}Compiler Instructions: {}",
                    r.unwrap_err(),
                    vm.dump_instruction(),
                    comp.dump_instruction()
                );

                let stack_el = vm.last_popped_stack_el();

                assert!(stack_el.is_some());

                let stack_el = stack_el.unwrap();

                panic::catch_unwind(AssertUnwindSafe(|| {
                    handle_object(Some(stack_el), expected);
                }))
                .map_err(|e| {
                    println!("panic: {:?}", e);
                })
                .expect(
                    format!(
                        "Case failed:\n index={},\n input={},\n instruction={}",
                        index,
                        input,
                        comp.dump_instruction()
                    )
                    .as_str(),
                );
            });
    }

    #[test]
    fn test_integer_arithmetic() {
        let cases = vec![
            ("1", testing_result!(Int, 1)),
            ("2", testing_result!(Int, 2)),
            ("1 + 2", testing_result!(Int, 3)),
            ("2 - 2", testing_result!(Int, 0)),
            ("2 * 2", testing_result!(Int, 4)),
            ("2 / 2", testing_result!(Int, 1)),
            ("1 - 2", testing_result!(Int, 1 - 2)),
            ("1 * 2", testing_result!(Int, 1 * 2)),
            ("4 / 2", testing_result!(Int, 4 / 2)),
            ("50 / 2 * 2 + 10 - 5", testing_result!(Int, 55)),
            ("5 + 5+ 5+5 - 10", testing_result!(Int, 10)),
            ("2*2*2*2*2", testing_result!(Int, 32)),
            ("5 * 2 + 10", testing_result!(Int, 5 * 2 + 10)),
            ("5 + 2 * 10", testing_result!(Int, 5 + 2 * 10)),
            ("5 * (2 + 10)", testing_result!(Int, 5 * (2 + 10))),
            ("-5", testing_result!(Int, -5)),
            ("-10", testing_result!(Int, -10)),
            ("-50 + 100 + -50", testing_result!(Int, -50 + 100 + -50)),
            (
                "(5 + 10 * 2 + 15 / 2) * 2 + -10",
                testing_result!(Int, (5 + 10 * 2 + 15 / 2) * 2 + -10),
            ),
        ];
        run_vm_test(&cases);
    }

    #[test]
    fn test_bool() {
        let cases = vec![
            ("true", testing_result!(Bool, true)),
            ("false", testing_result!(Bool, false)),
            // Int
            ("1<2", testing_result!(Bool, true)),
            ("1>2", testing_result!(Bool, false)),
            ("1<1", testing_result!(Bool, false)),
            ("1>1", testing_result!(Bool, false)),
            // Int eq
            ("1==1", testing_result!(Bool, true)),
            ("1!=1", testing_result!(Bool, false)),
            ("1==2", testing_result!(Bool, false)),
            ("1!=2", testing_result!(Bool, true)),
            // bool
            ("true == true", testing_result!(Bool, true)),
            ("false == false", testing_result!(Bool, true)),
            ("true == false", testing_result!(Bool, false)),
            ("true != false", testing_result!(Bool, true)),
            ("false != true", testing_result!(Bool, true)),
            // Op
            ("(1 < 2) == true", testing_result!(Bool, true)),
            ("(1 < 2) == false", testing_result!(Bool, false)),
            ("(1 > 2) == true", testing_result!(Bool, false)),
            ("(1 > 2) == false", testing_result!(Bool, true)),
            // prefix
            ("!true", testing_result!(Bool, false)),
            ("!false", testing_result!(Bool, true)),
            ("!5", testing_result!(Bool, false)),
            ("!!true", testing_result!(Bool, true)),
            ("!!false", testing_result!(Bool, false)),
            ("!!5", testing_result!(Bool, true)),
            ("!(if (false) { 5; })", testing_result!(Bool, true)),
            (
                "if ((if (false) { 10 })) { 10 } else { 20 }",
                testing_result!(Int, 20),
            ),
        ];
        run_vm_test(&cases);
    }

    #[test]
    fn test_condition() {
        let cases = vec![
            ("if (true) { 10 }", testing_result!(Int, 10)),
            ("if (true) { 10 } else { 20 }", testing_result!(Int, 10)),
            ("if (false) { 10 } else { 20 }", testing_result!(Int, 20)),
            ("if (1) { 10 }", testing_result!(Int, 10)),
            ("if (1 < 2) { 10 }", testing_result!(Int, 10)),
            ("if (1 < 2) { 10 } else { 20 }", testing_result!(Int, 10)),
            ("if (1 > 2) { 10 } else { 20 }", testing_result!(Int, 20)),
            ("if (1 > 2) { 10 }", testing_result!(Nil)),
            ("if (false) { 10 }", testing_result!(Nil)),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_global_let_statements() {
        let cases = vec![
            ("let one = 1; one", testing_result!(Int, 1)),
            (
                "let one = 1; let two = 2; one + two",
                testing_result!(Int, 3),
            ),
            (
                "let one = 1; let two = one + one; one + two",
                testing_result!(Int, 3),
            ),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_string_expression() {
        let cases = vec![
            (r#""monkey""#, testing_result!(String, "monkey")),
            (r#""mon" + "key""#, testing_result!(String, "monkey")),
            (
                r#""mon" + "key" + "banana""#,
                testing_result!(String, "monkeybanana"),
            ),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_array_literal() {
        let cases = vec![
            (r#"[]"#, testing_result!(Vec, vec![])),
            (r#"[1,2,3]"#, testing_result!(Vec, vec![1, 2, 3])),
            (r#"[1+2, 3*4, 5+6]"#, testing_result!(Vec, vec![3, 12, 11])),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_hash_literal() {
        let cases = vec![
            (r#"{}"#, testing_result!(Hash, HashMap::from([]))),
            (
                r#"{1: 2, 2: 3 }"#,
                testing_result!(
                    Hash,
                    HashMap::from([
                        (
                            Rc::new(AstExpression::IntegerLiteral(IntegerLiteral {
                                value: 1,
                                token: Default::default()
                            })),
                            testing_result!(Int, 2)
                        ),
                        (
                            Rc::new(AstExpression::IntegerLiteral(IntegerLiteral {
                                value: 2,
                                token: Default::default()
                            })),
                            testing_result!(Int, 3)
                        ),
                    ])
                ),
            ),
            (
                r#"{1 + 1: 2 * 2}"#,
                testing_result!(
                    Hash,
                    HashMap::from([(
                        Rc::new(AstExpression::IntegerLiteral(IntegerLiteral {
                            value: 2,
                            token: Default::default()
                        })),
                        testing_result!(Int, 4)
                    ),])
                ),
            ),
            (
                r#"{1 + 1: 2 * 2, 3 + 3: 4 * 4}"#,
                testing_result!(
                    Hash,
                    HashMap::from([
                        (
                            Rc::new(AstExpression::IntegerLiteral(IntegerLiteral {
                                value: 2,
                                token: Default::default()
                            })),
                            testing_result!(Int, 4)
                        ),
                        (
                            Rc::new(AstExpression::IntegerLiteral(IntegerLiteral {
                                value: 6,
                                token: Default::default()
                            })),
                            testing_result!(Int, 16)
                        ),
                    ])
                ),
            ),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_index() {
        let cases = vec![
            ("[1, 2, 3][1]", testing_result!(Int, 2)),
            ("[1, 2, 3][0 + 2]", testing_result!(Int, 3)),
            ("[[1, 1, 1]][0][0]", testing_result!(Int, 1)),
            ("[][0]", testing_result!(Nil)),
            ("[1, 2, 3][99]", testing_result!(Nil)),
            ("[1][-1]", testing_result!(Nil)),
            ("{1: 1, 2: 2}[1]", testing_result!(Int, 1)),
            ("{1: 1, 2: 2}[2]", testing_result!(Int, 2)),
            ("{1: 1}[0]", testing_result!(Nil)),
            ("{}[0]", testing_result!(Nil)),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_function_calls_no_arg() {
        let cases = vec![
            (
                "let fivePlusTen = fn() { 5 + 10 }; fivePlusTen()",
                testing_result!(Int, 15),
            ),
            (
                r#"let one = fn() { 1 };
let two = fn() { 2 };
one() + two()"#,
                testing_result!(Int, 3),
            ),
            (
                r#"let a = fn() { 1 };
let b = fn() { a() + 1 };
let c = fn() { b() + 1 };
c()"#,
                testing_result!(Int, 3),
            ),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_function_calls_with_return() {
        let cases = vec![
            (
                "let earlyExit = fn() { return 99; 100 }; earlyExit()",
                testing_result!(Int, 99),
            ),
            (
                "let earlyExit = fn() { return 99; return 100; }; earlyExit()",
                testing_result!(Int, 99),
            ),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_function_calls_without_return_value() {
        let cases = vec![
            ("let noReturn = fn() { }; noReturn()", testing_result!(Nil)),
            (
                r#"
let noReturn = fn() { };
let n = fn() { noReturn(); };
noReturn();
n();
"#,
                testing_result!(Nil),
            ),
        ];

        run_vm_test(&cases);
    }

    #[test]
    fn test_first_class_function() {
        let cases = vec![
            (
                r#"
let fOne = fn() { 1 };
let fTwo = fn() { fOne };
fTwo()();
"#,
                testing_result!(Int, 1),
            ),
            (
                r#"let returnOneReturner = fn() {
let returnsOne = fn () { 1 };
returnsOne;
};
returnOneReturner()()
}"#,
                testing_result!(Int, 1),
            ),
        ];

        run_vm_test(&cases);
    }

    //            │              │
    // VM SP  ───►│              │
    //            ├──────────────┤
    //            │Local 2       │◄────┐
    //            ├──────────────┤     │Reserved for Local bindings
    //            │Local 1       │◄────┘
    //            ├──────────────┤
    //            │Function      │
    //            ├──────────────┤
    //            │Other Value 2 │◄────┐
    //            ├──────────────┤     │Pushed before call fn
    //            │Other Value 1 │◄────┘
    //            └──────────────┘
    #[test]
    fn test_calling_functions_with_bindings() {
        let cases = vec![
            (
                r#"let one = fn() { let one = 1; one }; one()"#,
                testing_result!(Int, 1),
            ),
            (
                r#"let oneAndTwo = fn() { let one = 1; let two = 2; one + two }; oneAndTwo()"#,
                testing_result!(Int, 3),
            ),
            (
                r#"let oneAndTwo = fn() { let one = 1; let two = 2; one + two };
let threeAndFour = fn() { let three = 3; let four = 4; three + four };
oneAndTwo() + threeAndFour()"#,
                testing_result!(Int, 10),
            ),
            (
                r#"let firstFoobar = fn() { let foobar = 50; foobar };
let secondFoobar = fn() { let foobar = 100; foobar };
firstFoobar() + secondFoobar()"#,
                testing_result!(Int, 150),
            ),
            // FIXME: why without comma, compile failed
            (
                r#"let globalSeed = 50;
let minOne = fn() { let num = 1; globalSeed - num };
let minTwo = fn() { let num = 2; globalSeed - num };
minOne() + minTwo()"#,
                testing_result!(Int, 97),
            ),
        ];
        run_vm_test(&cases);
    }
    #[test]
    fn test_calling_functions_with_args_and_bindings() {
        let cases = vec![
            (
                r#"let identity = fn(a) { a; };
identity(4);"#,
                testing_result!(Int, 4),
            ),
            (
                r#"let sum = fn(a, b) { a + b; }; sum(1, 2);"#,
                testing_result!(Int, 3),
            ),
            (
                r#"let sum = fn(a, b) {
let c = a + b;
c; };
sum(1, 2);"#,
                testing_result!(Int, 3),
            ),
            (
                r#"let sum = fn(a, b) {
let c = a + b;
c; };
sum(1, 2) + sum(3, 4);"#,
                testing_result!(Int, 10),
            ),
            (
                r#"let sum = fn(a, b) {
    let c = a + b;
c; };
let outer = fn() {
    sum(1, 2) + sum(3, 4)
};
outer()"#,
                testing_result!(Int, 10),
            ),
            (
                r#"
let globalNum = 10;
let sum = fn(a, b) {
    let c = a + b;
    c + globalNum;
};
let outer = fn() {
    sum(1, 2) + sum(3, 4) + globalNum
};
outer() + globalNum"#,
                testing_result!(Int, 50),
            ),
        ];
        run_vm_test(&cases);
    }

    #[test]
    fn test_calling_function_with_wrong_args() {
        let cases = vec![
            (
                r#"fn() {1}(1)"#,
                testing_result!(Throw, "wrong number of arguments: want=0, got=1"),
            ),
            (
                r#"fn(a) {a}()"#,
                testing_result!(Throw, "wrong number of arguments: want=1, got=0"),
            ),
            (
                r#"fn(a, b) {a + b}(1)"#,
                testing_result!(Throw, "wrong number of arguments: want=2, got=1"),
            ),
        ];
        run_vm_test(&cases);
    }

    #[test]
    fn test_builtin_functions() {
        testing::cases::BUILTIN_CASES.with(|cases| {
            run_vm_test(&cases);
        });
    }

    #[test]
    fn test_closure() {
        testing::cases::CLOSURE_CASE.with(|cases| {
            run_vm_test(&cases);
        });
    }

    #[test]
    fn test_recursive() {
        testing::cases::RECUSIVE_CASE.with(|cases| {
            run_vm_test(&cases);
        });
    }
}

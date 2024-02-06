#[cfg(test)]
mod compiler_test {
    use crate::symbol_table::*;
    use crate::*;
    use ::code::*;
    use ::lexer::*;
    use ::parser::*;
    use ::testing::*;
    use code::OpCode::{OpAdd, OpMul, OpSub};
    use interpreter::testing_object::handle_object;
    use std::collections::HashMap;
    use std::panic::{self, AssertUnwindSafe};

    struct CompileTestCase<'a> {
        pub input: &'a str,
        pub expected_constants: Vec<TestingResult>,
        pub expected_instruction: Vec<Instructions>,
    }

    #[test]
    fn test_integer_arithmetic() {
        let v = vec![0, 1, 2];
        let cases = vec![
            CompileTestCase {
                input: "1 + 2",
                expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpAdd, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "1; 2",
                expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpPop, &v[0..0]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "1 - 2",
                expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpSub, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "1 * 2",
                expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpMul, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "2 / 1",
                expected_constants: vec![testing_result!(Int, 2), testing_result!(Int, 1)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpDiv, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "-1",
                expected_constants: vec![testing_result!(Int, 1)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpMinus, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];

        run_compile_test(cases);
    }

    #[test]
    fn test_boolean_expression() {
        let v = vec![0, 1, 2];
        let cases = vec![
            CompileTestCase {
                input: "true",
                expected_constants: vec![],
                expected_instruction: vec![
                    make(&OpCode::OpTrue, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "false",
                expected_constants: vec![],
                expected_instruction: vec![
                    make(&OpCode::OpFalse, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "1 > 2",
                expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpGreaterThan, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "1 < 2",
                expected_constants: vec![testing_result!(Int, 2), testing_result!(Int, 1)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpGreaterThan, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "1 == 2",
                expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpEqual, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "1 != 2",
                expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpNotEqual, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "true == false",
                expected_constants: vec![],
                expected_instruction: vec![
                    make(&OpCode::OpTrue, &v[0..0]),
                    make(&OpCode::OpFalse, &v[1..1]),
                    make(&OpCode::OpEqual, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "true != false",
                expected_constants: vec![],
                expected_instruction: vec![
                    make(&OpCode::OpTrue, &v[0..0]),
                    make(&OpCode::OpFalse, &v[1..1]),
                    make(&OpCode::OpNotEqual, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "!true",
                expected_constants: vec![],
                expected_instruction: vec![
                    make(&OpCode::OpTrue, &v[0..0]),
                    make(&OpCode::OpBang, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];

        run_compile_test(cases);
    }

    #[test]
    fn test_conditions() {
        let v = vec![0, 1, 2, 7];
        let cases = vec![
            CompileTestCase {
                input: "if (true) { 10 }; 3333;",
                expected_constants: vec![testing_result!(Int, 10), testing_result!(Int, 3333)],
                expected_instruction: vec![
                    // 0000
                    make(&OpCode::OpTrue, &v[0..0]),
                    // 0001
                    make(&OpCode::OpJNT, &vec![10]),
                    // 0004
                    make(&OpCode::OpConstant, &v[0..1]),
                    // 0007
                    make(&OpCode::OpJMP, &vec![11]),
                    // 0010
                    make(&OpCode::OpNull, &v[0..0]),
                    // 0011
                    make(&OpCode::OpPop, &v[0..0]),
                    // 0012
                    make(&OpCode::OpConstant, &v[1..2]),
                    // 0015
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "if (true) { 10 } else { 20 }; 3333;",
                expected_constants: vec![
                    testing_result!(Int, 10),
                    testing_result!(Int, 20),
                    testing_result!(Int, 3333),
                ],
                expected_instruction: vec![
                    // 0000
                    make(&OpCode::OpTrue, &v[0..0]),
                    // 0001
                    make(&OpCode::OpJNT, &vec![10]),
                    // 0004
                    make(&OpCode::OpConstant, &vec![0]),
                    // 0007
                    make(&OpCode::OpJMP, &vec![13]),
                    // 0010
                    make(&OpCode::OpConstant, &vec![1]),
                    // 0013
                    make(&OpCode::OpPop, &v[0..0]),
                    // 0014
                    make(&OpCode::OpConstant, &vec![2]),
                    // 0017
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];

        run_compile_test(cases);
    }

    #[test]
    fn test_let() {
        let v = vec![0, 1, 2, 7];
        let cases = vec![
            CompileTestCase {
                input: "let one = 1; let two = 2;",
                expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &vec![0]),
                    make(&OpCode::OpSetGlobal, &vec![0]),
                    make(&OpCode::OpConstant, &vec![1]),
                    make(&OpCode::OpSetGlobal, &vec![1]),
                ],
            },
            CompileTestCase {
                input: "let one = 1; one;",
                expected_constants: vec![testing_result!(Int, 1)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &vec![0]),
                    make(&OpCode::OpSetGlobal, &vec![0]),
                    make(&OpCode::OpGetGlobal, &vec![0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: "let one = 1; let two = one; two;",
                expected_constants: vec![testing_result!(Int, 1)],
                expected_instruction: vec![
                    make(&OpCode::OpConstant, &vec![0]),
                    make(&OpCode::OpSetGlobal, &vec![0]),
                    make(&OpCode::OpGetGlobal, &vec![0]),
                    make(&OpCode::OpSetGlobal, &vec![1]),
                    make(&OpCode::OpGetGlobal, &vec![1]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];

        run_compile_test(cases);
    }

    fn run_compile_test(tests: Vec<CompileTestCase>) {
        tests.iter().for_each(
            |CompileTestCase {
                 input,
                 expected_constants,
                 expected_instruction,
             }| {
                let lex = Lexer::new(*input);
                let p = Parser::new(lex);
                let pr = p.parse_program();
                assert!(pr.is_some(), "parse program failed");
                let pr = pr.unwrap();
                // dbg!(&pr);
                let compiler = Compiler::new();
                let r = compiler.compile(&pr);
                assert!(r.is_ok(), "compile failed: {}", r.unwrap_err());
                dbg!("compile succeed");
                // dbg!(&compiler.dump_instruction());
                let bytecode = compiler.bytecode();
                handle_instructions(expected_instruction.clone(), &bytecode.instructions);

                handle_constants(expected_constants, &*bytecode.constants.borrow());
            },
        );
    }

    fn handle_instructions(expected: Vec<Instructions>, actual: &Instructions) {
        let concat_ins = concat_instructions(expected);
        assert_eq!(
            concat_ins.len(),
            actual.len(),
            "wrong instruction length.\nwanted={}\ngot={}",
            format_display_instructions(&concat_ins),
            format_display_instructions(&actual)
        );
        concat_ins
            .iter()
            .zip(actual.iter())
            .enumerate()
            .for_each(|(idx, (ex, ac))| {
                assert_eq!(
                    *ex,
                    *ac,
                    r#"wrong instruction at {idx}.
wanted={}
got   ={}
wanted instructions={}
got    instructions={}
wanted instructions vec={:?}
got    instructions vec={:?}
"#,
                    *ex,
                    *ac,
                    format_display_instructions(&concat_ins),
                    format_display_instructions(&actual),
                    &concat_ins,
                    &actual
                );
            })
    }

    fn handle_constants(expected: &Vec<TestingResult>, actual: &Vec<Rc<dyn Object>>) {
        assert_eq!(
            expected.len(),
            actual.len(),
            "wrong number of constants. got={}, want={}",
            actual.len(),
            expected.len()
        );

        expected
            .iter()
            .zip(actual.iter())
            .enumerate()
            .for_each(|(idx, (ex, ac))| {
                let result = panic::catch_unwind(AssertUnwindSafe(|| {
                    handle_object(Some(ac.clone()), ex);
                }));
                assert!(
                    result.is_ok(),
                    "constant #{} testing failed: {}\n wanted={},\n got={}",
                    idx,
                    result
                        .unwrap_err()
                        .downcast_ref::<String>()
                        .unwrap_or(&"failed".to_string()),
                    ex,
                    ac
                );
            });
    }

    fn concat_instructions(s: Vec<Instructions>) -> Instructions {
        s.into_iter().flatten().collect()
    }

    #[test]
    fn test_helper_concat_instructions() {
        let input = vec![0, 1, 2, 3, 4];
        let out = concat_instructions(vec![input.clone(), input.clone()]);
        assert_eq!(out, vec![0, 1, 2, 3, 4, 0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_define() {
        let expected = HashMap::from([
            (
                "a".to_string(),
                Symbol {
                    name: Rc::new(Identifier::from("a".to_string())),
                    scope: GLOBAL_SCOPE,
                    index: 0,
                },
            ),
            (
                "b".to_string(),
                Symbol {
                    name: Rc::new(Identifier::from("b".to_string())),
                    scope: GLOBAL_SCOPE,
                    index: 1,
                },
            ),
        ]);

        let global = SymbolTable::new();
        let a = global.define(Rc::new(Identifier::from("a".to_string())));
        assert_eq!(*a, *expected.get("a").unwrap());

        let b = global.define(Rc::new(Identifier::from("b".to_string())));
        assert_eq!(*b, *expected.get("b").unwrap());
    }

    #[test]
    fn test_resolve() {
        let global = SymbolTable::new();
        global.define(Rc::new(Identifier::from("a".to_string())));
        global.define(Rc::new(Identifier::from("b".to_string())));

        let expected = vec![
            (
                "a",
                Symbol {
                    name: Rc::new(Identifier::from("a".to_string())),
                    scope: GLOBAL_SCOPE,
                    index: 0,
                },
            ),
            (
                "b",
                Symbol {
                    name: Rc::new(Identifier::from("b".to_string())),
                    scope: GLOBAL_SCOPE,
                    index: 1,
                },
            ),
        ];

        expected.iter().for_each(|(name, sy)| {
            let r = global.resolve(Rc::new(Identifier::from(name.to_string())));
            assert!(r.is_ok(), "name {} not resolvable", &sy.name);
            let r = r.unwrap();
            assert_eq!(
                *r, *sy,
                "expected {} to resolve to {:?}, got={:?}",
                &sy.name, sy, *r
            );
        });
    }

    #[test]
    fn test_string() {
        let v = vec![0, 1, 2];
        let cases = vec![
            CompileTestCase {
                input: r#""monkey""#,
                expected_constants: vec![testing_result!(String, "monkey")],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#""mon" + "key""#,
                expected_constants: vec![
                    testing_result!(String, "mon"),
                    testing_result!(String, "key"),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpAdd, &v[0..0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];
        run_compile_test(cases);
    }

    #[test]
    fn test_array_literal() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let cases = vec![
            CompileTestCase {
                input: r#"[]"#,
                expected_constants: vec![],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpArray, &v[0..1]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"[1,2,3]"#,
                expected_constants: vec![
                    testing_result!(Int, 1),
                    testing_result!(Int, 2),
                    testing_result!(Int, 3),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpConstant, &v[2..3]),
                    make(&OpCode::OpArray, &vec![3]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"[1+2,3-4,5*6]"#,
                expected_constants: vec![
                    testing_result!(Int, 1),
                    testing_result!(Int, 2),
                    testing_result!(Int, 3),
                    testing_result!(Int, 4),
                    testing_result!(Int, 5),
                    testing_result!(Int, 6),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpAdd, &vec![]),
                    //
                    make(&OpCode::OpConstant, &v[2..3]),
                    make(&OpCode::OpConstant, &v[3..4]),
                    make(&OpCode::OpSub, &vec![]),
                    //
                    make(&OpCode::OpConstant, &v[4..5]),
                    make(&OpCode::OpConstant, &v[5..6]),
                    make(&OpCode::OpMul, &vec![]),
                    //
                    make(&OpCode::OpArray, &vec![3]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];
        run_compile_test(cases);
    }

    #[test]
    fn test_hash() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cases = vec![
            CompileTestCase {
                input: r#"{1:2,3:4,5:6}"#,
                expected_constants: vec![
                    testing_result!(Int, 1),
                    testing_result!(Int, 2),
                    testing_result!(Int, 3),
                    testing_result!(Int, 4),
                    testing_result!(Int, 5),
                    testing_result!(Int, 6),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    //
                    make(&OpCode::OpConstant, &v[2..3]),
                    make(&OpCode::OpConstant, &v[3..4]),
                    //
                    make(&OpCode::OpConstant, &v[4..5]),
                    make(&OpCode::OpConstant, &v[5..6]),
                    //
                    make(&OpCode::OpHash, &vec![6]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"{1:2+3, 4: 5*6}"#,
                expected_constants: vec![
                    testing_result!(Int, 1),
                    testing_result!(Int, 2),
                    testing_result!(Int, 3),
                    testing_result!(Int, 4),
                    testing_result!(Int, 5),
                    testing_result!(Int, 6),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpConstant, &v[2..3]),
                    make(&OpCode::OpAdd, &vec![]),
                    //
                    make(&OpCode::OpConstant, &v[3..4]),
                    make(&OpCode::OpConstant, &v[4..5]),
                    make(&OpCode::OpConstant, &v[5..6]),
                    make(&OpCode::OpMul, &vec![]),
                    //
                    make(&OpCode::OpHash, &vec![4]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"{1+2:3-4,5*6: 7 +8 }"#,
                expected_constants: vec![
                    testing_result!(Int, 1),
                    testing_result!(Int, 2),
                    testing_result!(Int, 3),
                    testing_result!(Int, 4),
                    testing_result!(Int, 5),
                    testing_result!(Int, 6),
                    testing_result!(Int, 7),
                    testing_result!(Int, 8),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpAdd, &vec![]),
                    //
                    make(&OpCode::OpConstant, &v[2..3]),
                    make(&OpCode::OpConstant, &v[3..4]),
                    make(&OpCode::OpSub, &vec![]),
                    //
                    make(&OpCode::OpConstant, &v[4..5]),
                    make(&OpCode::OpConstant, &v[5..6]),
                    make(&OpCode::OpMul, &vec![]),
                    //
                    make(&OpCode::OpConstant, &v[6..7]),
                    make(&OpCode::OpConstant, &v[7..8]),
                    make(&OpCode::OpAdd, &vec![]),
                    //
                    make(&OpCode::OpHash, &vec![4]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];
        run_compile_test(cases);
    }

    #[test]
    fn test_index() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cases = vec![
            CompileTestCase {
                input: r#"[1,2,3][1+1]"#,
                expected_constants: vec![
                    testing_result!(Int, 1),
                    testing_result!(Int, 2),
                    testing_result!(Int, 3),
                    testing_result!(Int, 1),
                    testing_result!(Int, 1),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpConstant, &v[2..3]),
                    make(&OpCode::OpArray, &v[3..4]),
                    //
                    make(&OpCode::OpConstant, &v[3..4]),
                    make(&OpCode::OpConstant, &v[4..5]),
                    make(&OpCode::OpAdd, &v[4..4]),
                    //
                    make(&OpCode::OpIndex, &vec![]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"{1:2}[2-1]"#,
                expected_constants: vec![
                    testing_result!(Int, 1),
                    testing_result!(Int, 2),
                    testing_result!(Int, 2),
                    testing_result!(Int, 1),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpHash, &vec![2]),
                    //
                    make(&OpCode::OpConstant, &v[2..3]),
                    make(&OpCode::OpConstant, &v[3..4]),
                    make(&OpCode::OpSub, &vec![]),
                    //
                    make(&OpCode::OpIndex, &v[3..3]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];
        run_compile_test(cases);
    }

    // OpConstant 0
    // OpConstant 1
    // OpAdd
    // OpReturnValue
    #[test]
    fn test_functions() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cases = vec![
            CompileTestCase {
                input: r#"fn () { return 5 + 10; }"#,
                expected_constants: vec![
                    testing_result!(Int, 5),
                    testing_result!(Int, 10),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpConstant, &v[0..1]),
                            make(&OpCode::OpConstant, &v[1..2]),
                            make(&OpCode::OpAdd, &v[0..0]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![2, 0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"fn () { return 5 + 10 }"#,
                expected_constants: vec![
                    testing_result!(Int, 5),
                    testing_result!(Int, 10),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpConstant, &v[0..1]),
                            make(&OpCode::OpConstant, &v[1..2]),
                            make(&OpCode::OpAdd, &v[0..0]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![2, 0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"fn () { 5 + 10 }"#,
                expected_constants: vec![
                    testing_result!(Int, 5),
                    testing_result!(Int, 10),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpConstant, &v[0..1]),
                            make(&OpCode::OpConstant, &v[1..2]),
                            make(&OpCode::OpAdd, &v[0..0]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![2, 0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"fn () { 1; 2 }"#,
                expected_constants: vec![
                    testing_result!(Int, 1),
                    testing_result!(Int, 2),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpConstant, &v[0..1]),
                            make(&OpCode::OpPop, &v[0..0]),
                            make(&OpCode::OpConstant, &v[1..2]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![2, 0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];
        run_compile_test(cases);
    }

    #[test]
    fn test_compilation_scopes() {
        let compiler = Compiler::new();
        let binding = compiler.symbol_table.borrow().clone();
        let global_symbol_table = binding.as_ref();
        assert_eq!(
            compiler.scope_index.get(),
            0,
            "scope_index wrong. got={}, wanted={}",
            compiler.scope_index.get(),
            0
        );
        compiler.emit(OpMul, &vec![]);
        compiler.enter_scope();
        assert_eq!(
            compiler.scope_index.get(),
            1,
            "scope_index wrong. got={}, wanted={}",
            compiler.scope_index.get(),
            1
        );
        compiler.emit(OpSub, &vec![]);
        assert_eq!(
            compiler.scope_index.get(),
            1,
            "scope_index wrong. got={}, wanted={}",
            compiler.scope_index.get(),
            1
        );
        {
            let scopes = compiler.scopes.borrow();
            let scopes = scopes.get(compiler.scope_index.get());
            assert!(scopes.is_some(), "except got the last scope");
            let scopes = scopes.unwrap();
            assert_eq!(
                scopes.instructions.borrow().len(),
                1,
                "instruction lens wrong, got={}",
                scopes.instructions.borrow().len()
            );
            let last = scopes.last_instruction.get();
            assert_eq!(
                last.op_code, OpSub,
                "last_instruction.op_code wrong. got={}, want={}",
                last.op_code, OpSub
            );
        }
        {
            assert!(
                compiler
                    .symbol_table
                    .borrow()
                    .outer
                    .borrow()
                    .as_ref()
                    .is_some(),
                "compiler did not enclose symbol table"
            );
            assert_eq!(
                *global_symbol_table,
                *(compiler
                    .symbol_table
                    .borrow()
                    .outer
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .clone()),
                "compiler did not enclose symbol table"
            );
        }
        compiler.leave_scope();
        assert_eq!(
            compiler.scope_index.get(),
            0,
            "scope_index wrong. got={}, wanted={}",
            compiler.scope_index.get(),
            0
        );
        {
            assert!(
                compiler
                    .symbol_table
                    .borrow()
                    .outer
                    .borrow()
                    .as_ref()
                    .is_none(),
                "compiler did not enclose symbol table"
            );
            assert_eq!(
                *global_symbol_table,
                **compiler.symbol_table.borrow(),
                "compiler did not enclose symbol table"
            );
        }
        compiler.emit(OpAdd, &vec![]);
        {
            let scopes = compiler.scopes.borrow();
            let scopes = scopes.get(compiler.scope_index.get());
            assert!(scopes.is_some(), "except got the last scope");
            let scopes = scopes.unwrap();
            assert_eq!(
                scopes.instructions.borrow().len(),
                2,
                "instruction lens wrong, got={}",
                scopes.instructions.borrow().len()
            );
            let last = scopes.last_instruction.get();
            assert_eq!(
                last.op_code, OpAdd,
                "last_instruction.op_code wrong. got={}, want={}",
                last.op_code, OpAdd
            );

            let previous = scopes.previous_instruction.get();
            assert_eq!(
                previous.op_code, OpMul,
                "last_instruction.op_code wrong. got={}, want={}",
                last.op_code, OpMul
            );
        }
    }

    #[test]
    fn test_function_calls() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cases = vec![
            CompileTestCase {
                input: r#"fn () { 24 }()"#,
                expected_constants: vec![
                    testing_result!(Int, 24),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpConstant, &v[0..1]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![1, 0]),
                    make(&OpCode::OpCall, &v[0..1]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"let noArg = fn () { 24 }; noArg();"#,
                expected_constants: vec![
                    testing_result!(Int, 24),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpConstant, &v[0..1]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![1, 0]),
                    make(&OpCode::OpSetGlobal, &v[0..1]),
                    make(&OpCode::OpGetGlobal, &v[0..1]),
                    make(&OpCode::OpCall, &v[0..1]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"let oneArg = fn (a) { a }; oneArg(24);"#,
                expected_constants: vec![
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpGetLocal, &v[0..1]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                    testing_result!(Int, 24),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![0, 0]),
                    make(&OpCode::OpSetGlobal, &v[0..1]),
                    make(&OpCode::OpGetGlobal, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpCall, &v[1..2]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"let manyArg = fn (a, b, c) { a; b; c }; manyArg(24, 25, 26);"#,
                expected_constants: vec![
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpGetLocal, &v[0..1]),
                            make(&OpCode::OpPop, &v[0..0]),
                            make(&OpCode::OpGetLocal, &v[1..2]),
                            make(&OpCode::OpPop, &v[0..0]),
                            make(&OpCode::OpGetLocal, &v[2..3]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                    testing_result!(Int, 24),
                    testing_result!(Int, 25),
                    testing_result!(Int, 26),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![0, 0]),
                    make(&OpCode::OpSetGlobal, &v[0..1]),
                    make(&OpCode::OpGetGlobal, &v[0..1]),
                    make(&OpCode::OpConstant, &v[1..2]),
                    make(&OpCode::OpConstant, &v[2..3]),
                    make(&OpCode::OpConstant, &v[3..4]),
                    make(&OpCode::OpCall, &v[3..4]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];
        run_compile_test(cases);
    }

    #[test]
    fn test_let_statement_scope() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cases = vec![
            CompileTestCase {
                input: r#"let num = 55;
fn() { num };
"#,
                expected_constants: vec![
                    testing_result!(Int, 55),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpGetGlobal, &v[0..1]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpSetGlobal, &v[0..1]),
                    make(&OpCode::OpClosure, &vec![1, 0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"fn() { let num = 55; num };"#,
                expected_constants: vec![
                    testing_result!(Int, 55),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            make(&OpCode::OpConstant, &v[0..1]),
                            make(&OpCode::OpSetLocal, &v[0..1]),
                            make(&OpCode::OpGetLocal, &v[0..1]),
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![1, 0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"fn() { let a = 55; let b = 77; a+b };"#,
                expected_constants: vec![
                    testing_result!(Int, 55),
                    testing_result!(Int, 77),
                    testing_result!(
                        CompiledFunction,
                        vec![
                            // a
                            make(&OpCode::OpConstant, &v[0..1]),
                            make(&OpCode::OpSetLocal, &v[0..1]),
                            // b
                            make(&OpCode::OpConstant, &v[1..2]),
                            make(&OpCode::OpSetLocal, &v[1..2]),
                            // a + b
                            make(&OpCode::OpGetLocal, &v[0..1]),
                            make(&OpCode::OpGetLocal, &v[1..2]),
                            make(&OpCode::OpAdd, &v[0..0]),
                            // return
                            make(&OpCode::OpReturnValue, &v[0..0]),
                        ]
                    ),
                ],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![2, 0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];
        run_compile_test(cases);
    }

    #[test]
    fn test_compiler_scopes() {
        let compiler = Compiler::new();
        assert_eq!(
            compiler.scope_index.get(),
            0,
            "scope_index wrong. got={}, wanted={}",
            compiler.scope_index.get(),
            0
        );

        // let g = compiler.symbol_table.as_ref();
        compiler.emit(OpCode::OpMul, &vec![]);

        compiler.enter_scope();
    }

    #[test]
    fn test_builtins() {
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let cases = vec![
            CompileTestCase {
                input: r#"len([]);
push([],1);
"#,
                expected_constants: vec![testing_result!(Int, 1)],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpGetBuiltin, &v[0..1]),
                    make(&OpCode::OpArray, &v[0..1]),
                    make(&OpCode::OpCall, &v[1..2]),
                    make(&OpCode::OpPop, &v[0..0]),
                    make(&OpCode::OpGetBuiltin, &v[5..6]),
                    make(&OpCode::OpArray, &v[0..1]),
                    make(&OpCode::OpConstant, &v[0..1]),
                    make(&OpCode::OpCall, &v[2..3]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
            CompileTestCase {
                input: r#"fn() { len([]) };"#,
                expected_constants: vec![testing_result!(
                    CompiledFunction,
                    vec![
                        make(&OpCode::OpGetBuiltin, &v[0..1]),
                        make(&OpCode::OpArray, &v[0..1]),
                        make(&OpCode::OpCall, &v[1..2]),
                        make(&OpCode::OpReturnValue, &v[0..0]),
                    ]
                )],
                expected_instruction: vec![
                    // 表示的是变量的 index
                    make(&OpCode::OpClosure, &vec![0, 0]),
                    make(&OpCode::OpPop, &v[0..0]),
                ],
            },
        ];
        run_compile_test(cases);
    }
}

#[cfg(test)]
mod symbol_table_test {

    use crate::{Symbol, SymbolTable, BUILTIN_SCOPE, GLOBAL_SCOPE, LOCAL_SCOPE};
    use ::ast::*;
    use std::rc::Rc;

    #[test]
    fn test_resolve_local() {
        let global = Rc::new(SymbolTable::new());
        let a = Rc::new(Identifier::from("a".to_string()));
        let b = Rc::new(Identifier::from("b".to_string()));
        global.define(a.clone());
        global.define(b.clone());

        let local = Rc::new(SymbolTable::new_enclosed(global.clone()));
        let c = Rc::new(Identifier::from("c".to_string()));
        let d = Rc::new(Identifier::from("d".to_string()));
        local.define(c.clone());
        local.define(d.clone());

        let tests = vec![
            (
                a.clone(),
                0,
                global.clone(),
                Symbol {
                    name: a.clone(),
                    scope: GLOBAL_SCOPE,
                    index: 0,
                },
            ),
            (
                b.clone(),
                1,
                global.clone(),
                Symbol {
                    name: b.clone(),
                    scope: GLOBAL_SCOPE,
                    index: 1,
                },
            ),
            (
                c.clone(),
                0,
                local.clone(),
                Symbol {
                    name: c.clone(),
                    scope: LOCAL_SCOPE,
                    index: 0,
                },
            ),
            (
                d.clone(),
                1,
                local.clone(),
                Symbol {
                    name: d.clone(),
                    scope: LOCAL_SCOPE,
                    index: 1,
                },
            ),
        ];

        tests
            .iter()
            .for_each(|(name, expected, symbol_table, symbol)| {
                let r = symbol_table.resolve(name.clone());
                assert!(r.is_ok(), "name {} not resolvable", name);
                let r = r.unwrap();
                assert_eq!(
                    *r, *symbol,
                    "expected {} to resolve to {:?}, got={:?}",
                    symbol.name, symbol, r
                );
                assert_eq!(
                    r.index, *expected,
                    "expected {} to resolve to {}, got={:?}",
                    name, expected, *r
                );
            });
    }

    #[test]
    fn test_nested_resolve_local() {
        let global = Rc::new(SymbolTable::new());
        let a = Rc::new(Identifier::from("a".to_string()));
        let b = Rc::new(Identifier::from("b".to_string()));
        global.define(a.clone());
        global.define(b.clone());

        let local = Rc::new(SymbolTable::new_enclosed(global.clone()));
        let c = Rc::new(Identifier::from("c".to_string()));
        let d = Rc::new(Identifier::from("d".to_string()));
        local.define(c.clone());
        local.define(d.clone());

        let local1 = Rc::new(SymbolTable::new_enclosed(local.clone()));
        let e = Rc::new(Identifier::from("e".to_string()));
        let f = Rc::new(Identifier::from("f".to_string()));
        local1.define(e.clone());
        local1.define(f.clone());

        let sa = Rc::new(Symbol {
            name: a.clone(),
            scope: GLOBAL_SCOPE,
            index: 0,
        });
        let sb = Rc::new(Symbol {
            name: b.clone(),
            scope: GLOBAL_SCOPE,
            index: 1,
        });
        let sc = Rc::new(Symbol {
            name: c.clone(),
            scope: LOCAL_SCOPE,
            index: 0,
        });
        let sd = Rc::new(Symbol {
            name: d.clone(),
            scope: LOCAL_SCOPE,
            index: 1,
        });
        let se = Rc::new(Symbol {
            name: e.clone(),
            scope: LOCAL_SCOPE,
            index: 0,
        });
        let sf = Rc::new(Symbol {
            name: f.clone(),
            scope: LOCAL_SCOPE,
            index: 1,
        });

        let tests = vec![
            (a.clone(), 0, global.clone(), sa.clone()),
            (b.clone(), 1, global.clone(), sb.clone()),
            (c.clone(), 0, local.clone(), sc.clone()),
            (d.clone(), 1, local.clone(), sd.clone()),
            (e.clone(), 0, local1.clone(), se.clone()),
            (f.clone(), 1, local1.clone(), sf.clone()),
        ];

        tests
            .iter()
            .for_each(|(name, expected, symbol_table, symbol)| {
                let r = symbol_table.resolve(name.clone());
                assert!(r.is_ok(), "name {} not resolvable", name);
                let r = r.unwrap();
                assert_eq!(
                    *r,
                    *symbol.clone(),
                    "expected {} to resolve to {:?}, got={:?}",
                    symbol.name,
                    symbol,
                    r
                );
                assert_eq!(
                    r.index, *expected,
                    "expected {} to resolve to {}, got={:?}",
                    name, expected, *r
                );
            });

        let tests = vec![
            (
                local.clone(),
                vec![sa.clone(), sb.clone(), sc.clone(), sd.clone()],
            ),
            (
                local1.clone(),
                vec![sa.clone(), sb.clone(), se.clone(), sf.clone()],
            ),
        ];
        tests.iter().for_each(|(symbol_table, symbols)| {
            symbols.iter().for_each(|symbol| {
                let r = symbol_table.resolve(symbol.name.clone());
                assert!(r.is_ok(), "name {} not resolvable", symbol.name.clone());
                let r = r.unwrap();
                assert_eq!(
                    *r,
                    *symbol.clone(),
                    "expected {} to resolve to {:?}, got={:?}",
                    symbol.name.clone(),
                    symbol,
                    *r
                );
            });
        });
    }
    #[test]
    fn test_define() {
        let global = Rc::new(SymbolTable::new());

        let local = Rc::new(SymbolTable::new_enclosed(global.clone()));

        let local1 = Rc::new(SymbolTable::new_enclosed(local.clone()));

        let a = Rc::new(Identifier::from("a".to_string()));
        let b = Rc::new(Identifier::from("b".to_string()));
        let c = Rc::new(Identifier::from("c".to_string()));
        let d = Rc::new(Identifier::from("d".to_string()));
        let e = Rc::new(Identifier::from("e".to_string()));
        let f = Rc::new(Identifier::from("f".to_string()));

        let sa = Rc::new(Symbol {
            name: a.clone(),
            scope: GLOBAL_SCOPE,
            index: 0,
        });
        let sb = Rc::new(Symbol {
            name: b.clone(),
            scope: GLOBAL_SCOPE,
            index: 1,
        });
        let sc = Rc::new(Symbol {
            name: c.clone(),
            scope: LOCAL_SCOPE,
            index: 0,
        });
        let sd = Rc::new(Symbol {
            name: d.clone(),
            scope: LOCAL_SCOPE,
            index: 1,
        });
        let se = Rc::new(Symbol {
            name: e.clone(),
            scope: LOCAL_SCOPE,
            index: 0,
        });
        let sf = Rc::new(Symbol {
            name: f.clone(),
            scope: LOCAL_SCOPE,
            index: 1,
        });

        let tests = vec![
            (a.clone(), 0, global.clone(), sa.clone()),
            (b.clone(), 1, global.clone(), sb.clone()),
            (c.clone(), 0, local.clone(), sc.clone()),
            (d.clone(), 1, local.clone(), sd.clone()),
            (e.clone(), 0, local1.clone(), se.clone()),
            (f.clone(), 1, local1.clone(), sf.clone()),
        ];

        tests
            .iter()
            .for_each(|(name, expected, symbol_table, symbol)| {
                let d = symbol_table.define(name.clone());
                assert_eq!(*symbol.clone(), *d, "expected c={:?}, got={:?}", symbol, d);
            });
    }

    #[test]
    fn test_define_resolve_builtins() {
        let global = Rc::new(SymbolTable::new());
        let first_local = Rc::new(SymbolTable::new_enclosed(global.clone()));
        let second_local = Rc::new(SymbolTable::new_enclosed(first_local.clone()));
        let expected = vec![
            Rc::new(Symbol {
                name: Rc::new(Identifier::from("a".to_string())),
                scope: BUILTIN_SCOPE,
                index: 0,
            }),
            Rc::new(Symbol {
                name: Rc::new(Identifier::from("b".to_string())),
                scope: BUILTIN_SCOPE,
                index: 1,
            }),
            Rc::new(Symbol {
                name: Rc::new(Identifier::from("c".to_string())),
                scope: BUILTIN_SCOPE,
                index: 2,
            }),
            Rc::new(Symbol {
                name: Rc::new(Identifier::from("d".to_string())),
                scope: BUILTIN_SCOPE,
                index: 3,
            }),
            Rc::new(Symbol {
                name: Rc::new(Identifier::from("e".to_string())),
                scope: BUILTIN_SCOPE,
                index: 4,
            }),
            Rc::new(Symbol {
                name: Rc::new(Identifier::from("f".to_string())),
                scope: BUILTIN_SCOPE,
                index: 5,
            }),
        ];
        expected.iter().enumerate().for_each(|(i, f)| {
            global.clone().define_builtin(i, f.name.clone());
        });
        vec![global, first_local, second_local]
            .iter()
            .for_each(|scope| {
                expected.iter().for_each(|sym| {
                    let r = scope.resolve(sym.name.clone());
                    assert!(r.is_ok(), "name {} not resolvable", sym.name.clone());
                    let r = r.unwrap();
                    assert_eq!(
                        *r,
                        *sym.clone(),
                        "expected {} to resolve to {:?}, got={:?}",
                        sym.name.clone(),
                        sym,
                        *r
                    );
                })
            });
    }
}

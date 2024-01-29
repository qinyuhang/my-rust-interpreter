#[cfg(test)]
mod test {
    use crate::*;
    use ::code::*;
    use ::lexer::*;
    use ::parser::*;
    use ::testing::*;
    use interpreter::testing_object::handle_object;
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
        let cases = vec![CompileTestCase {
            input: "if (true) { 10 }; 3333;",
            expected_constants: vec![testing_result!(Int, 10), testing_result!(Int, 3333)],
            expected_instruction: vec![
                // 0000
                make(&OpCode::OpTrue, &v[0..0]),
                // 0001
                make(&OpCode::OpJNT, &vec![7]),
                // 0004
                make(&OpCode::OpConstant, &v[0..1]),
                // 0007
                make(&OpCode::OpPop, &v[0..0]),
                // 0008
                make(&OpCode::OpConstant, &v[1..2]),
                // 0011
                make(&OpCode::OpPop, &v[0..0]),
            ],
        }];

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

                let compiler = Compiler::new();
                let r = compiler.compile(&pr.unwrap());
                assert!(r.is_ok(), "compile failed: {}", r.unwrap_err());
                let bytecode = compiler.bytecode();
                handle_instructions(
                    expected_instruction.clone(),
                    &*bytecode.instructions.borrow(),
                );

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
                    "constant {} testing failed: {}\n wanted={},\n got={}",
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
}

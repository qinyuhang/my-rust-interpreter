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
    fn lang_compiler_canary_test() {
        assert_eq!(1, 1);
        let cases = vec![CompileTestCase {
            input: "1 + 2",
            expected_constants: vec![testing_result!(Int, 1), testing_result!(Int, 2)],
            expected_instruction: vec![
                make(&OpCode::OpConstant, vec![0]),
                make(&OpCode::OpConstant, vec![1]),
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
                assert!(pr.is_some());

                let compiler = Compiler::new();
                let r = compiler.compile(&pr.unwrap());
                assert!(r.is_ok());
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
                    *ex, *ac,
                    "wrong instruction at {idx}.\nwanted={}\ngot={}",
                    *ex, *ac
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

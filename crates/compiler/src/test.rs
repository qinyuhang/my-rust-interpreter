#[cfg(test)]
mod test {
    use crate::*;
    use ::code::*;
    use ::lexer::*;
    use ::parser::*;

    struct CompileTestCase<'a> {
        pub input: &'a str,
        pub expected_constants: Vec<u8>,
        pub expected_instruction: Vec<Instructions>,
    }

    #[test]
    fn lang_compiler_canary_test() {
        assert_eq!(1, 1);
        let cases = vec![CompileTestCase {
            input: "1 + 2",
            expected_constants: vec![1, 2],
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
                let _bytecode = compiler.bytecode();
                // assert_eq!(bytecode.instructions, expected_instruction);

                // assert_eq!(bytecode.constants, expected_constants);
            },
        );
    }

    fn handle_instructions() {}

    fn handle_constants() {}

    fn concat_instructions(s: Vec<Instructions>) -> Instructions {
        s.into_iter().flatten().collect()
    }

    #[test]
    fn test_helper_concat_instructions() {
        let input = vec![0, 1, 2, 3, 4];
        let out = concat_instructions(vec![input.clone(), input.clone()]);
        assert_eq!(out, vec![0, 1, 2, 3, 4, 0, 1, 2, 3, 4]);
    }
    #[allow(unused)]
    fn handle_result() {}
}

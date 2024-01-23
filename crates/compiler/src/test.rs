#[cfg(test)]
mod test {
    use super::*;
    use crate::*;
    use ::code::*;
    use ::lexer::*;
    use ::parser::*;
    use interpreter::*;
    use std::rc::Rc;

    struct CompileTestCase<'a> {
        pub input: &'a str,
        pub expected_constants: Vec<u8>,
        pub expected_instruction: Vec<Instructions>,
    }

    #[test]
    fn lang_vm_canary_test() {
        assert_eq!(1, 1);
        let cases = vec![CompileTestCase {
            input: "1 + 2",
            expected_constants: vec![1, 2],
            expected_instruction: vec![
                make(&OpCode::OpConstant, vec![0]),
                make(&OpCode::OpConstant, vec![1]),
            ],
        }];

        runCompileTest(cases);
    }

    fn runCompileTest(tests: Vec<CompileTestCase>) {
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
                // assert_eq!(bytecode.instructions, expected_instruction);

                // assert_eq!(bytecode.constants, expected_constants);
            },
        );
    }

    #[allow(unused)]
    fn handle_result() {}
}

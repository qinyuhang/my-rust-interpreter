#[cfg(test)]
mod test {
    use crate::VM;
    use ::ast::Program;
    use ::compiler::*;
    use ::lexer::*;
    use ::parser::*;
    use ::testing::*;
    use interpreter::testing_object::*;

    fn parse(input: &str) -> Option<Program> {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        p.parse_program()
    }

    // FIXME: expected?
    fn run_vm_test(cases: &Vec<(/* input */ &str, /* expected */ TestingResult)>) {
        cases.iter().for_each(|(input, expected)| {
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
            assert!(r.is_ok(), "vm error: {}", r.unwrap_err());

            let stack_el = vm.stack_top();

            assert!(stack_el.is_some());

            let stack_el = stack_el.unwrap();

            handle_object(Some(stack_el), expected);
        });
    }

    #[test]
    fn test_integer_arithmetic() {
        let cases = vec![
            ("1", testing_result!(Int, 1)),
            ("2", testing_result!(Int, 2)),
            // ("1 + 2", testing_result!(Int, 3)),
        ];
        run_vm_test(&cases);
    }
}

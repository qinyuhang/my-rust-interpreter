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

            let stack_el = vm.last_popped_stack_el();

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
        ];
        run_vm_test(&cases);
    }
}

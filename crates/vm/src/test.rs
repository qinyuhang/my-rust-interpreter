#[cfg(test)]
mod test {
    use crate::VM;
    use ::ast::Program;
    use ::compiler::*;
    use ::lexer::*;
    use ::parser::*;
    use ::testing::*;

    fn parse(input: &str) -> Option<Program> {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        p.parse_program()
    }

    // FIXME: expected?
    fn run_vm_test(cases: &Vec<(/* input */ String, /* expected */ i32)>) {
        cases.iter().for_each(|(input, expected)| {
            let pr = parse(input).unwrap();
            let comp = Compiler::new();
            assert!(comp.compile(&pr).is_ok(), "fatal compiler error");

            let vm = VM::new(comp.bytecode());
        });
    }
}

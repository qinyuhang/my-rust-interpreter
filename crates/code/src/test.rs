#[cfg(test)]
mod test {
    use crate::*;
    use ::interpreter::testing_object::*;
    use ::testing::*;
    use interpreter::Object;

    #[test]
    fn lang_code_canary_test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_make() {
        let cases = vec![(
            OpCode::OpConstant,
            vec![65534],
            vec![OpCode::OpConstant as u8, 255, 254],
        )];
        cases.iter().for_each(|(op, operands, expected)| {
            let instruction = make(op, operands.clone());
            assert_eq!(instruction, expected.clone());
        });
    }

    #[test]
    fn test_concat_work() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        assert_eq!(concat_instructions(vec![a, b]), vec![1, 2, 3, 4])
    }

    fn handle_constants(expected: Vec<TestingResult>, actual: Vec<Rc<dyn Object>>) {
        assert_eq!(expected.len(), actual.len());
        expected
            .iter()
            .zip(actual.iter())
            .for_each(|(e, a)| match e {
                TestingResult::Int(v) => {
                    test_integer_object(Some(a.clone()), *v);
                }
                TestingResult::Bool(v) => {
                    test_boolean_object(Some(a.clone()), *v);
                }
                _ => {}
            })
    }

    fn concat_instructions(s: Vec<Instructions>) -> Instructions {
        s.iter()
            .fold(vec![], |acc, val| [acc, val.clone()].concat())
    }

    #[allow(unused)]
    fn handle_result() {}
}

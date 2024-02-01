#[cfg(test)]
mod test {
    use crate::*;
    // use ::interpreter::testing_object::*;
    // use ::testing::*;

    #[test]
    fn lang_code_canary_test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_make() {
        let cases = vec![
            (
                OpCode::OpConstant,
                vec![65534],
                vec![OpCode::OpConstant as u8, 255, 254],
            ),
            (OpCode::OpAdd, vec![], vec![OpCode::OpAdd as u8]),
        ];
        cases.iter().for_each(|(op, operands, expected)| {
            let instruction = make(op, &operands);
            assert_eq!(instruction, expected.clone());
        });
    }

    #[test]
    fn test_concat_work() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        assert_eq!(concat_instructions(vec![a, b]), vec![1, 2, 3, 4])
    }

    #[test]
    fn test_proper_format_instructions() {
        let v = vec![1, 2, 65535];
        let cases = vec![
            make(&OpCode::OpConstant, &v[0..1]),
            make(&OpCode::OpConstant, &v[1..2]),
            make(&OpCode::OpConstant, &v[2..3]),
        ];

        let expected = r#"0000 OpConstant 1
0003 OpConstant 2
0006 OpConstant 65535
"#;

        let c = concat_instructions(cases);
        assert_eq!(expected, format_display_instructions(&c));
    }

    #[test]
    fn test_proper_format_instructions_1() {
        let v = vec![2, 65535];
        let cases = vec![
            make(&OpCode::OpAdd, &v[0..0]),
            make(&OpCode::OpConstant, &v[0..1]),
            make(&OpCode::OpConstant, &v[1..2]),
        ];

        let expected = r#"0000 OpAdd
0001 OpConstant 2
0004 OpConstant 65535
"#;

        let c = concat_instructions(cases);
        assert_eq!(expected, format_display_instructions(&c));
    }

    #[test]
    fn test_read_operands() {
        let cases = vec![(OpCode::OpConstant, vec![65535], 2)];
        cases.iter().for_each(|(op, operands, bytes_read)| {
            let ins = make(op, &operands);
            let z = Definition::look_up(op);
            assert!(z.is_some(), "{}", format!("definition not found: {}", op));

            let (operands_read, n) = read_operands(z.unwrap(), &ins[1..]);
            assert_eq!(
                n,
                *bytes_read,
                "{}",
                format!("n wrong. want={}, got={}", bytes_read, n)
            );
            operands
                .iter()
                .zip(operands_read.iter())
                .for_each(|(ex, a)| {
                    assert_eq!(*ex, *a, "{}", format!(""));
                })
        });
    }
    // #[allow(dead_code)]
    // fn handle_operands() {}
    // #[allow(dead_code)]
    // fn handle_constants(expected: Vec<TestingResult>, actual: Vec<Rc<dyn Object>>) {
    //     assert_eq!(expected.len(), actual.len());
    //     expected
    //         .iter()
    //         .zip(actual.iter())
    //         .for_each(|(e, a)| match e {
    //             TestingResult::Int(v) => {
    //                 test_integer_object(Some(a.clone()), *v);
    //             }
    //             TestingResult::Bool(v) => {
    //                 test_boolean_object(Some(a.clone()), *v);
    //             }
    //             _ => {}
    //         })
    // }
    //
    // #[allow(dead_code)]
    // fn handle_instructions(expected: Vec<Instructions>, actual: Instructions) {
    //     let concat_instructions_result = concat_instructions(expected);
    //     assert_eq!(concat_instructions_result.len(), actual.len());
    //     concat_instructions_result
    //         .iter()
    //         .zip(actual.iter())
    //         .for_each(|(e, a)| {
    //             assert_eq!(*e, *a);
    //         })
    // }

    fn concat_instructions(s: Vec<Instructions>) -> Instructions {
        s.iter()
            .fold(vec![], |acc, val| [acc, val.clone()].concat())
    }

    #[allow(unused)]
    fn handle_result() {}
}

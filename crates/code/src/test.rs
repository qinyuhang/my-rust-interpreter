#[cfg(test)]
mod test {

    use crate::*;

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

    #[allow(unused)]
    fn handle_result() {}
}

use code::OpCode;

#[derive(Copy, Clone, Debug)]
pub struct EmittedInstruction {
    pub op_code: OpCode,
    pub position: usize,
}

impl Default for EmittedInstruction {
    fn default() -> Self {
        Self {
            position: 0,
            op_code: OpCode::OpConstant,
        }
    }
}

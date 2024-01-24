use byteorder::{BigEndian, ByteOrder};

use std::rc::Rc;

pub type Instructions = Vec<u8>;
// pub type OpCode = u8;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    OpConstant = 0u8,
    X,
}

pub struct Definition {
    pub name: String,
    pub operand_widths: Vec<u8>,
}

thread_local! {
    pub static DEFINITIONS: Vec<Rc<Definition>> = vec![
        Rc::new(Definition {
            name: "OpConst".into(),
            operand_widths: vec![2],
        }),
    ];
}

impl Definition {
    pub fn look_up(op: &OpCode) -> Option<Rc<Definition>> {
        DEFINITIONS.with(|definitions| definitions.get(*op as usize).cloned())
    }
}

// big end
// 65534 -> 0xff 0xfe
pub fn make(op: &OpCode, operands: Vec<u16>) -> Vec<u8> {
    let mut instruction = vec![];
    if let Some(definition) = Definition::look_up(op) {
        instruction.push(*op as u8);
        let mut instruction_len = 1;
        for width in definition.operand_widths.iter() {
            instruction_len += width;
        }

        for (i, operand) in operands.iter().enumerate() {
            let width = definition.operand_widths[i];
            match width {
                2 => {
                    let mut buf = [0u8; 2];
                    BigEndian::write_u16(&mut buf, *operand);
                    instruction.push(buf[0]);
                    instruction.push(buf[1]);
                }
                _ => {}
            }
        }
    }
    instruction
}

mod test;

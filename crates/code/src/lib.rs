use ast_macro::FromU8;
use byteorder::{BigEndian, ByteOrder};
use std::fmt::Formatter;

use std::rc::Rc;

pub type Instructions = Vec<u8>;
// pub type OpCode = u8;

// helper fns to display Instructions
pub fn format_display_instructions(instructions: &Instructions) -> String {
    let mut i = 0;
    let mut str = String::new();
    while i < instructions.len() {
        let def = Definition::look_up(&OpCode::from(instructions[i]));
        if def.is_none() {
            str.push_str("err");
            continue;
        }
        let def = def.unwrap();
        let (operands, read) = read_operands(def.clone(), &instructions[i + 1..]);
        str.push_str(&format!(
            "{:04} {}\n",
            i,
            format_one_instruction(def.clone(), &operands)
        ));
        i += 1 + (read as usize);
    }
    str
}

pub fn format_one_instruction(def: Rc<Definition>, operands: &Vec<u16>) -> String {
    let op_count = def.operand_widths.len();
    assert_eq!(op_count, operands.len(), "ERR");
    return match op_count {
        0 => format!("{}", def.name),
        1 => format!("{} {}", def.name, operands[0]),
        other => format!("unsupported format op_count {other}"),
    };
}

#[derive(Clone, Copy, Debug, FromU8, Eq, PartialEq)]
#[repr(u8)]
pub enum OpCode {
    OpConstant = 0u8,
    OpAdd,
    OpPop,

    OpSub, // 3
    OpMul,
    OpDiv, // 5

    OpTrue,  // 6
    OpFalse, // 7

    OpEqual, // 8
    OpNotEqual,
    OpGreaterThan, // 10

    // Why delete this? because we can rearrange x < y to x > y
    // to keep a minimal instruction, so delete this
    // OpLessThan, // 11
    OpMinus, // - 11
    OpBang,  // ! 12
    /// JumpNotTrue
    OpJNT,
    /// Jump
    OpJMP,
    OpNull, // 15
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

pub struct Definition {
    pub name: String,
    pub operand_widths: Vec<u8>,
}

thread_local! {
    pub static DEFINITIONS: Vec<Rc<Definition>> = vec![
        Rc::new(Definition {
            name: "OpConstant".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpAdd".into(),
            operand_widths: vec![],
        }),
        Rc::new(Definition {
            name: "OpPop".into(),
            operand_widths: vec![],
        }),
        Rc::new(Definition {
            name: "OpSub".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpMul".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpDiv".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpTrue".into(),
            operand_widths: vec![],
        }),
        Rc::new(Definition {
            name: "OpFalse".into(),
            operand_widths: vec![],
        }),
        Rc::new(Definition {
            name: "OpEqual".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpNotEqual".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpGreaterThan".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpMinus".into(),
            operand_widths: vec![],
        }),
        Rc::new(Definition {
            name: "OpBang".into(),
            operand_widths: vec![],
        }),
        Rc::new(Definition {
            name: "OpJNT".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpJMP".into(),
            operand_widths: vec![2],
        }),
        Rc::new(Definition {
            name: "OpNull".into(),
            operand_widths: vec![],
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
pub fn make(op: &OpCode, operands: &[u16]) -> Vec<u8> {
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

// slice to improve performance
pub fn read_operands(def: Rc<Definition>, ins: &[u8]) -> (/* operands */ Vec<u16>, /* read */ u8) {
    let mut operands = vec![0; def.operand_widths.len()];
    let mut offset = 0;
    for (i, &width) in def.operand_widths.iter().enumerate() {
        match width {
            2 => {
                operands[i] = read_uint16(&ins);
                offset += width;
            }
            _ => {}
        }
    }
    (operands, offset)
}

pub fn read_uint16(instructions: &[u8]) -> u16 {
    BigEndian::read_u16(instructions)
}

mod test;

mod symbol_table;
mod test;

use ::ast::*;
// use ::parser::*;
use crate::symbol_table::SymbolTable;
use ::object::*;
#[allow(unused)]
use byteorder::{BigEndian, ByteOrder};
use code::{
    self, format_display_instructions, format_one_instruction, make, read_operands, Definition,
    Instructions, OpCode,
};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct Compiler {
    instructions: RefCell<code::Instructions>,
    constants: RefCell<Vec<Rc<dyn Object>>>,

    last_instruction: Cell<EmittedInstruction>,
    previous_instruction: Cell<EmittedInstruction>,
    symbol_table: RefCell<SymbolTable>,
}

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

pub struct ByteCode {
    pub instructions: RefCell<code::Instructions>,
    pub constants: RefCell<Vec<Rc<dyn Object>>>,
}

thread_local! {
    static EMPTY_V16: Vec<u16> = vec![];
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: RefCell::new(vec![]),
            constants: RefCell::new(vec![]),

            last_instruction: Cell::new(EmittedInstruction::default()),
            previous_instruction: Cell::new(EmittedInstruction::default()),
            symbol_table: RefCell::new(SymbolTable::new()),
        }
    }

    pub fn compile(&self, node: &dyn Node) -> Result<(), String> {
        let n = node.as_any();
        if n.is::<Program>() {
            let n = n.downcast_ref::<Program>().unwrap();
            for st in n.statement.iter() {
                self.compile(st.get_expression().upcast())?;
            }
        }
        if n.is::<ExpressionStatement>() {
            if let Some(ExpressionStatement {
                expression: Some(bbq),
                ..
            }) = n.downcast_ref::<ExpressionStatement>()
            {
                let x = bbq.get_expression();
                let r = self.compile(x.upcast());
                EMPTY_V16.with(|v| self.emit(OpCode::OpPop, v));
                return r;
            }
        }
        if n.is::<InfixExpression>() {
            if let Some(InfixExpression {
                left: Some(left),
                right: Some(right),
                operator,
                ..
            }) = n.downcast_ref::<InfixExpression>()
            {
                let left = left.get_expression();
                let right = right.get_expression();
                // flip `x < y` to `y > x`
                if operator.as_str() == "<" {
                    self.compile(right.upcast())?;
                    self.compile(left.upcast())?;
                    EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpGreaterThan, v);
                    });
                    return Ok(());
                }
                self.compile(left.upcast())?;
                self.compile(right.upcast())?;
                match operator.as_str() {
                    "+" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpAdd, v);
                    }),
                    "-" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpSub, v);
                    }),
                    "*" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpMul, v);
                    }),
                    "/" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpDiv, v);
                    }),
                    ">" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpGreaterThan, v);
                    }),
                    "==" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpEqual, v);
                    }),
                    "!=" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpNotEqual, v);
                    }),
                    _ => return Err(format!("unknown operator: {}", operator)),
                }
            }
        }
        if n.is::<PrefixExpression>() {
            if let Some(PrefixExpression {
                right: Some(right),
                operator,
                ..
            }) = n.downcast_ref::<PrefixExpression>()
            {
                self.compile(right.upcast())?;
                match operator.as_str() {
                    "!" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpBang, v);
                    }),
                    "-" => EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpMinus, v);
                    }),
                    _ => return Err(format!("unknown operator {}", operator)),
                }
            };
        }
        if n.is::<IntegerLiteral>() {
            let i = n.downcast_ref::<IntegerLiteral>().unwrap();
            let i = Integer { value: i.value };
            self.emit(
                code::OpCode::OpConstant,
                &vec![self.add_constant(Rc::new(i)) as u16],
            );
        }
        if n.is::<BooleanLiteral>() {
            let i = n.downcast_ref::<BooleanLiteral>().unwrap();
            match i.value {
                true => EMPTY_V16.with(|v| {
                    self.emit(OpCode::OpTrue, v);
                }),
                false => EMPTY_V16.with(|v| {
                    self.emit(OpCode::OpFalse, v);
                }),
            };
        }
        if n.is::<IfExpression>() {
            // with alternative:
            //     condition
            //     jnt───────────────────┐
            //     consequence           │
            // ┌───jmp                   │
            // │   null / alternative◄───┘
            // └──►next
            let i = n.downcast_ref::<IfExpression>().unwrap();
            self.compile(i.condition.get_expression().upcast())?;

            // fake offset
            let jnt_position = self.emit(OpCode::OpJNT, &vec![9999]);

            if let Some(consequence) = &i.consequence {
                self.compile(consequence.get_expression().upcast())?;
            }
            if self.last_instruction_is_pop() {
                self.remove_last_pop();
            }

            let jmp_position = self.emit(OpCode::OpJMP, &vec![9999]);

            let after_consequence = self.instructions.borrow().len();
            self.change_operand(jnt_position, after_consequence)?;

            if let Some(alternative) = &i.alternative {
                self.compile(alternative.get_expression().upcast())?;
                if self.last_instruction_is_pop() {
                    self.remove_last_pop();
                }
            } else {
                EMPTY_V16.with(|v| self.emit(OpCode::OpNull, v));
            }
            let after_alternative = self.instructions.borrow().len();
            self.change_operand(jmp_position, after_alternative)?;

            // dbg!(after_consequence);
        }
        if n.is::<BlockStatement>() {
            let i = n.downcast_ref::<BlockStatement>().unwrap();
            for val in &i.statement {
                self.compile(val.get_expression().upcast())?;
            }
        }
        if n.is::<LetStatement>() {
            let i = n.downcast_ref::<LetStatement>().unwrap();
            self.compile(i.value.as_ref().unwrap().get_expression().upcast())?;
            // 这里和书不一样
            if self.last_instruction_is_pop() {
                self.remove_last_pop();
            }
            let symbol = self.symbol_table.borrow().define(i.name.clone());
            self.emit(OpCode::OpSetGlobal, &vec![symbol.index as u16]);
        }
        if n.is::<Identifier>() {
            let i = n.downcast_ref::<Identifier>().unwrap();
            // FIXME: DRAW BACK CLONE
            let symbol = self.symbol_table.borrow().resolve(Rc::new(i.clone()))?;
            self.emit(OpCode::OpGetGlobal, &vec![symbol.index as u16]);
        }
        // match _node.ty {  }
        Ok(())
    }

    fn emit(&self, op: OpCode, operands: &Vec<u16>) -> usize {
        let ins = make(&op, &operands[..]);
        let pos = self.add_instruction(&ins);
        self.set_last_instruction(op, pos);
        pos
    }

    fn set_last_instruction(&self, op: OpCode, pos: usize) {
        let prev = self.last_instruction.get();
        let last = EmittedInstruction {
            op_code: op,
            position: pos,
        };

        self.previous_instruction.set(prev);
        self.last_instruction.set(last);
    }

    fn add_instruction(&self, ins: &[u8]) -> usize {
        let pos_new_instruction = self.instructions.borrow().len();
        self.instructions.borrow_mut().extend_from_slice(ins);
        pos_new_instruction
    }

    fn add_constant(&self, obj: Rc<dyn Object>) -> usize {
        self.constants.borrow_mut().push(obj);
        self.constants.borrow().len() - 1
    }

    fn last_instruction_is_pop(&self) -> bool {
        self.last_instruction.get().op_code == OpCode::OpPop
    }

    fn remove_last_pop(&self) {
        let last_instruction_position = self.last_instruction.get().position;

        self.instructions
            .borrow_mut()
            .truncate(last_instruction_position);

        let previous_instruction = self.previous_instruction.get();
        self.last_instruction.set(previous_instruction);
    }

    fn replace_instruction(&self, pos: usize, n: &[u8]) {
        let mut i = 0;
        while i < n.len() {
            self.instructions.borrow_mut()[pos + i] = n[i];
            i += 1;
        }
    }

    fn change_operand(&self, op_pos: usize, operand: usize) -> Result<(), String> {
        let op = OpCode::from(*self.instructions.borrow_mut().get(op_pos).unwrap());

        // look up op to find the op_width
        // convert operand to Vec<u16> limited to op_width

        // hope this will guard the safety restriction
        // u16::Max = 65535
        // u32::Max = 4294967295
        // which means the program written in `monkey lang` should
        // be small enough to be compiled to bytecode
        // OR OTHERWISE WE MUST WIDER THE OPERAND OF OpJNT AND OpJMP
        // write usize to u16 array

        /// FIXME: operand is usize (consider it u64) it should be convert into u16 instead of `as u16`
        /// anyway it now works, later change the op_width of JNT JMP to 4, we need impl the convert
        let new_instruction = make(&op, &vec![operand as u16]);
        self.replace_instruction(op_pos, &new_instruction);
        Ok(())
    }

    pub fn bytecode(&self) -> Rc<ByteCode> {
        Rc::new(ByteCode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        })
    }

    pub fn dump_instruction(&self) -> String {
        let instructions: &Instructions = &*self.instructions.borrow();
        format_display_instructions(instructions)
    }
}

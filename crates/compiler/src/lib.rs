mod test;

use ::ast::*;
// use ::parser::*;
use ::object::*;
use code;
use code::{make, OpCode};
use interpreter::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Compiler {
    instructions: RefCell<code::Instructions>,
    constants: RefCell<Vec<Rc<dyn Object>>>,
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
        }
    }

    pub fn compile(&self, node: &dyn Node) -> Result<(), String> {
        let n = node.as_any();
        if n.is::<Program>() {
            let n = n.downcast_ref::<Program>().unwrap();
            for st in n.statement.iter() {
                match self.compile(st.get_expression().upcast()) {
                    Err(e) => return Err(e),
                    Ok(o) => {}
                }
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
                if operator == "<" {
                    if let Err(e) = self.compile(right.upcast()) {
                        return Err(e);
                    }
                    if let Err(e) = self.compile(left.upcast()) {
                        return Err(e);
                    }
                    EMPTY_V16.with(|v| {
                        self.emit(OpCode::OpGreaterThan, v);
                    });
                    return Ok(());
                }
                if let Err(e) = self.compile(left.upcast()) {
                    return Err(e);
                }
                if let Err(e) = self.compile(right.upcast()) {
                    return Err(e);
                }
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
        // match _node.ty {  }
        Ok(())
    }

    // TODO: maybe change to &Vec or slice to boost performance
    fn emit(&self, op: OpCode, operands: &Vec<u16>) -> usize {
        let ins = make(&op, &operands[..]);
        let pos = self.add_instruction(&ins);
        pos
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

    pub fn bytecode(&self) -> Rc<ByteCode> {
        Rc::new(ByteCode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        })
    }
}

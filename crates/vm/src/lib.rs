use code::{read_uint16, Instructions, OpCode};
use compiler::ByteCode;
use object::{ErrorObject, Null, Object};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

mod test;

pub const STACK_SIZE: usize = 2048usize;
pub struct VM {
    pub constants: RefCell<Vec<Rc<dyn Object>>>,
    pub instructions: RefCell<Instructions>,

    pub stack: RefCell<Vec<Rc<dyn Object>>>,
    // stack_pointer
    pub sp: Cell<usize>,
}

impl VM {
    pub fn new(byte_code: Rc<ByteCode>) -> Self {
        let ept = Rc::new(Null {});
        let stack = (0..STACK_SIZE)
            .map(|_| ept.clone() as Rc<dyn Object>)
            .collect();
        Self {
            constants: RefCell::new(byte_code.constants.borrow().clone()),
            instructions: RefCell::new(byte_code.instructions.borrow().clone()),
            stack: RefCell::new(stack),
            sp: Cell::new(0),
        }
    }

    // FIXME: type
    pub fn run(&self) -> Result<Rc<dyn Object>, String> {
        let mut ip = 0;
        while ip < self.instructions.borrow().len() {
            let ins = *self.instructions.borrow().get(ip).unwrap();
            let op = OpCode::from(ins);
            match op {
                OpCode::OpConstant => {
                    let const_index = read_uint16(&self.instructions.borrow()[ip + 1..]);
                    ip += 2;
                    assert!(
                        self.constants.borrow().get(const_index as usize).is_some(),
                        "expect can get constants from vm, vm.constants.len={}",
                        self.constants.borrow().len()
                    );
                    if let Some(c) = self.constants.borrow().get(const_index as usize) {
                        if let Err(e) = self.push(c.clone()) {
                            return Err(e);
                        }
                    }
                    dbg!(const_index);
                }
                _ => {
                    dbg!(op);
                }
            }

            ip += 1;
        }
        Ok(Rc::new(Null {}))
    }

    pub fn stack_top(&self) -> Option<Rc<dyn Object>> {
        match self.sp.get() {
            0 => Some(Rc::new(Null {})),
            other => self.stack.borrow().get(other - 1).cloned(),
        }
    }

    pub fn push(&self, o: Rc<dyn Object>) -> Result<(), String> {
        let sp = self.sp.get();
        if sp >= STACK_SIZE {
            return Err("stack overflow".into());
        }
        self.stack.borrow_mut()[sp] = o;
        self.sp.set(sp + 1);
        Ok(())
    }
}

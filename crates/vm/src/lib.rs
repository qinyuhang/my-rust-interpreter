use code::{read_uint16, Instructions, OpCode};
use compiler::ByteCode;
use interpreter::eval_infix_expression;
use object::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

mod test;

pub const STACK_SIZE: usize = 2048usize;

thread_local! {
    static TRUE: Rc<dyn Object> = Rc::new(Boolean { value : true });
    static FALSE: Rc<dyn Object> = Rc::new(Boolean { value : false });
}

pub struct VM {
    pub constants: RefCell<Vec<Rc<dyn Object>>>,
    pub instructions: RefCell<Instructions>,

    pub stack: RefCell<Vec<Rc<dyn Object>>>,
    // stack_pointer always point to the next empty stack of stack_top
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
                }
                OpCode::OpAdd | OpCode::OpSub | OpCode::OpMul | OpCode::OpDiv => {
                    self.execute_binary_operation(op)?;
                }
                OpCode::OpPop => {
                    self.pop()?;
                }
                OpCode::OpTrue => {
                    TRUE.with(|v| self.push(v.clone()))?;
                }
                OpCode::OpFalse => {
                    FALSE.with(|v| self.push(v.clone()))?;
                }
                OpCode::OpEqual | OpCode::OpNotEqual | OpCode::OpGreaterThan => {
                    self.execute_comparison(op)?;
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

    pub fn pop(&self) -> Result<Rc<dyn Object>, String> {
        let sp = self.sp.get();
        if sp < 0 {
            return Err(format!("stack pointer less then 0, got={sp}"));
        }
        let stack = self.stack.borrow();
        let r = stack.get(sp - 1).unwrap();
        self.sp.set(sp - 1);
        Ok(r.clone())
    }

    pub fn last_popped_stack_el(&self) -> Option<Rc<dyn Object>> {
        return self.stack.borrow().get(self.sp.get()).cloned();
    }

    fn execute_binary_operation(&self, op: OpCode) -> Result<(), String> {
        let right = self.pop()?;
        let left = self.pop()?;
        if left.object_type() == INTEGER_OBJECT && right.object_type() == INTEGER_OBJECT {
            let right = right.as_any().downcast_ref::<Integer>().unwrap();
            let left = left.as_any().downcast_ref::<Integer>().unwrap();
            return self.execute_int_binary_operation(op, left, right);
        }

        Ok(())
    }

    fn execute_comparison(&self, op: OpCode) -> Result<(), String> {
        let right = self.pop()?;
        let left = self.pop()?;

        if left.object_type() == INTEGER_OBJECT && left.object_type() == INTEGER_OBJECT {
            return self.execute_int_comparison(op, left.clone(), right.clone());
        }

        match op {
            OpCode::OpEqual => self.push(
                eval_infix_expression("==", Some(left.clone()), Some(right.clone())).unwrap(),
            ),
            OpCode::OpNotEqual => self.push(
                eval_infix_expression("!=", Some(left.clone()), Some(right.clone())).unwrap(),
            ),
            _ => Err(format!("unknown operator: {}", op)),
        }
    }

    fn execute_int_comparison(
        &self,
        op: OpCode,
        left: Rc<dyn Object>,
        right: Rc<dyn Object>,
    ) -> Result<(), String> {
        let l = left.as_any().downcast_ref::<Integer>().unwrap();
        let r = right.as_any().downcast_ref::<Integer>().unwrap();
        match op {
            OpCode::OpEqual => self.push(self.convert_rust_bool_to_bool_object(l.value == r.value)),
            OpCode::OpNotEqual => {
                self.push(self.convert_rust_bool_to_bool_object(l.value != r.value))
            }
            OpCode::OpGreaterThan => {
                self.push(self.convert_rust_bool_to_bool_object(l.value > r.value))
            }
            _ => Err(format!("unknown operator {}", op)),
        }
    }

    fn convert_rust_bool_to_bool_object(&self, v: bool) -> Rc<dyn Object> {
        if v {
            TRUE.with(|v| v.clone())
        } else {
            FALSE.with(|v| v.clone())
        }
    }

    fn execute_int_binary_operation(
        &self,
        op: OpCode,
        left: &Integer,
        right: &Integer,
    ) -> Result<(), String> {
        let value = match op {
            OpCode::OpAdd => left.value.wrapping_add(right.value),
            OpCode::OpSub => left.value.wrapping_sub(right.value),
            OpCode::OpMul => left.value.wrapping_mul(right.value),
            OpCode::OpDiv => left.value.wrapping_div(right.value),
            _ => panic!("Should never reach here"),
        };
        self.push(Rc::new(Integer { value }))
    }
}

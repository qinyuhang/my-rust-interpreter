use ast::WrapF64;
use code::{format_display_instructions, read_uint16, Instructions, OpCode};
use compiler::ByteCode;
use interpreter::*;
use object::*;
use std::cell::{Cell, RefCell};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::rc::Rc;

mod test;

pub const STACK_SIZE: usize = 2048usize;
pub const GLOBALS_SIZE: usize = 65536;

thread_local! {
    static TRUE: Rc<dyn Object> = Rc::new(Boolean { value : true });
    static FALSE: Rc<dyn Object> = Rc::new(Boolean { value : false });
    static NULL: Rc<dyn Object> = Rc::new(Null {});
}

pub struct VM<'a> {
    pub constants: RefCell<Vec<Rc<dyn Object>>>,
    pub instructions: RefCell<Instructions>,

    pub stack: RefCell<Vec<Rc<dyn Object>>>,
    // stack_pointer always point to the next empty stack of stack_top
    pub sp: Cell<usize>,
    globals: RefCell<Vec<Rc<dyn Object>>>,
    external_globals: RefCell<Option<&'a mut Vec<Rc<dyn Object>>>>,
}

impl<'a> VM<'a> {
    pub fn create_globals() -> Vec<Rc<dyn Object>> {
        let ept = Rc::new(Null {});
        (0..GLOBALS_SIZE)
            .map(|_| ept.clone() as Rc<dyn Object>)
            .collect()
    }
    pub fn new(byte_code: Rc<ByteCode>) -> Self {
        let ept = Rc::new(Null {});
        let stack = (0..STACK_SIZE)
            .map(|_| ept.clone() as Rc<dyn Object>)
            .collect();
        let globals = (0..GLOBALS_SIZE)
            .map(|_| ept.clone() as Rc<dyn Object>)
            .collect();
        Self {
            constants: RefCell::new(byte_code.constants.borrow().clone()),
            instructions: RefCell::new(byte_code.instructions.borrow().clone()),
            stack: RefCell::new(stack),
            sp: Cell::new(0),
            globals: RefCell::new(globals),
            external_globals: RefCell::new(None),
        }
    }

    /// IF want call this, must call before run
    pub fn load_external_globals(
        &self,
        external_globals: &'a mut Vec<Rc<dyn Object>>,
    ) -> Result<(), String> {
        if self.sp.get() != 0 {
            return Err(format!("call load external_globals before vm.run()"));
        }
        *self.external_globals.borrow_mut() = Some(external_globals);
        Ok(())
    }

    fn set_global(&self, index: usize, value: Rc<dyn Object>) {
        let has_external = self.external_globals.borrow().is_some();
        if has_external {
            self.external_globals.borrow_mut().as_mut().unwrap()[index] = value;
            return;
        }

        self.globals.borrow_mut()[index] = value;
    }

    fn get_global(&self, index: usize) -> Rc<dyn Object> {
        let has_external = self.external_globals.borrow().is_some();
        if has_external {
            return self.external_globals.borrow().as_ref().unwrap()[index].clone();
        }
        return self.globals.borrow()[index].clone();
    }

    // FIXME: type
    pub fn run(&self) -> Result<Rc<dyn Object>, String> {
        // ip 表示 [(操作符, 操作数), (操作符, 操作数)] 的一个 范围切片的位置
        // pos 表示[u8, u8] 的位置
        // 虽然都是 [u8] 但是看的颗粒度不一样
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
                OpCode::OpBang => {
                    self.execute_bang_operator()?;
                }
                OpCode::OpMinus => {
                    self.execute_minus_operator()?;
                }
                OpCode::OpJMP => {
                    let pos = read_uint16(&self.instructions.borrow()[ip + 1..]);
                    ip = (pos as usize) - 1;
                }
                OpCode::OpJNT => {
                    let pos = read_uint16(&self.instructions.borrow()[ip + 1..]);
                    ip += 2;

                    let condition = self.pop()?;
                    if !is_truthy(Some(condition)) {
                        ip = (pos as usize) - 1;
                    }
                }
                OpCode::OpNull => {
                    NULL.with(|v| self.push(v.clone()))?;
                }
                OpCode::OpSetGlobal => {
                    let global_index = read_uint16(&self.instructions.borrow()[ip + 1..]);
                    ip += 2;

                    self.set_global(global_index as usize, self.pop()?);
                }
                OpCode::OpGetGlobal => {
                    let global_index = read_uint16(&self.instructions.borrow()[ip + 1..]);
                    ip += 2;

                    self.push(self.get_global(global_index as usize))?;
                }
                OpCode::OpArray => {
                    let num_els = read_uint16(&self.instructions.borrow()[ip + 1..]) as usize;
                    ip += 2;

                    let arr = self.build_array(self.sp.get() - num_els, self.sp.get());
                    self.sp.set(self.sp.get() - num_els);

                    self.push(arr)?
                }
                OpCode::OpHash => {
                    let num_els = read_uint16(&self.instructions.borrow()[ip + 1..]) as usize;
                    ip += 2;

                    let hash = self.build_hash(self.sp.get() - num_els, self.sp.get());
                    self.sp.set(self.sp.get() - num_els);

                    self.push(hash?)?
                }
                OpCode::OpIndex => {
                    let index = self.pop()?;
                    let left = self.pop()?;
                    self.execute_index_expression(left, index)?;
                }
                #[allow(unreachable_patterns)]
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
        if sp <= 0 {
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
        if left.object_type() == STRING_OBJECT && right.object_type() == STRING_OBJECT {
            let right = right.as_any().downcast_ref::<StringObject>().unwrap();
            let left = left.as_any().downcast_ref::<StringObject>().unwrap();
            return self.execute_string_binary_operation(op, left, right);
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

    fn execute_string_binary_operation(
        &self,
        op: OpCode,
        left: &StringObject,
        right: &StringObject,
    ) -> Result<(), String> {
        let value = match op {
            OpCode::OpAdd => format!("{}{}", left.value, right.value), // left.value.wrapping_add(right.value),
            _ => panic!("Should never reach here"),
        };
        self.push(Rc::new(StringObject {
            value: Rc::new(value),
        }))
    }

    fn execute_bang_operator(&self) -> Result<(), String> {
        let operand = self.pop()?;
        self.push(self.convert_rust_bool_to_bool_object(!is_truthy(Some(operand))))?;
        Ok(())
    }

    fn execute_minus_operator(&self) -> Result<(), String> {
        let operand = self.pop()?;
        if operand.object_type() != INTEGER_OBJECT && operand.object_type() != FLOAT_OBJECT {
            return Err(format!(
                "unsupported type for negation: {}",
                operand.object_type()
            ));
        }
        let oa = operand.as_any();
        if oa.is::<Integer>() {
            let Integer { value } = oa.downcast_ref::<Integer>().unwrap();
            return self.push(Rc::new(Integer { value: -value }));
        };
        if oa.is::<FloatObject>() {
            let FloatObject { value } = oa.downcast_ref::<FloatObject>().unwrap();
            return self.push(Rc::new(FloatObject {
                value: WrapF64(-value.0),
            }));
        }
        Ok(())
    }

    fn build_array(&self, start_index: usize, end_index: usize) -> Rc<dyn Object> {
        let els = (start_index..end_index)
            .map(|idx| self.stack.borrow().get(idx).unwrap().clone())
            .collect();

        Rc::new(ArrayObject {
            elements: RefCell::new(els),
        })
    }

    fn build_hash(&self, start_index: usize, end_index: usize) -> Result<Rc<dyn Object>, String> {
        let x = (start_index..end_index)
            .enumerate()
            .filter(|(index, _)| index % 2 == 0)
            .map(|(_, val)| {
                let key = self.stack.borrow().get(val).unwrap().clone();
                let key = Rc::new(HashKey::try_from(key).unwrap());
                let value = self.stack.borrow().get(val + 1).unwrap().clone();
                (key, value)
            })
            .collect::<Vec<_>>();
        let hm: HashMap<Rc<object::HashKey>, Rc<dyn object::Object>, RandomState> =
            HashMap::from_iter(x);
        let ho = HashObject {
            pairs: RefCell::new(hm),
        };
        Ok(Rc::new(ho))
    }

    fn execute_index_expression(
        &self,
        left: Rc<dyn Object>,
        index: Rc<dyn Object>,
    ) -> Result<(), String> {
        if left.object_type() == ARRAY_OBJECT && index.object_type() == INTEGER_OBJECT {
            return self.execute_array_index_expression(left, index);
        }
        if left.object_type() == HASH_OBJECT {
            return self.execute_hash_index(left, index);
        }
        Err("".into())
    }

    fn execute_array_index_expression(
        &self,
        left: Rc<dyn Object>,
        index: Rc<dyn Object>,
    ) -> Result<(), String> {
        let arr = left.as_any().downcast_ref::<ArrayObject>().unwrap();
        let idx = index.as_any().downcast_ref::<Integer>().unwrap();
        let max = arr.elements.borrow().len() as i64 - 1;
        NULL.with(|shared_null| {
            if idx.value < 0 || idx.value > max {
                self.push(shared_null.clone())?
            }
            let arr = arr.elements.borrow();
            self.push(
                arr.get(idx.value as usize)
                    .map_or(shared_null.clone(), |vv| vv.clone()),
            )
        })
    }
    fn execute_hash_index(
        &self,
        left: Rc<dyn Object>,
        index: Rc<dyn Object>,
    ) -> Result<(), String> {
        let hm = left.as_any().downcast_ref::<HashObject>().unwrap();
        let key = HashKey::try_from(index)?;
        let pairs = hm.pairs.borrow();
        NULL.with(|shared_null| {
            self.push(
                pairs
                    .get(&Rc::new(key))
                    .map_or(shared_null.clone(), |vv| (vv.clone())),
            )
        })
    }

    pub fn dump_instruction(&self) -> String {
        let instruction = &*self.instructions.borrow();
        format_display_instructions(instruction)
    }
}

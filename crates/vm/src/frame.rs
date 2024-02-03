use code::*;
use object::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Frame {
    // do we need refcell?
    pub func: Rc<CompiledFunction>,
    pub ip: Cell<usize>,
    pub base_pointer: Cell<usize>,
}

impl Frame {
    pub fn new(func: Rc<CompiledFunction>, base_pointer: usize) -> Self {
        Self {
            func,
            ip: Cell::new(0),
            base_pointer: Cell::new(base_pointer),
        }
    }
    pub fn instruction(&self) -> Rc<Instructions> {
        self.func.instructions.clone()
    }

    pub fn bump_ip_by(&self, inc: usize) {
        let ip = self.ip.get();
        self.ip.set(ip + inc);
    }
}

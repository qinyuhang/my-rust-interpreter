use code::*;
use object::*;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Frame {
    // do we need refcell?
    pub func: Rc<CompiledFunction>,
    pub ip: Cell<usize>,
}

impl Frame {
    pub fn new(func: Rc<CompiledFunction>) -> Self {
        Self {
            func,
            ip: Cell::new(0),
        }
    }
    pub fn instruction(&self) -> Rc<Instructions> {
        self.func.instructions.clone()
    }
}

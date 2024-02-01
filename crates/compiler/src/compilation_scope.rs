use crate::EmittedInstruction;
use code::Instructions;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug)]
pub struct CompilationScope {
    pub(crate) instructions: Rc<RefCell<Instructions>>,
    pub(crate) last_instruction: Cell<EmittedInstruction>,
    pub(crate) previous_instruction: Cell<EmittedInstruction>,
}

impl CompilationScope {
    pub fn new() -> Self {
        Self {
            instructions: Rc::new(RefCell::new(vec![])),
            last_instruction: Cell::new(EmittedInstruction::default()),
            previous_instruction: Cell::new(EmittedInstruction::default()),
        }
    }
}

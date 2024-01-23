mod test;

use interpreter::object::*;
use lang_parser::Node;
use lang_vm::code;
use std::rc::Rc;

pub struct Compiler {
    instructions: code::Instructions,
    constants: Vec<Rc<dyn Object>>,
}

pub struct ByteCode {
    pub instructions: code::Instructions,
    pub constants: Vec<Rc<dyn Object>>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            instructions: vec![],
            constants: vec![],
        }
    }

    pub fn compile(&self, node: &dyn Node) -> Result<String, String> {
        Ok("".into())
    }

    pub fn bytecode(&self) -> Rc<ByteCode> {
        Rc::new(ByteCode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        })
    }
}

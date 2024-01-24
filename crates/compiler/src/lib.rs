mod test;

use ::ast::*;
// use ::parser::*;
use code;
use interpreter::*;
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

    pub fn compile(&self, _node: &dyn Node) -> Result<String, String> {
        Ok("".into())
    }

    pub fn bytecode(&self) -> Rc<ByteCode> {
        Rc::new(ByteCode {
            instructions: self.instructions.clone(),
            constants: self.constants.clone(),
        })
    }
}

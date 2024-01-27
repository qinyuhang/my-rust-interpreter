use compiler::ByteCode;
use std::rc::Rc;

mod test;

pub struct VM {}

impl VM {
    pub fn new(byte_code: Rc<ByteCode>) -> Self {
        Self {}
    }
}

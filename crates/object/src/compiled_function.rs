use crate::*;
use ast_macro::{object, object_with_try_from};
use code::{format_display_instructions, Instructions};
use std::rc::Rc;

#[object(COMPILED_FUNCTION)]
#[object_with_try_from(COMPILED_FUNCTION)]
#[derive(Hash, Eq, PartialEq)]
pub struct CompiledFunction {
    pub instructions: Rc<Instructions>,
    pub num_locals: usize,
}

impl ObjectInspect for CompiledFunction {
    fn _inspect(&self) -> String {
        format!("{}", format_display_instructions(&self.instructions))
    }
}

use crate::*;
use ast_macro::{object, object_with_try_from};
use std::rc::Rc;
use code::{Instructions,format_display_instructions};

#[object(COMPILED_FUNCTION)]
#[object_with_try_from(COMPILED_FUNCTION)]
#[derive(Hash, Eq, PartialEq)]
pub struct CompiledFunction {
  pub instructions: Rc<Instructions>,
}

impl ObjectInspect for CompiledFunction {
  fn _inspect(&self) -> String {
    format!("{}", format_display_instructions(&self.instructions))
  }
}

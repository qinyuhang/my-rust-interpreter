use crate::*;
use ast_macro::{object, object_with_try_from};
use std::cell::RefCell;
use std::rc::Rc;

#[object(CLOSURE_OBJECT)]
#[object_with_try_from(CLOSURE_OBJECT)]
pub struct ClosureObject {
    pub func: Rc<CompiledFunction>,
    pub free: Rc<RefCell<Vec<Rc<dyn Object>>>>,
}

impl ObjectInspect for ClosureObject {
    fn _inspect(&self) -> String {
        format!("Closure: {}", self)
    }
}

use crate::object::*;
use ast_macro::object;
use std::rc::Rc;

pub type BuiltinFunction = fn(args: Vec<Rc<dyn Object>>) -> Option<Rc<dyn Object>>;
#[object(BUILTIN_OBJECT)]
pub struct BuiltinObject {
    // function
    pub func: Rc<BuiltinFunction>,
}

impl ObjectInspect for BuiltinObject {
    fn _inspect(&self) -> String {
        "builtin function".into()
    }
}

use crate::object::*;
use ::ast::WrapF64;
use ast_macro::{object, object_with_try_from};
#[allow(unused)]
use std::rc::Rc;

#[object(FLOAT_OBJECT)]
#[object_with_try_from(FLOAT_OBJECT)]
#[derive(Hash, Eq, PartialEq)]
pub struct FloatObject {
    pub value: WrapF64,
}

impl ObjectInspect for FloatObject {
    fn _inspect(&self) -> String {
        self.value.to_string()
    }
}

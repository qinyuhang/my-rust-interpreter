use crate::object::*;
use ast_macro::{object, object_with_try_from};
#[allow(unused)]
use std::rc::Rc;

#[object(NULL_OBJECT)]
#[object_with_try_from(NULL_OBJECT)]
pub struct Null {}

impl ObjectInspect for Null {
    fn _inspect(&self) -> String {
        "null".to_string()
    }
}

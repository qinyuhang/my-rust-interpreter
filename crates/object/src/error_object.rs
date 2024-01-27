use crate::*;
use ast_macro::{object, object_with_try_from};
use std::rc::Rc;

#[object(ERROR_OBJECT)]
#[object_with_try_from(ERROR_OBJECT)]
pub struct ErrorObject {
    pub message: String,
}

impl ObjectInspect for ErrorObject {
    fn _inspect(&self) -> String {
        format!("Error: {}", self.message)
    }
}

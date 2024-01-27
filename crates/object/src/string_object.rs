use crate::*;
use ast_macro::{object, object_with_try_from};
use std::rc::Rc;

#[object(STRING_OBJECT)]
#[object_with_try_from(STRING_OBJECT)]
#[derive(Hash, Eq, PartialEq)]
pub struct StringObject {
    pub value: Rc<String>,
}

impl ObjectInspect for StringObject {
    fn _inspect(&self) -> String {
        self.value.as_ref().clone()
    }
}

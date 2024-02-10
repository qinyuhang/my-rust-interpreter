use crate::*;
use ast_macro::{object, object_with_try_from};
use std::rc::Rc;

#[object(RETURN_VALUE_OBJECT)]
#[object_with_try_from(RETURN_VALUE_OBJECT)]
pub struct ReturnValue {
    pub value: Rc<dyn Object>,
}

impl ObjectInspect for ReturnValue {
    fn _inspect(&self) -> String {
        self.value.inspect()
    }
}

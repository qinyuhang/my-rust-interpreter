pub use crate::object::*;
use ast_macro::object;
pub use std::rc::Rc;

#[object(RETURN_VALUE_OBJECT)]
pub struct ReturnValue {
    pub value: Rc<dyn Object>,
}

impl ObjectInspect for ReturnValue {
    fn _inspect(&self) -> String {
        self.value.inspect()
    }
}

impl TryFrom<Rc<dyn Object>> for ReturnValue {
    type Error = String;

    fn try_from(value: Rc<dyn Object>) -> Result<Self, Self::Error> {
        let val = value.as_any();
        if val.is::<ReturnValue>() {
            if let Some(v) = val.downcast_ref::<ReturnValue>() {
                return Ok((*v).clone());
            }
        }
        Err("Str".into())
    }
}

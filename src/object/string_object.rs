pub use crate::object::*;
use ast_macro::object;
pub use std::rc::Rc;

#[object(STRING_OBJECT)]
pub struct StringObject {
    pub value: Rc<String>,
}

impl ObjectInspect for StringObject {
    fn _inspect(&self) -> String {
        self.value.as_ref().clone()
    }
}

impl TryFrom<Rc<dyn Object>> for StringObject {
    type Error = String;

    fn try_from(value: Rc<dyn Object>) -> Result<Self, Self::Error> {
        let val = value.as_any();
        if val.is::<StringObject>() {
            if let Some(v) = val.downcast_ref::<StringObject>() {
                return Ok((*v).clone());
            }
        }
        Err("Str".into())
    }
}

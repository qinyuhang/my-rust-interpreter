pub use crate::object::*;
use ast_macro::object;
pub use std::rc::Rc;

#[object(INTEGER_OBJECT)]
#[derive(Hash, Eq, PartialEq)]
pub struct Integer {
    pub value: i64,
}

impl ObjectInspect for Integer {
    fn _inspect(&self) -> String {
        self.value.to_string()
    }
}
impl TryFrom<Rc<dyn Object>> for Integer {
    type Error = String;

    fn try_from(value: Rc<dyn Object>) -> Result<Self, Self::Error> {
        let val = value.as_any();
        if val.is::<Integer>() {
            if let Some(v) = val.downcast_ref::<Integer>() {
                return Ok((*v).clone());
            }
        }
        Err("Str".into())
    }
}

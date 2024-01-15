pub use crate::object::*;
use ast_macro::object;

#[object(BOOLEAN_OBJECT)]
pub struct Boolean {
    pub value: bool,
}

impl ObjectInspect for Boolean {
    fn _inspect(&self) -> String {
        format!("{}", self.value)
    }
}

impl TryFrom<Rc<dyn Object>> for Boolean {
    type Error = String;

    fn try_from(value: Rc<dyn Object>) -> Result<Self, Self::Error> {
        let val = value.as_any();
        if val.is::<Boolean>() {
            if let Some(v) = val.downcast_ref::<Boolean>() {
                return Ok((*v).clone());
            }
        }
        Err("Str".into())
    }
}

// use ast_macro::object;

pub use crate::object::*;
use ast_macro::object;

#[object(ERROR_OBJECT)]
pub struct ErrorObject {
    pub message: String,
}

impl ObjectInspect for ErrorObject {
    fn _inspect(&self) -> String {
        format!("Error: {}", self.message)
    }
}

impl std::fmt::Display for ErrorObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl TryFrom<Rc<dyn Object>> for ErrorObject {
    type Error = String;

    fn try_from(value: Rc<dyn Object>) -> Result<Self, Self::Error> {
        let val = value.as_any();
        if val.is::<ErrorObject>() {
            if let Some(v) = val.downcast_ref::<ErrorObject>() {
                return Ok((*v).clone());
            }
        }
        Err("Str".into())
    }
}

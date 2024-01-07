// use ast_macro::object;

pub use crate::object::*;

#[derive(Debug, Clone)]
pub struct ErrorObject {
    pub message: String,
}

impl Object for ErrorObject {
    fn object_type(&self) -> ObjectType {
        ERROR_OBJECT
    }

    fn inspect(&self) -> String {
        format!("Error: {}", self.message)
    }

    fn as_any(&self) -> &dyn Any {
        self
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

pub use crate::object::*;
pub use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct StringObject {
    pub value: Rc<String>,
}

impl Object for StringObject {
    fn object_type(&self) -> ObjectType {
        STRING_OBJECT
    }

    fn inspect(&self) -> String {
        self.value.as_ref().clone()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Display for StringObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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

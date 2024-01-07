pub use crate::object::*;
pub use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ReturnValue {
    pub value: Rc<dyn Object>,
}

impl Object for ReturnValue {
    fn object_type(&self) -> ObjectType {
        RETURN_VALUE_OBJECT
    }

    fn inspect(&self) -> String {
        self.value.inspect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Display for ReturnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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

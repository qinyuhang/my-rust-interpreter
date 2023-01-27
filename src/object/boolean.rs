pub use crate::object::*;

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool
}

impl Object for Boolean {
    fn object_type(&self) -> ObjectType {
        BOOLEAN_OBJECT
    }

    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
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
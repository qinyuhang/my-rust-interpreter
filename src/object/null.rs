use crate::object::*;

#[derive(Debug, Clone)]
pub struct Null {}

impl Object for Null {
    fn object_type(&self) -> ObjectType {
        NULL_OBJECT
    }

    fn inspect(&self) -> String {
        "null".to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "null")
    }
}

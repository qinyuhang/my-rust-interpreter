use crate::*;
use ast_macro::{object, object_with_try_from};

#[object(BOOLEAN_OBJECT)]
#[object_with_try_from(BOOLEAN_OBJECT)]
#[derive(Hash, Eq, PartialEq)]
pub struct Boolean {
    pub value: bool,
}

impl ObjectInspect for Boolean {
    fn _inspect(&self) -> String {
        format!("{}", self.value)
    }
}

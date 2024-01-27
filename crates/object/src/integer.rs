use crate::*;
use ast_macro::{object, object_with_try_from};
#[allow(unused)]
use std::rc::Rc;

#[object(INTEGER_OBJECT)]
#[object_with_try_from(INTEGER_OBJECT)]
#[derive(Hash, Eq, PartialEq)]
pub struct Integer {
    pub value: i64,
}

impl ObjectInspect for Integer {
    fn _inspect(&self) -> String {
        self.value.to_string()
    }
}

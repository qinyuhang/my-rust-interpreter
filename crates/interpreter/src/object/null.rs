use crate::object::*;
use ast_macro::object;

#[object(NULL_OBJECT)]
pub struct Null {}

impl ObjectInspect for Null {
    fn _inspect(&self) -> String {
        "null".to_string()
    }
}

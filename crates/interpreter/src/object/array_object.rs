use crate::object::*;
use ast_macro::{object, object_with_try_from};
use std::cell::RefCell;
use std::rc::Rc;

#[object(ARRAY_OBJECT)]
#[object_with_try_from(ARRAY_OBJECT)]
pub struct ArrayObject {
    pub elements: RefCell<Vec<Rc<dyn Object>>>,
}

impl ObjectInspect for ArrayObject {
    fn _inspect(&self) -> String {
        format!(
            "[{}]",
            self.elements
                .borrow()
                .iter()
                .map(|v| v.inspect())
                .collect::<String>()
        )
    }
}

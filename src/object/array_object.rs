pub use crate::object::*;
use ast_macro::object;
use std::cell::RefCell;

#[object(ARRAY_OBJECT)]
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

pub use crate::object::*;
use ast_macro::object;

#[object(ARRAY_OBJECT)]
pub struct ArrayObject {
    pub elements: Vec<Rc<dyn Object>>,
}

impl ObjectInspect for ArrayObject {
    fn _inspect(&self) -> String {
        format!(
            "[{}]",
            self.elements
                .iter()
                .map(|v| v.inspect())
                .collect::<String>()
        )
    }
}

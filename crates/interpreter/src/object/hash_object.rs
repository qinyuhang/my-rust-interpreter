use crate::object::*;
use ast_macro::object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[object(HASH_OBJECT)]
pub struct HashObject {
    pub pairs: RefCell<HashMap<Rc<HashKey>, Rc<dyn Object>>>,
}

impl ObjectInspect for HashObject {
    fn _inspect(&self) -> String {
        format!(
            "{{ {} }}",
            self.pairs
                .borrow()
                .iter()
                .map(|(k, v)| { format!("{}:{}", k, v) })
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl HashObject {
    pub fn insert() {}
    pub fn get() {}
}

#[cfg(test)]
mod test {}

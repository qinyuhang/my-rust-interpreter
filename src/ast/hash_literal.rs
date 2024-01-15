use crate::ast::*;
use crate::{Object, ObjectInspect, Token};
use ast_macro::*;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[ast_node(Expression)]
pub struct HashLiteral {
    pub token: Token,
    // due to HashMap cannot be Eq PartialEq Hash, so we are unable to put Rc<dyn Expression> to key
    pub pairs: RefCell<HashMap<Rc<String>, Rc<dyn Expression>>>,
}

impl std::fmt::Display for HashLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{{ {} }}",
                self.pairs
                    .borrow()
                    .iter()
                    .map(|(k, v)| { format!("{}:{}", k, v) })
                    .collect::<Vec<_>>()
                    .join(",")
            )
        )
    }
}

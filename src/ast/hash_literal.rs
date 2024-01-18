use crate::ast::*;
use crate::{Object, ObjectInspect, Token};
use ast_macro::*;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hasher;
use std::rc::Rc;

#[ast_node(Expression)]
pub struct HashLiteral {
    pub token: Token,
    // due to HashMap cannot be Eq PartialEq Hash, so we are unable to put Rc<dyn Expression> to key
    pub pairs: RefCell<HashMap<Rc<AstExpression>, Rc<AstExpression>>>,
}

impl std::hash::Hash for HashLiteral {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        todo!()
    }
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

use crate::ast::*;
use crate::token::Token;
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
        // 对 token 进行哈希化
        self.token.hash(state);

        // 获取 pairs 的借用
        let pairs = self.pairs.borrow();

        // 对键值对的数量进行哈希化，以确保不同数量的键值对产生不同的哈希值
        pairs.len().hash(state);

        // 对每个键值对进行迭代并进行哈希化
        for (key, value) in pairs.iter() {
            // 由于 Rc<T> 本身不能被哈希化，我们需要解引用并获取其内部值
            // 注意：这里假设 AstExpression 已经实现了 Hash
            (**key).hash(state);
            (**value).hash(state);
        }
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        data.iter().for_each(|s| HashLiteral::hash(s, state))
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

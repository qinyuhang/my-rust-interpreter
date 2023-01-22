use crate::ast::Expression;
use std::rc::Rc;

pub type PrefixParseFn = dyn Fn() -> Option<Rc<dyn Expression>>;
pub type InfixParseFn = dyn Fn(Rc<dyn Expression>) -> Option<Rc<dyn Expression>>;
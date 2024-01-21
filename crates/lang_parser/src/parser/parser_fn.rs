use crate::ast::Expression;
use crate::AstExpression;
use std::rc::Rc;

pub type PrefixParseFn = dyn Fn() -> Option<Rc<AstExpression>>;
pub type InfixParseFn = dyn Fn(/* left: */ Rc<AstExpression>) -> Option<Rc<AstExpression>>;

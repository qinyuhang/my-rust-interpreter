pub use ast_macro::ast_node;
use std::{
    any::Any,
    fmt::{Debug, Display},
};

mod test;

mod array_literal;
pub mod block_statement;
pub mod bool_literal;
pub mod call_expression;
pub mod expression_statement;
pub mod function_literal;
pub mod identifier;
pub mod if_expression;
mod index_expression;
pub mod infix_expression;
pub mod int_literal;
pub mod let_statement;
pub mod prefix_expression;
pub mod program;
pub mod return_statement;
mod string_literal;

pub use array_literal::*;
pub use block_statement::*;
pub use bool_literal::*;
pub use call_expression::*;
pub use expression_statement::*;
pub use function_literal::*;
pub use identifier::*;
pub use if_expression::*;
pub use index_expression::*;
pub use infix_expression::*;
pub use int_literal::*;
pub use let_statement::*;
pub use prefix_expression::*;
pub use program::*;
pub use return_statement::*;
pub use string_literal::*;

pub trait Node: Debug + Display {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {
    fn statement_node(&self);
    fn upcast(&self) -> &dyn Node;
}

pub trait Expression: Node {
    // node: Node;
    fn expression_node(&self);
    fn upcast(&self) -> &dyn Node;
}

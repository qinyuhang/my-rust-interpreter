use std::{any::Any, fmt::{Display, Debug}};

mod test;

pub mod identifier;
pub mod let_statement;
pub mod program;
pub mod return_statement;
pub mod expression_statement;
pub mod int_literal;
pub mod prefix_expression;
pub mod infix_expression;
pub mod bool_literal;
pub mod if_expression;
pub mod block_statement;
pub mod function_literal;

pub use identifier::*;
pub use let_statement::*;
pub use program::*;
pub use return_statement::*;
pub use expression_statement::*;
pub use int_literal::*;
pub use prefix_expression::*;
pub use infix_expression::*;
pub use bool_literal::*;
pub use if_expression::*;
pub use block_statement::*;
pub use function_literal::*;

pub trait Node: Debug + Display {
    fn token_literal(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node {
    fn statement_node(&self);
}

pub trait Expression: Node {
    // node: Node;
    fn expression_node(&self);
}

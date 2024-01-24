pub use ast_macro::ast_node;
use std::fmt::Formatter;
use std::{
    any::Any,
    fmt::{Debug, Display},
};

mod test;

mod array_literal;
mod block_statement;
mod bool_literal;
mod call_expression;
mod expression_statement;
mod function_literal;
mod hash_literal;
mod identifier;
mod if_expression;
mod index_expression;
mod infix_expression;
mod int_literal;
mod let_statement;
mod prefix_expression;
mod program;
mod return_statement;
mod string_literal;
mod while_loop_literal;

pub use array_literal::*;
pub use block_statement::*;
pub use bool_literal::*;
pub use call_expression::*;
pub use expression_statement::*;
pub use function_literal::*;
pub use hash_literal::*;
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
pub use while_loop_literal::*;

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

// #[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum AstStatement {
    BlockStatement(BlockStatement),
    ExpressionStatement(ExpressionStatement),
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
}
#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub enum AstExpression {
    ArrayLiteral(ArrayLiteral),
    BlockStatement(BlockStatement),
    BooleanLiteral(BooleanLiteral),
    CallExpression(CallExpression),
    ExpressionStatement(ExpressionStatement),
    FunctionLiteral(FunctionLiteral),
    HashLiteral(HashLiteral),
    Identifier(Identifier),
    IfExpression(IfExpression),
    IndexExpression(IndexExpression),
    InfixExpression(InfixExpression),
    IntegerLiteral(IntegerLiteral),
    LetStatement(LetStatement),
    PrefixExpression(PrefixExpression),
    // Program(Program),
    ReturnStatement(ReturnStatement),
    StringLiteral(StringLiteral),
}

impl std::fmt::Display for AstExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AstExpression::ArrayLiteral(a) => format!("{}", a),
                AstExpression::BlockStatement(a) => format!("{}", a),
                AstExpression::BooleanLiteral(a) => format!("{}", a),
                AstExpression::CallExpression(a) => format!("{}", a),
                AstExpression::ExpressionStatement(a) => format!("{}", a),
                AstExpression::FunctionLiteral(a) => format!("{}", a),
                AstExpression::HashLiteral(a) => format!("{}", a),
                AstExpression::Identifier(a) => format!("{}", a),
                AstExpression::IfExpression(a) => format!("{}", a),
                AstExpression::IndexExpression(a) => format!("{}", a),
                AstExpression::InfixExpression(a) => format!("{}", a),
                AstExpression::IntegerLiteral(a) => format!("{}", a),
                AstExpression::LetStatement(a) => format!("{}", a),
                AstExpression::PrefixExpression(a) => format!("{}", a),
                // AstExpression::Program(a)=> format!("{}", a),
                AstExpression::ReturnStatement(a) => format!("{}", a),
                AstExpression::StringLiteral(a) => format!("{}", a),
            }
        )
    }
}

impl AstExpression {
    pub fn as_any(&self) -> &dyn Any {
        match self {
            AstExpression::ArrayLiteral(a) => a,
            AstExpression::BlockStatement(a) => a,
            AstExpression::BooleanLiteral(a) => a,
            AstExpression::CallExpression(a) => a,
            AstExpression::ExpressionStatement(a) => a,
            AstExpression::FunctionLiteral(a) => a,
            AstExpression::HashLiteral(a) => a,
            AstExpression::Identifier(a) => a,
            AstExpression::IfExpression(a) => a,
            AstExpression::IndexExpression(a) => a,
            AstExpression::InfixExpression(a) => a,
            AstExpression::IntegerLiteral(a) => a,
            AstExpression::LetStatement(a) => a,
            AstExpression::PrefixExpression(a) => a,
            // AstExpression::Program(a)=> a,
            AstExpression::ReturnStatement(a) => a,
            AstExpression::StringLiteral(a) => a,
        }
    }

    pub fn get_expression(&self) -> &dyn Expression {
        match self {
            AstExpression::ArrayLiteral(a) => a,
            AstExpression::BlockStatement(a) => a,
            AstExpression::BooleanLiteral(a) => a,
            AstExpression::CallExpression(a) => a,
            AstExpression::ExpressionStatement(a) => a,
            AstExpression::FunctionLiteral(a) => a,
            AstExpression::HashLiteral(a) => a,
            AstExpression::Identifier(a) => a,
            AstExpression::IfExpression(a) => a,
            AstExpression::IndexExpression(a) => a,
            AstExpression::InfixExpression(a) => a,
            AstExpression::IntegerLiteral(a) => a,
            AstExpression::LetStatement(a) => a,
            AstExpression::PrefixExpression(a) => a,
            // AstExpression::Program(a)=> a,
            AstExpression::ReturnStatement(a) => a,
            AstExpression::StringLiteral(a) => a,
        }
    }

    pub fn upcast(&self) -> &dyn Node {
        match self {
            AstExpression::ArrayLiteral(a) => a,
            AstExpression::BlockStatement(a) => a,
            AstExpression::BooleanLiteral(a) => a,
            AstExpression::CallExpression(a) => a,
            AstExpression::ExpressionStatement(a) => a,
            AstExpression::FunctionLiteral(a) => a,
            AstExpression::HashLiteral(a) => a,
            AstExpression::Identifier(a) => a,
            AstExpression::IfExpression(a) => a,
            AstExpression::IndexExpression(a) => a,
            AstExpression::InfixExpression(a) => a,
            AstExpression::IntegerLiteral(a) => a,
            AstExpression::LetStatement(a) => a,
            AstExpression::PrefixExpression(a) => a,
            // AstExpression::Program(a)=> a,
            AstExpression::ReturnStatement(a) => a,
            AstExpression::StringLiteral(a) => a,
        }
    }
}

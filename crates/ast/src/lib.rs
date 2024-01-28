pub use ast_macro::{ast_node, ast_node_with_try_from, ForAstExpression};
use std::fmt::Formatter;
use std::{
    any::Any,
    fmt::{Debug, Display},
};

mod test;

mod array_literal;
mod assign_expression;
mod block_statement;
mod bool_literal;
mod break_expression;
mod call_expression;
mod expression_statement;
mod float_literal;
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
mod update_expression;
mod while_loop_literal;

pub use array_literal::*;
pub use assign_expression::*;
pub use block_statement::*;
pub use bool_literal::*;
pub use break_expression::*;
pub use call_expression::*;
pub use expression_statement::*;
pub use float_literal::*;
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
pub use update_expression::*;
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
#[derive(Eq, PartialEq, Hash, Debug, Clone, ForAstExpression)]
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
    FloatLiteral(FloatLiteral),
    LetStatement(LetStatement),
    PrefixExpression(PrefixExpression),
    // Program(Program),
    ReturnStatement(ReturnStatement),
    StringLiteral(StringLiteral),
    WhileLoopLiteral(WhileLoopLiteral),
    Break(Break),
    AssignExpression(AssignExpression),
    UpdateExpression(update_expression::UpdateExpression),
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
                AstExpression::FloatLiteral(a) => format!("{}", a),
                AstExpression::LetStatement(a) => format!("{}", a),
                AstExpression::PrefixExpression(a) => format!("{}", a),
                // AstExpression::Program(a)=> format!("{}", a),
                AstExpression::ReturnStatement(a) => format!("{}", a),
                AstExpression::StringLiteral(a) => format!("{}", a),
                AstExpression::WhileLoopLiteral(a) => format!("{}", a),
                AstExpression::Break(a) => format!("{}", a),
                AstExpression::AssignExpression(a) => format!("{}", a),
                AstExpression::UpdateExpression(a) => format!("{}", a),
            }
        )
    }
}

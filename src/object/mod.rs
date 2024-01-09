use std::{any::Any, fmt::Debug};
pub mod boolean;
pub mod context;
mod error_object;
pub mod function_object;
pub mod integer;
pub mod null;
pub mod return_value;

pub use boolean::*;
pub use context::*;
pub use error_object::*;
pub use function_object::*;
pub use integer::*;
pub use null::*;
pub use return_value::*;

pub type ObjectType = &'static str;

pub trait Object: Debug + std::fmt::Display {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub const BOOLEAN_OBJECT: &str = "BOOLEAN";
pub const INTEGER_OBJECT: &str = "INTEGER";
pub const NULL_OBJECT: &str = "NULL";
pub const RETURN_VALUE_OBJECT: &str = "RETURN_VALUE";
pub const ERROR_OBJECT: &str = "ERROR_OBJECT";
pub const FUNCTION_OBJECT: &str = "FUNCTION_OBJECT";

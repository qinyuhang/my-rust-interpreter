use std::{any::Any, fmt::Debug};
pub mod boolean;
pub mod integer;
pub mod null;
pub mod return_value;

pub use boolean::*;
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

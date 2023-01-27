use std::{any::Any, fmt::Debug};
pub mod boolean;
pub mod integer;

pub use boolean::*;
pub use integer::*;

pub type ObjectType = &'static str;

pub trait Object: Debug {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

pub const BOOLEAN_OBJECT: &str = "BOOLEAN";
pub const INTEGER_OBJECT: &str = "INTEGER";
pub const NULL_OBJECT: &str = "NULL";

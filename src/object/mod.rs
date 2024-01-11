use std::{any::Any, fmt::Debug};
pub mod boolean;
pub mod context;
mod error_object;
pub mod function_object;
pub mod integer;
pub mod null;
pub mod return_value;
mod string_object;

pub use boolean::*;
pub use context::*;
pub use error_object::*;
pub use function_object::*;
pub use integer::*;
pub use null::*;
pub use return_value::*;
pub use string_object::*;

pub type ObjectType = &'static str;

pub trait ObjectWithoutInspect {
    fn _object_type(&self) -> ObjectType;

    fn _as_any(&self) -> &dyn Any;
}

pub trait ObjectInspect {
    fn _inspect(&self) -> String;
}
pub trait Object: Debug + std::fmt::Display + ObjectInspect + ObjectWithoutInspect {
    fn object_type(&self) -> ObjectType {
        ObjectWithoutInspect::_object_type(self)
    }
    fn inspect(&self) -> String {
        ObjectInspect::_inspect(self)
    }
    fn as_any(&self) -> &dyn Any {
        ObjectWithoutInspect::_as_any(self)
    }
}

pub const BOOLEAN_OBJECT: &str = "BOOLEAN";
pub const INTEGER_OBJECT: &str = "INTEGER";
pub const NULL_OBJECT: &str = "NULL";
pub const RETURN_VALUE_OBJECT: &str = "RETURN_VALUE";
pub const ERROR_OBJECT: &str = "ERROR_OBJECT";
pub const FUNCTION_OBJECT: &str = "FUNCTION_OBJECT";
pub const STRING_OBJECT: &str = "STRING_OBJECT";

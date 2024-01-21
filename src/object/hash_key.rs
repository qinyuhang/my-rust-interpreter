use crate::*;
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum HashKey {
    Integer(Integer),
    Boolean(Boolean),
    StringObject(StringObject),
}

impl TryFrom<Rc<&dyn Object>> for HashKey {
    type Error = String;

    fn try_from(value: Rc<&dyn Object>) -> Result<Self, Self::Error> {
        let val = value.as_any();
        if val.is::<Integer>() {
            if let Some(v) = val.downcast_ref::<Integer>() {
                return Ok(HashKey::Integer((*v).clone()));
            }
        }
        if val.is::<Boolean>() {
            if let Some(v) = val.downcast_ref::<Boolean>() {
                return Ok(HashKey::Boolean((*v).clone()));
            }
        }
        if val.is::<StringObject>() {
            if let Some(v) = val.downcast_ref::<StringObject>() {
                return Ok(HashKey::StringObject((*v).clone()));
            }
        }
        Err("Str".into())
    }
}
impl TryFrom<Box<&dyn Object>> for HashKey {
    type Error = String;
    fn try_from(value: Box<&dyn Object>) -> Result<Self, Self::Error> {
        Self::try_from(Rc::new(*value))
    }
}
impl TryFrom<Rc<dyn Object>> for HashKey {
    type Error = String;

    fn try_from(value: Rc<dyn Object>) -> Result<Self, Self::Error> {
        Self::try_from(Rc::new(&*value))
    }
}
impl TryFrom<Box<dyn Object>> for HashKey {
    type Error = String;
    fn try_from(value: Box<dyn Object>) -> Result<Self, Self::Error> {
        Self::try_from(Rc::new(value.as_ref()))
    }
}

impl HashKey {}

impl std::fmt::Display for HashKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashKey::Integer(v) => write!(f, "{}", v),
            HashKey::Boolean(v) => write!(f, "{}", v),
            HashKey::StringObject(v) => write!(f, "{}", v),
        }
    }
}

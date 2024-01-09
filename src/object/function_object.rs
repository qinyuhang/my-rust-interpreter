use crate::object::*;
use crate::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct FunctionObject {
    pub context: Rc<Context>,

    // TODO change to RC
    pub parameters: Option<Vec<Rc<Identifier>>>,
    // blockStatement
    pub body: Option<Rc<dyn Statement>>,
    // pub body: Option<BlockStatement>,
}

impl Object for FunctionObject {
    fn object_type(&self) -> ObjectType {
        FUNCTION_OBJECT
    }

    fn inspect(&self) -> String {
        format!("fn ({}) {{ {} }}", 1, 2)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Display for FunctionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "")
    }
}

impl TryFrom<Rc<dyn Object>> for FunctionObject {
    type Error = String;

    fn try_from(value: Rc<dyn Object>) -> Result<Self, Self::Error> {
        // let val = value.as_any();
        // if val.is::<Integer>() {
        //     if let Some(v) = val.downcast_ref::<Integer>() {
        //         return Ok((*v).clone());
        //     }
        // }
        Err("Str".into())
    }
}

mod test {
    use crate::*;
}

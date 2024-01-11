use crate::object::*;
use crate::*;
use ast_macro::object;
use std::rc::Rc;

#[object(FUNCTION_OBJECT)]
pub struct FunctionObject {
    pub context: Rc<Context>,

    // TODO change to RC
    pub parameters: Option<Vec<Rc<Identifier>>>,
    // blockStatement
    pub body: Option<Rc<dyn Statement>>,
    // pub body: Option<BlockStatement>,
}

impl ObjectInspect for FunctionObject {
    fn _inspect(&self) -> String {
        format!("fn ({}) {{ {} }}", 1, 2)
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

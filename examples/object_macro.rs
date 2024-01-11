use ast_macro::object;
use my_rust_interpreter::*;
use std::any::Any;

#[object(BOOLEAN_OBJECT)]
struct A {}

impl ObjectInspect for A {
    fn _inspect(&self) -> String {
        "todo!()".into()
    }
}
impl std::fmt::Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "A {{ }}")
    }
}
fn main() {
    let a = A {};
    dbg!(a);
}

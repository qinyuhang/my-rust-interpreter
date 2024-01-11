use ast_macro::object;
use my_rust_interpreter::*;
use std::any::Any;

#[object(BOOLEAN_OBJECT)]
struct A {}

impl ObjectInspect for A {
    fn _inspect(&self) -> String {
        "A {{ }}".into()
    }
}

fn main() {
    let a = A {};
    dbg!(a);
}

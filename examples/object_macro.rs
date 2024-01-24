use my_rust_interpreter::*;

// #[object(BOOLEAN_OBJECT)]
#[derive(Hash, Debug)]
struct A {
    pub a: Vec<Rc<String>>,
}

pub trait AA {}
impl AA for A {}

impl A {
    pub fn default() -> Self {
        Self { a: vec![] }
    }
}

impl ObjectInspect for A {
    fn _inspect(&self) -> String {
        "A {{ }}".into()
    }
}

fn main() {
    let a = A::default();
    dbg!(a);
}

pub trait OBJ {}

#[derive(Hash, Eq, PartialEq, Default)]
struct A {
    pub value: i32,
}
#[derive(Eq, PartialEq, Default)]
struct B {
    pub value: bool,
}

impl OBJ for A {}
impl OBJ for B {}

fn main() {
    // let mut hm = HashMap::<Rc<dyn OBJ>, i32>::new();
    // let a = A::default();
    // let b = B::default();
    // hm.insert(Rc::new(a), 1);
    // hm.insert(Rc::new(b), 1);
}

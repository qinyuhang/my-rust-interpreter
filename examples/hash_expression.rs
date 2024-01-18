use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum EXXE {
    B(B),
}

pub trait EEX {}

#[derive(Debug)]
pub struct A {
    pub pair: RefCell<HashMap<Rc<EXXE>, Rc<EXXE>>>,
    pub els: RefCell<Vec<Rc<EXXE>>>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct B {
    pub m: String,
    // pub pair: RefCell<HashMap<Rc<EXXE>, Rc<EXXE>>>,
    // pub els: RefCell<Vec<Rc<String>>>,
}

impl EEX for B {}

impl B {
    pub fn default() -> Self {
        B { m: "".into() }
    }
}

fn main() {
    let m = HashMap::from([(
        Rc::new(EXXE::B(B::default())),
        Rc::new(EXXE::B(B::default())),
    )]);
    let x = Rc::new(EXXE::B(B::default())) == Rc::new(EXXE::B(B::default()));
    dbg!(x);
    // assert_eq!(Rc::new(EXXE::B( B ::default())), Rc::new(EXXE::B( B ::default())));
    let a = A {
        pair: RefCell::new(m),
        els: RefCell::new(vec![]),
    };
    dbg!(&a);
}

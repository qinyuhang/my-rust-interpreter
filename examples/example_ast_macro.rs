// use ::ast::*;
// use ::token::*;
// use ast_macro::*;
// use core::any::Any;
// use my_rust_interpreter::*;
// use std::collections::HashMap;
// use std::fmt::Formatter;
// use std::hash::Hasher;

// use ast::*;
// use ast_macro::*;

// struct A {}
// struct B {}

// #[derive(ForAstExpression)]
// enum XY {
//     A(A),
//     B(B),
// }

// #[ast_node(Expression)]
// struct A {
//     pub token: Token,
//     pub pairs: HashMap<Rc<dyn Expression>, Rc<dyn Expression>>,
// }

// #[ast_node(Expression)]
// pub struct HashLiteralA {
//     // pub token: Token,
//     // pub pairs: HashMap<Rc<dyn Expression>, Rc<dyn Expression>>,
//     // pub right: Option<Rc<dyn Expression>>,
// }

// impl std::fmt::Display for HashLiteralA {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "")
//     }
// }

// impl std::hash::Hash for HashLiteralA {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         todo!()
//     }
// }

fn main() {
    // let mut a = HashLiteralA {
    //     token: Token::default(),
    //     pairs: HashMap::new(),
    //     right: None,
    // };
    // let mut b = HashLiteralA {
    //     token: Token::default(),
    //     pairs: HashMap::new(),
    //     right: None,
    // };
    // let mut c = HashLiteralA {
    //     token: Token::default(),
    //     pairs: HashMap::from([(
    //         Rc::new(a) as Rc<dyn Expression>,
    //         Rc::new(b) as Rc<dyn Expression>,
    //     )]),
    //     right: None,
    // };
    // dbg!(a);
}

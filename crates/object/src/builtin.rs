use crate::*;
use ast_macro::object;
use std::cell::RefCell;
use std::rc::Rc;

// maybe Rc<Vec<Rc<dyn Object>>> is better
pub type BuiltinFunction = fn(args: Rc<Vec<Rc<dyn Object>>>) -> Option<Rc<dyn Object>>;
#[object(BUILTIN_OBJECT)]
pub struct BuiltinObject {
    // function
    pub func: Rc<BuiltinFunction>,
}

impl ObjectInspect for BuiltinObject {
    fn _inspect(&self) -> String {
        "builtin function".into()
    }
}

thread_local! {
    pub static NULLOBJ: Rc<dyn Object> = Rc::new(Null {});
    pub static TRUEOBJ: Rc<dyn Object> = Rc::new(Boolean { value: true });
    pub static FALSEOBJ:Rc<dyn Object> = Rc::new(Boolean { value: false });

    pub static BUILTINS:Rc<Vec<(&'static str, Rc<dyn Object>)>> = Rc::new(vec![
        (
            "len",
            Rc::new(BuiltinObject { func: Rc::new(|args: Rc<Vec<Rc<dyn Object>>>| {
                match args.as_slice() {
                    &[]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) })),
                    [_, _, ..]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) })),
                    [a] if a.as_ref().as_any().is::<StringObject>() => {
                        let inner_string = a.as_any().downcast_ref::<StringObject>().unwrap() ;
                        return Some(Rc::new(Integer { value: inner_string.value.to_string().len() as i64 }));
                    },
                    [a] if a.as_ref().as_any().is::<ArrayObject>() =>{
                        let inner = a.as_any().downcast_ref::<ArrayObject>().unwrap();
                        return Some(Rc::new(Integer { value: inner.elements.borrow().len() as i64}));
                    },
                    [a] => {
                        return Some(Rc::new(ErrorObject { message: format!( "argument to `len` not supported, got {}", a.object_type())}));
                    },
                }
            }) }) as Rc<dyn Object>
        ),
        (
            "puts",
            Rc::new(BuiltinObject { func: Rc::new(|args: Rc<Vec<Rc<dyn Object>>>| {
                for arg in args.iter() {
                    println!("{}", arg.inspect());
                }
                Some(Rc::new(Null {}))
            })}),
        ),
        (
            "first",
            Rc::new(BuiltinObject { func: Rc::new(|args: Rc<Vec<Rc<dyn Object>>>| {
                match args.as_slice() {
                    &[]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) })),
                    [_, _, ..]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) })),
                    [a] if a.as_ref().as_any().is::<ArrayObject>() => {
                        let inner = a.as_any().downcast_ref::<ArrayObject>().unwrap();
                        return Some(inner.elements.borrow().first().unwrap_or(&NULLOBJ.with(|n| n.clone())).clone());
                    },
                    [a] => Some(Rc::new(ErrorObject { message: format!("argument to `first` must be ARRAY, got {}", a.object_type())})),
                }
            })})
        ),
        (
            "last",
            Rc::new(BuiltinObject { func: Rc::new(|args: Rc<Vec<Rc<dyn Object>>>| {
                match args.as_slice() {
                    &[]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) })),
                    [_, _, ..]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) })),
                    [a] if a.as_ref().as_any().is::<ArrayObject>()  => {
                        let inner = a.as_any().downcast_ref::<ArrayObject>().unwrap();
                        return Some(inner.elements.borrow().last().unwrap_or(&NULLOBJ.with(|n| n.clone())).clone());
                    },
                    [a] => Some(Rc::new(ErrorObject { message: format!("argument to `last` must be ARRAY, got {}", a.object_type())}))
                }
            })})
        ),
        (
            // returns the other data in a new Array
            // etc: rest([1,2,3]) -> [2,3]
            "rest",
            Rc::new(BuiltinObject { func: Rc::new(|args: Rc<Vec<Rc<dyn Object>>>| {
                match args.as_slice() {
                    &[]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) })),
                    [_, _, ..]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) })),
                    [a] if a.as_ref().as_any().is::<ArrayObject>()  => {
                        let inner = a.as_any().downcast_ref::<ArrayObject>().unwrap();
                        let els = inner
                            .elements
                            .borrow()
                            .iter()
                            .enumerate()
                            .map(|(idx, val)| {
                                if idx > 0 {
                                    return Some(val.clone());
                                }
                                return None;
                            })
                            .filter(|val| val.is_some()).map(|v| v.unwrap())
                            .collect::<Vec<_>>();
                        return Some(Rc::new(ArrayObject {
                            elements: RefCell::new(els),
                        }));
                    },
                    [a] => Some(Rc::new(ErrorObject { message: format!("argument to `first` must be ARRAY, got {}", a.object_type())}))
                }
            })}),
        ),
        (
            // let a = [1,2,3]
            // push(a, 4);
            // a // -> [1,2,3,4]
            "push",
            Rc::new(BuiltinObject { func: Rc::new(|args: Rc<Vec<Rc<dyn Object>>>| {
                match args.as_slice() {
                    &[_] | &[]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=2", args.len()) })),
                    [_, _, _, ..]=> Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=2", args.len()) })),
                    [a, target] if a.as_ref().as_any().is::<ArrayObject>()  => {
                        let inner = a.as_any().downcast_ref::<ArrayObject>().unwrap();
                        let mut els = inner
                            .elements
                            .borrow()
                            .iter()
                            .map(|v| v.clone())
                            .collect::<Vec<Rc<dyn Object>>>();
                        els.push(target.clone());
                        return Some(Rc::new(ArrayObject {
                            elements: RefCell::new(els),
                        }));
                    },
                    [a, _, ..] => Some(Rc::new(ErrorObject { message: format!("argument[0] to `push` must be ARRAY, got {}", a.object_type())}))
                }
            })}),
        ),

    ]);
}

pub fn get_builtin_by_name<'a>(name: &'a str) -> Option<Rc<dyn Object>> {
    BUILTINS.with(|b| {
        b.as_ref()
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, o)| o.clone())
    })
}

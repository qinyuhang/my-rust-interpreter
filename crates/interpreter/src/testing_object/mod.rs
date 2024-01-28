use crate::*;
use ::testing::*;
use ast::Program;
use lexer::Lexer;
use object::*;
use parser::Parser;
use std::rc::Rc;

pub fn test_string_object(obj: Option<Rc<dyn Object>>, expected: String) -> bool {
    let i = StringObject::try_from(obj.unwrap());
    assert!(i.is_ok(), "expect=OK, got={}", i.unwrap_err());
    let i = i.unwrap();
    assert_eq!(*i.value, expected, "expect={}, got={}", expected, *i.value);
    true
}
pub fn test_error_object(object: Option<Rc<dyn Object>>, expected: String) -> bool {
    let err = ErrorObject::try_from(object.clone().unwrap());
    assert!(err.is_ok(), "{}", err.unwrap_err());
    // println!("{:?}", err.unwrap());
    assert_eq!(
        err.clone().unwrap().message,
        expected.to_string(),
        "expect={}, got={}",
        expected,
        err.clone().unwrap().message
    );
    true
}

pub fn test_integer_object(obj: Option<Rc<dyn Object>>, expected: i64) -> bool {
    // println!("test_integer_object {:?}", obj);
    let i = Integer::try_from(obj.unwrap());
    assert!(i.is_ok(), "expect=OK, got={}", i.unwrap_err());
    let i = i.unwrap();
    assert_eq!(i.value, expected, "expect={}, got={}", expected, i.value);
    true
}

pub fn test_float_object(obj: Option<Rc<dyn Object>>, expected: f64) -> bool {
    let i = FloatObject::try_from(obj.unwrap());
    assert!(i.is_ok());
    let i = i.unwrap();
    assert_eq!(
        i.value.0, expected,
        "expect={}, got={}",
        expected, i.value.0
    );
    true
}

pub fn test_boolean_object(obj: Option<Rc<dyn Object>>, expected: bool) -> bool {
    let i = Boolean::try_from(obj.unwrap());
    assert!(i.is_ok(), "expect=OK, got={}", i.unwrap_err());
    let i = i.unwrap();
    assert_eq!(i.value, expected);
    true
}

pub fn test_null_object(obj: &Option<Rc<dyn Object>>) {
    assert!(obj.is_some(), "expect=Some, got=None");
    // println!("test null object: {}", obj.as_ref().unwrap());
    let x = obj.as_ref().unwrap().as_any();
    assert!(x.downcast_ref::<Null>().is_some());
}

pub fn test_parse(input: &str) -> Option<Program> {
    let l = Lexer::new(input);
    let p = Parser::new(l);
    let pr = p.parse_program();
    return pr;
}

pub fn test_eval(input: &str) -> Option<Rc<dyn Object>> {
    let l = Lexer::new(input);
    let p = Parser::new(l);
    let pr = p.parse_program();
    assert!(pr.is_some());
    let pr = pr.unwrap();
    let context = Context::new();
    return eval(&pr, Rc::new(context));
}

pub fn handle_test_case(case: &str, out: &TestingResult) {
    let input = case;
    let evaluated = test_eval(input);
    assert!(
        evaluated.is_some(),
        "expect=Some, got=None, input={}",
        input
    );
    dbg!(&evaluated);
    handle_object(evaluated, out);
}

pub fn handle_object(evaluated: Option<Rc<dyn Object>>, out: &TestingResult) {
    match out {
        TestingResult::STRING(s) => {
            test_string_object(evaluated, s.to_string());
        }
        TestingResult::Int(i) => {
            test_integer_object(evaluated, *i);
        }
        TestingResult::Float(f) => {
            test_float_object(evaluated, *f);
        }
        TestingResult::Bool(b) => {
            test_boolean_object(evaluated, *b);
        }
        TestingResult::Vec(v) => {
            v.iter()
                .zip(
                    evaluated
                        .unwrap()
                        .as_any()
                        .downcast_ref::<ArrayObject>()
                        .unwrap()
                        .elements
                        .clone()
                        .borrow()
                        .iter(),
                )
                .for_each(|(expected, ev)| {
                    test_integer_object(Some(ev.clone()), *expected);
                });
        }
        TestingResult::Err(st) => {
            test_error_object(evaluated, st.to_string());
            // convert to ErrorObject
            // let err = ErrorObject::try_from(evaluated.clone().unwrap());
            // assert!(err.is_ok());
            // // println!("{:?}", err.unwrap());
            // assert_eq!(err.unwrap().message, st.to_string());
        }
        TestingResult::Hash(h) => {}
        TestingResult::Nil => {
            test_null_object(&evaluated);
        }
        _ => assert!(false),
    }
}

use std::collections::HashMap;
use std::rc::Rc;

pub enum TestingResult {
    STRING(String),
    Int(i64),
    Bool(bool),
    Vec(Vec<i64>),
    Err(String),
    Nil,
    Hash(HashMap<Rc<String>, Box<TestingResult>>),
}

#[macro_export]
macro_rules! testing_result {
    (String, $e:expr) => {
        TestingResult::String($e.to_string())
    };
    (Int, $e:expr) => {
        TestingResult::Int($e)
    };
    (Bool, $e:expr) => {
        TestingResult::Bool($e)
    };
    (Vec, $e:expr) => {
        TestingResult::Vec($e)
    };
    (Err, $e:expr) => {
        TestingResult::Err($e.to_string())
    };
    (Nil) => {
        TestingResult::Nil
    };
    (Hash, $e:expr) => {
        TestingResult::Hash($e)
    };
}

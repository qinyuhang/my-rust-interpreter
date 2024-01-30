use std::collections::HashMap;
use std::fmt::Formatter;
use std::rc::Rc;

pub enum TestingResult {
    STRING(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Vec(Vec<i64>),
    Err(String),
    Nil,
    Hash(HashMap<Rc<String>, Box<TestingResult>>),
}

impl std::fmt::Display for TestingResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestingResult::Bool(b) => write!(f, "{}", b),
            TestingResult::Err(e) => write!(f, "Error: {e}"),
            TestingResult::Hash(_h) => write!(f, "HashMap"),
            TestingResult::Float(ff) => write!(f, "{ff}"),
            TestingResult::Int(a) => write!(f, "{a}"),
            TestingResult::STRING(a) => write!(f, "{a}"),
            TestingResult::Vec(a) => write!(f, "{:?}", a),
            TestingResult::Nil => write!(f, "Nil"),
        }
    }
}

#[macro_export]
macro_rules! testing_result {
    (String, $e:expr) => {
        TestingResult::String($e.to_string())
    };
    (Int, $e:expr) => {
        TestingResult::Int($e)
    };
    (Float, $e:expr) => {
        TestingResult::Float($e)
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
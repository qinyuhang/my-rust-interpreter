use ast::AstExpression;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::rc::Rc;

pub enum TestingResult {
    STRING(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Vec(Vec<i64>),
    VecInstruction(Vec<Vec<u8>>),
    Err(String),
    Nil,
    // FIXME: 改成正确的key type
    Hash(HashMap<Rc<AstExpression>, TestingResult>),
    CompiledFunction(Vec<Vec<u8>>),
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
            TestingResult::VecInstruction(a) => write!(f, "{:?}", a),
            TestingResult::CompiledFunction(a) => write!(f, "{:?}", a),
            TestingResult::Nil => write!(f, "Nil"),
        }
    }
}

#[macro_export]
macro_rules! testing_result {
    (String, $e:expr) => {
        TestingResult::STRING($e.to_string())
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
    (VecInstruction, $e:expr) => {
        TestingResult::VecInstruction($e)
    };
    (CompiledFunction, $e:expr) => {
        TestingResult::CompiledFunction($e)
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

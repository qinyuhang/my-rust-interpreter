pub use crate::ast::*;
pub use crate::lexer::*;
pub use crate::object::*;
pub use crate::parser::*;
pub use std::rc::Rc;

pub fn eval(node: &dyn Node) -> Option<Rc<dyn Object>> {
    let n = node.as_any();
    // println!("eval: {:?}", node);
    // Program
    // ExpressionStatement
    // IntegerLiteral
    if n.is::<Program>() {
        if let Some(n) = n.downcast_ref::<Program>() {
            return eval_statements(n.statement.clone());
        }
    }
    if n.is::<ExpressionStatement>() {
        if let Some(n) = n.downcast_ref::<ExpressionStatement>() {
            return eval(n.expression.as_ref().unwrap().upcast());
        }
    }
    if n.is::<IntegerLiteral>() {
        if let Some(n) = n.downcast_ref::<IntegerLiteral>() {
            return Some(Rc::new(Integer { value: n.value }));
        }
    }
    None
}

// fn e(node: &dyn Node) {}

pub fn eval_statements(stmts: Vec<Rc<dyn Statement>>) -> Option<Rc<dyn Object>> {
    let mut result = None;
    stmts.iter().for_each(|st| {
        // converter Statement to Node
        // rust not support convert sub-trait-object to parent-trait-object
        // so here using a upcast function to convert Statement/Expression to Node trait
        result = eval(st.upcast());
    });
    result
}

mod test {
    #[allow(unused)]
    use {crate::evaluator::*, crate::lexer::*, crate::object::*, crate::parser::*};

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![("5", 5), ("10", 10)];

        tests.iter().for_each(|&(input, expected)| {
            let evaluated = test_eval(input);
            assert!(test_integer_object(evaluated, expected));
        });
    }

    fn test_eval(input: &str) -> Option<Rc<dyn Object>> {
        let l = Lexer::new(input);
        let p = Parser::new(l);
        let pr = p.parse_program();
        assert!(pr.is_some());
        let pr = pr.unwrap();
        return eval(&pr);
    }

    fn test_integer_object(obj: Option<Rc<dyn Object>>, expected: i64) -> bool {
        println!("test_integer_object {:?}", obj);
        let i = Integer::try_from(obj.unwrap());
        assert!(i.is_ok());
        let i = i.unwrap();
        assert_eq!(i.value, expected);
        true
    }
}

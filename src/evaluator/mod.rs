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
    if n.is::<BooleanLiteral>() {
        if let Some(n) = n.downcast_ref::<BooleanLiteral>() {
            return Some(Rc::new(Boolean { value: n.value }));
        }
    }
    // null if ident
    // if n.is::<>() {
    //     if let Some(n) = n.downcast_ref::<Null>() {
    //         return Some(Rc::new(Null {}));
    //     }
    // }
    if n.is::<PrefixExpression>() {
        if let Some(n) = n.downcast_ref::<PrefixExpression>() {
            let right = eval(n.right.as_ref().unwrap().upcast());
            return eval_prefix_expression(&n.operator, right);
        }
    }
    None
}

pub fn eval_prefix_expression(
    operator: &str,
    right: Option<Rc<dyn Object>>,
) -> Option<Rc<dyn Object>> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        _ => None,
    }
}

pub fn eval_bang_operator_expression(right: Option<Rc<dyn Object>>) -> Option<Rc<dyn Object>> {
    if let Some(right) = right {
        let v_any = right.as_any();
        if v_any.is::<Boolean>() {
            if let Some(v) = v_any.downcast_ref::<Boolean>() {
                return Some(Rc::new(Boolean { value: !v.value }));
            }
        }
        if v_any.is::<Null>() {
            return Some(Rc::new(Boolean { value: true }));
        }
        return Some(Rc::new(Boolean { value: false }));
    }
    Some(Rc::new(Boolean { value: false }))
}

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

    #[test]
    fn test_boolean_expression() {
        let tests = vec![("true", true), ("false", false)];

        tests.iter().for_each(|&(input, expected)| {
            let evaluated = test_eval(input);
            assert!(test_boolean_object(evaluated, expected));
        });
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
            // ("!null", true),
        ];
        tests.iter().for_each(|&(input, expected)| {
            let evaluated = test_eval(input);
            assert!(test_boolean_object(evaluated, expected));
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

    fn test_boolean_object(obj: Option<Rc<dyn Object>>, expected: bool) -> bool {
        let i = Boolean::try_from(obj.unwrap());
        assert!(i.is_ok());
        let i = i.unwrap();
        assert_eq!(i.value, expected);
        true
    }
}

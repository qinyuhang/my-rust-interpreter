pub use crate::ast::*;
pub use crate::lexer::*;
pub use crate::object::*;
pub use crate::parser::*;
pub use std::rc::Rc;

mod test;

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
    if n.is::<InfixExpression>() {
        if let Some(n) = n.downcast_ref::<InfixExpression>() {
            let left = eval(n.left.as_ref().unwrap().upcast());
            let right = eval(n.right.as_ref().unwrap().upcast());
            return eval_infix_expression(&n.operator, left, right);
        }
    }
    None
}

pub fn eval_infix_expression(
    operator: &str,
    left: Option<Rc<dyn Object>>,
    right: Option<Rc<dyn Object>>,
) -> Option<Rc<dyn Object>> {
    match (left.as_ref(), right.as_ref()) {
        (Some(l), Some(r))
            if (left.as_ref().unwrap().as_any()).is::<Integer>()
                && (right.as_ref().unwrap().as_any()).is::<Integer>() =>
        {
            let l = l.as_any().downcast_ref::<Integer>().unwrap();
            let r = r.as_any().downcast_ref::<Integer>().unwrap();
            if let Some(val) = match operator {
                "+" => Some(l.value + r.value),
                "-" => Some(l.value - r.value),
                "*" => Some(l.value * r.value),
                "/" => Some(l.value / r.value),
                _ => None,
            } {
                return Some(Rc::new(Integer { value: val }));
            }
            None
        }
        _ => None,
    }
}

pub fn eval_prefix_expression(
    operator: &str,
    right: Option<Rc<dyn Object>>,
) -> Option<Rc<dyn Object>> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Some(Rc::new(Null {})),
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

pub fn eval_minus_prefix_operator_expression(
    right: Option<Rc<dyn Object>>,
) -> Option<Rc<dyn Object>> {
    if let Some(right) = right {
        if right.object_type() == INTEGER_OBJECT {
            return Some(Rc::new(Integer {
                value: -Integer::try_from(right).unwrap().value,
            }));
        }
    }
    Some(Rc::new(Null {}))
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

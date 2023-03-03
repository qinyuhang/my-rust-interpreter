pub use crate::ast::*;
pub use crate::lexer::*;
pub use crate::object::*;
pub use crate::parser::*;
pub use std::rc::Rc;

mod test;

thread_local! {
    pub static NULLOBJ: Rc<dyn Object> = Rc::new(Null {});
    pub static TRUEOBJ: Rc<dyn Object> = Rc::new(Boolean { value: true });
    pub static FALSEOBJ:Rc<dyn Object> = Rc::new(Boolean { value: false });
}

pub fn eval(node: &dyn Node) -> Option<Rc<dyn Object>> {
    let n = node.as_any();
    // println!("eval: {:?}", node);
    // Program
    // ExpressionStatement
    // IntegerLiteral
    println!("eval: {}", node);
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
            return Some(if n.value {
                TRUEOBJ.with(|val| val.clone())
            } else {
                FALSEOBJ.with(|val| val.clone())
            });
        }
    }
    if n.is::<IfExpression>() {
        if let Some(n) = n.downcast_ref::<IfExpression>() {
            println!("IfExpression {:?}", n);
            return eval_if_expression(n);
            // return Some(Rc::new(If));
        }
    }
    // null is ident
    // if n.is::<>() {
    //     if let Some(n) = n.downcast_ref::<Null>() {
    //         return Some(NULLOBJ.with(|val| val.clone()));
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
    if n.is::<BlockStatement>() {
        println!("eval block Statement");
        if let Some(n) = n.downcast_ref::<BlockStatement>() {
            return eval_statements(n.statement.clone());
        }
    }
    if n.is::<ReturnStatement>() {
        if let Some(n) = n.downcast_ref::<ReturnStatement>() {
            if n.return_value.is_some() {
                if let Some(value) = eval(n.return_value.as_ref().unwrap().upcast()) {
                    return Some(Rc::new(ReturnValue { value }));
                }
            }
        }
    }
    None
}

pub fn eval_if_expression(ex: &IfExpression) -> Option<Rc<dyn Object>> {
    if is_truthy(eval(ex.condition.upcast())) {
        return eval(ex.consequence.as_ref().unwrap().upcast());
    } else if ex.alternative.is_some() {
        return eval(ex.alternative.as_ref().unwrap().upcast());
    } else {
        return Some(NULLOBJ.with(|val| val.clone()));
    }
}

pub fn is_truthy(obj: Option<Rc<dyn Object>>) -> bool {
    println!("is_truthy: {:?}", obj);
    obj.map_or(false, |val| {
        let v_a = val.as_any();
        if v_a.is::<Null>() {
            return false;
        }
        if v_a.is::<Boolean>() {
            return v_a.downcast_ref::<Boolean>().unwrap().value;
        }
        true
    })
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
            // Some(Rc::new(Integer { value: val }))
            match operator {
                "+" => Some(Rc::new(Integer {
                    value: l.value + r.value,
                })),
                "-" => Some(Rc::new(Integer {
                    value: l.value - r.value,
                })),
                "*" => Some(Rc::new(Integer {
                    value: l.value * r.value,
                })),
                "/" => Some(Rc::new(Integer {
                    value: l.value / r.value,
                })),
                "<" => Some(if l.value < r.value {
                    TRUEOBJ.with(|val| val.clone())
                } else {
                    FALSEOBJ.with(|val| val.clone())
                }),
                ">" => Some(if l.value > r.value {
                    TRUEOBJ.with(|val| val.clone())
                } else {
                    FALSEOBJ.with(|val| val.clone())
                }),
                "==" => Some(if l.value == r.value {
                    TRUEOBJ.with(|val| val.clone())
                } else {
                    FALSEOBJ.with(|val| val.clone())
                }),
                "!=" => Some(if l.value != r.value {
                    TRUEOBJ.with(|val| val.clone())
                } else {
                    FALSEOBJ.with(|val| val.clone())
                }),
                _ => None,
            }
        }
        (Some(l), Some(r))
            if (left.as_ref().unwrap().as_any()).is::<Boolean>()
                && (right.as_ref().unwrap().as_any()).is::<Boolean>() =>
        {
            let l = l.as_any().downcast_ref::<Boolean>().unwrap();
            let r = r.as_any().downcast_ref::<Boolean>().unwrap();
            // Some(Rc::new(Integer { value: val }))
            match operator {
                "==" => Some(if l.value == r.value {
                    TRUEOBJ.with(|val| val.clone())
                } else {
                    FALSEOBJ.with(|val| val.clone())
                }),
                "!=" => Some(if l.value != r.value {
                    TRUEOBJ.with(|val| val.clone())
                } else {
                    FALSEOBJ.with(|val| val.clone())
                }),
                _ => None,
            }
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
        _ => Some(NULLOBJ.with(|val| val.clone())),
    }
}

pub fn eval_bang_operator_expression(right: Option<Rc<dyn Object>>) -> Option<Rc<dyn Object>> {
    if let Some(right) = right {
        let v_any = right.as_any();
        if v_any.is::<Boolean>() {
            if let Some(v) = v_any.downcast_ref::<Boolean>() {
                return Some(if !v.value {
                    TRUEOBJ.with(|val| val.clone())
                } else {
                    FALSEOBJ.with(|val| val.clone())
                });
            }
        }
        if v_any.is::<Null>() {
            return Some(TRUEOBJ.with(|val| val.clone()));
        }
        return Some(FALSEOBJ.with(|val| val.clone()));
    }
    Some(FALSEOBJ.with(|val| val.clone()))
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
    Some(NULLOBJ.with(|val| val.clone()))
}

pub fn eval_statements(stmts: Vec<Rc<dyn Statement>>) -> Option<Rc<dyn Object>> {
    let mut result = None;
    for st in stmts.iter() {
        // converter Statement to Node
        // rust not support convert sub-trait-object to parent-trait-object
        // so here using a upcast function to convert Statement/Expression to Node trait
        result = eval(st.upcast());
        // if
        if let Some(result) = result.as_ref(){
            if result.as_any().is::<ReturnValue>() {
                return Some(result
                    .as_any()
                    .downcast_ref::<ReturnValue>()
                    .unwrap()
                    .value
                    .clone());
            }
        }
    }
    result
}

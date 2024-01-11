pub use crate::ast::*;
pub use crate::lexer::*;
pub use crate::object::*;
pub use crate::parser::*;
pub use crate::token::*;
use std::collections::HashMap;
pub use std::rc::Rc;
use std::vec::Vec;

mod test;

thread_local! {
    pub static NULLOBJ: Rc<dyn Object> = Rc::new(Null {});
    pub static TRUEOBJ: Rc<dyn Object> = Rc::new(Boolean { value: true });
    pub static FALSEOBJ:Rc<dyn Object> = Rc::new(Boolean { value: false });

    pub static BUILTINS:Rc<HashMap<&'static str, Rc<dyn Object>>> = Rc::new([
        (
            "len",
            Rc::new(BuiltinObject { func: Rc::new(|args: Vec<Rc<dyn Object>>| {
                match args.as_slice() {
                    [a] => {
                        if let Some(inner_string) = a.as_any().downcast_ref::<StringObject>() {
                            return Some(Rc::new(Integer { value: inner_string.value.to_string().len() as i64 }));
                        }
                        return Some(Rc::new(ErrorObject { message: format!( "argument to `len` not supported, got {}", a.object_type())}));
                    },
                    _ => {
                        Some(Rc::new(ErrorObject { message: format!("wrong number of arguments. got={}, want=1", args.len()) }))
                    }
                }
            }) }) as Rc<dyn Object>
        ),
    ].iter().cloned().collect::<HashMap<&'static str, Rc<dyn Object>>>()); // Rc::new(HashMap::new());
}

pub fn eval(node: &dyn Node, context: Rc<Context>) -> Option<Rc<dyn Object>> {
    let n = node.as_any();
    // println!("eval: {:?}", node);
    // Program
    // ExpressionStatement
    // IntegerLiteral
    println!("eval: {}", node);
    if n.is::<Program>() {
        if let Some(n) = n.downcast_ref::<Program>() {
            return eval_program(n.statement.clone());
        }
    }
    if n.is::<ExpressionStatement>() {
        if let Some(n) = n.downcast_ref::<ExpressionStatement>() {
            // println!("ExpressionStatement {:?}", n);
            return eval(n.expression.as_ref().unwrap().upcast(), context.clone());
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
            // println!("IfExpression {:?}", n);
            return eval_if_expression(n, context.clone());
            // return Some(Rc::new(If));
        }
    }
    // null is ident
    if n.is::<Identifier>() {
        if let Some(n) = n.downcast_ref::<Identifier>() {
            // println!("W====================== {}", n.token);
            if let Some(val) = context.get(&Rc::new(n.clone())) {
                return Some(val);
            }

            let idf = &n.value;

            let hm = BUILTINS.with(|hm| hm.clone());
            if let Some(val) = hm.get(idf.as_str()) {
                return Some(val.clone());
            }

            return Some(Rc::new(ErrorObject {
                message: format!("identifier not found: {}", n),
            }));
        }
        // if let Some(n) = n.downcast_ref::<Null>() {
        //     // return Some(NULLOBJ.with(|val| val.clone()));
        //     return eval_identifier(n, context);
        // }
    }
    if n.is::<LetStatement>() {
        if let Some(n) = n.downcast_ref::<LetStatement>() {
            if let Some(val) = n.value.as_ref() {
                let result = eval(val.upcast(), context.clone());
                if let Some(r) = result.as_ref() {
                    if r.as_any().is::<ErrorObject>() {
                        return result;
                    }
                    context.set(n.name.clone(), r.clone());
                }

                // if r is error, return
            }
        }
    }
    if n.is::<PrefixExpression>() {
        if let Some(n) = n.downcast_ref::<PrefixExpression>() {
            let right = eval(n.right.as_ref().unwrap().upcast(), context.clone());
            return eval_prefix_expression(&n.operator, right);
        }
    }
    if n.is::<InfixExpression>() {
        if let Some(n) = n.downcast_ref::<InfixExpression>() {
            let left = eval(n.left.as_ref().unwrap().upcast(), context.clone());
            let right = eval(n.right.as_ref().unwrap().upcast(), context.clone());
            return eval_infix_expression(&n.operator, left, right);
        }
    }
    if n.is::<BlockStatement>() {
        println!("eval block Statement");
        if let Some(n) = n.downcast_ref::<BlockStatement>() {
            return eval_block_statement(n.clone(), context.clone());
        }
    }
    if n.is::<ReturnStatement>() {
        if let Some(n) = n.downcast_ref::<ReturnStatement>() {
            if n.return_value.is_some() {
                if let Some(value) =
                    eval(n.return_value.as_ref().unwrap().upcast(), context.clone())
                {
                    return Some(Rc::new(ReturnValue { value }));
                }
            }
        }
    }
    if n.is::<FunctionLiteral>() {
        if let Some(n) = n.downcast_ref::<FunctionLiteral>() {
            let function = Rc::new(FunctionObject {
                parameters: n.parameters.clone(),
                body: n.body.clone(),
                context: context.clone(),
            });
            if let Some(ref name) = n.name {
                context.set(name.clone(), function.clone());
            }
            return Some(function.clone());
        }
    }
    if n.is::<CallExpression>() {
        if let Some(n) = n.downcast_ref::<CallExpression>() {
            if let Some(ref f) = n.function {
                // get the function from context;
                if let Some(r) = eval(f.as_ref().upcast(), context.clone()) {
                    if r.as_any().is::<ErrorObject>() {
                        return Some(r);
                    }
                    let args =
                        eval_expressions(n.arguments.as_ref().unwrap_or(&vec![]), context.clone());

                    // 错误处理
                    if args.iter().any(|item| item.is_none()) {
                        return Some(Rc::new(ErrorObject {
                            message: "Err".into(),
                        }));
                    }
                    let args = args.iter().map(|item| item.clone().unwrap()).collect();
                    return apply_function(r, args);
                }
            }
        }
    }
    if n.is::<StringLiteral>() {
        if let Some(f) = n.downcast_ref::<StringLiteral>() {
            return Some(Rc::new(StringObject {
                value: f.value.clone(),
            }));
        }
    }
    None
}

pub fn apply_function(func: Rc<dyn Object>, args: Vec<Rc<dyn Object>>) -> Option<Rc<dyn Object>> {
    if let Some(f) = func.as_any().downcast_ref::<FunctionObject>() {
        let extended_context = extend_function_context(f, &args);
        if let Some(ref body) = f.body {
            let evaluated = eval(body.as_ref().upcast(), extended_context);
            return evaluated;
        }
    }
    if let Some(f) = func.as_any().downcast_ref::<BuiltinObject>() {
        return (f.func)(args.clone());
    }
    None
}

//
pub fn extend_function_context(func: &FunctionObject, args: &Vec<Rc<dyn Object>>) -> Rc<Context> {
    let context = Context::extend(func.context.clone());
    // func.parameters
    if let Some(ref pr) = func.parameters {
        pr.iter()
            .zip(args.iter())
            .for_each(|(id, ob)| context.set(id.clone(), ob.clone()));
    }
    Rc::new(context)
}

pub fn eval_expressions(
    exps: &Vec<Rc<dyn Expression>>,
    context: Rc<Context>,
) -> Vec<Option<Rc<dyn Object>>> {
    exps.iter()
        .map(|exp| eval(exp.upcast(), context.clone()))
        .collect()
}

pub fn eval_if_expression(ex: &IfExpression, context: Rc<Context>) -> Option<Rc<dyn Object>> {
    if is_truthy(eval(ex.condition.upcast(), context.clone())) {
        return eval(ex.consequence.as_ref().unwrap().upcast(), context.clone());
    } else if ex.alternative.is_some() {
        return eval(ex.alternative.as_ref().unwrap().upcast(), context.clone());
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
                _ => Some(Rc::new(ErrorObject {
                    message: format!(
                        "unknown operator: {} {} {}",
                        l.object_type(),
                        operator,
                        r.object_type()
                    ),
                })),
            }
        }
        (Some(l), Some(r))
            if (left.as_ref().unwrap().as_any()).is::<StringObject>()
                && (right.as_ref().unwrap().as_any()).is::<StringObject>() =>
        {
            let l = l.as_any().downcast_ref::<StringObject>().unwrap();
            let r = r.as_any().downcast_ref::<StringObject>().unwrap();
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
                "+" => Some(Rc::new(StringObject {
                    value: Rc::new(format!("{}{}", l.value, r.value)),
                })),
                _ => Some(Rc::new(ErrorObject {
                    message: format!(
                        "unknown operator: {} {} {}",
                        l.object_type(),
                        operator,
                        r.object_type()
                    ),
                })),
            }
        }
        (Some(a), Some(b)) => Some(Rc::new(ErrorObject {
            message: format!(
                "type mismatch: {} {} {}",
                a.object_type(),
                operator,
                b.object_type()
            ),
        })),
        _ => Some(Rc::new(ErrorObject {
            message: format!("{:?} {} {:?}", left.as_ref(), operator, right.as_ref()),
        })),
    }
}

pub fn eval_prefix_expression(
    operator: &str,
    right: Option<Rc<dyn Object>>,
) -> Option<Rc<dyn Object>> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Some(Rc::new(ErrorObject { message: "".into() })),
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
        return Some(Rc::new(ErrorObject {
            message: format!("unknown operator: -{}", right.object_type()),
        }));
    }
    Some(Rc::new(ErrorObject {
        message: "unknown operator: -".into(),
    }))
}

pub fn eval_program(stmts: Vec<Rc<dyn Statement>>) -> Option<Rc<dyn Object>> {
    let mut result = None;
    let context = Rc::new(Context::new());
    // add builtin functions to context
    for st in stmts.iter() {
        // converter Statement to Node
        // rust not support convert sub-trait-object to parent-trait-object
        // so here using a upcast function to convert Statement/Expression to Node trait
        println!("try eval st: {:?}", st);
        result = eval(st.upcast(), context.clone());
        // if
        if let Some(r) = result.as_ref() {
            if r.as_any().is::<ErrorObject>() {
                return result;
            }
            if r.as_any().is::<ReturnValue>() {
                return Some(
                    r.as_any()
                        .downcast_ref::<ReturnValue>()
                        .unwrap()
                        .value
                        .clone(),
                );
            }
        }
    }
    result
}

pub fn eval_block_statement(blk: BlockStatement, context: Rc<Context>) -> Option<Rc<dyn Object>> {
    let mut result = None;
    for st in blk.statement.iter() {
        result = eval(st.upcast(), context.clone());
        if result.is_some() {
            let r = result.as_ref().unwrap();
            if r.object_type() == ERROR_OBJECT {
                return result;
            }
            if r.object_type() == RETURN_VALUE_OBJECT {
                return result;
            }
        }
    }
    result
}

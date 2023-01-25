use crate::ast::*;
use crate::token::{Token, EOF};

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Rc<dyn Expression>>,
}
impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl Statement for ExpressionStatement {
    fn statement_node(&self) {
        todo!()
    }
}
impl Expression for ExpressionStatement {
    fn expression_node(&self) {
        todo!()
    }
}
impl TryFrom<Box<&dyn Statement>> for ExpressionStatement {
    type Error = String;
    fn try_from(value: Box<&dyn Statement>) -> Result<Self, Self::Error> {
        println!("cast Statement to Expression: {:?}", value.as_any());
        if let Some(v) = value.as_any().downcast_ref::<Self>() {
            let mut ex = None;
            if v.expression.is_some() {
                if let Some(ref x) = v.expression {
                    ex = ExpressionStatement::try_from(Box::new(x.as_ref()))
                        .unwrap()
                        .expression;
                }
            };
            return Ok(ExpressionStatement {
                token: v.token.clone(),
                expression: ex,
            });
        }
        Err(format!("error cast object {:?}", value))
    }
}
impl TryFrom<Box<&dyn Expression>> for ExpressionStatement {
    type Error = String;
    fn try_from(value: Box<&dyn Expression>) -> Result<Self, Self::Error> {
        println!(
            "casting dyn Expression to ExpressionStatement, {:?}",
            *value
        );
        let x = (value).as_any(); // .downcast_ref::<Self>();
        println!(
            "casting dyn Expression to ExpressionStatement result: {:?}",
            x.downcast_ref::<Self>()
        );

        let mut ex = None;
        let mut did_match = false;
        let mut token: Token = Token {
            token_type: EOF,
            literal: "".into(),
        };

        let v_any = value.as_any();

        if v_any.is::<ExpressionStatement>() {
            did_match = true;
            // expression 不一定能转换成 ExpressionStatement 也有可能是转换成别的 dyn Expression 的
            // 这里除非知道所有的 dyn Expression 可以转换的对象，挨个都试一下？
            if let Some(v) = value.as_any().downcast_ref::<Self>() {
                if v.expression.is_some() {
                    // println!("wtf wtf: {:?}", v.expression);
                    if let Some(ref x) = v.expression {
                        ex = ExpressionStatement::try_from(Box::new(x.as_ref()))
                            .map(|x| Rc::new(x) as Rc<dyn Expression>)
                            .ok();
                    }
                };
                token = v.token.clone();

                // return Ok(Rc::new(ExpressionStatement {
                //     token: v.token.clone(),
                //     expression: ex,
                // }));
            }
        }
        if v_any.is::<Identifier>() {
            did_match = true;
            if let Some(v) = value.as_any().downcast_ref::<Identifier>() {
                token = v.token.clone();
                ex = Some(Rc::new(Identifier {
                    token: v.token.clone(),
                    value: v.value.clone(),
                }));
            }
        }
        if v_any.is::<LetStatement>() {
            did_match = true;
            if let Some(v) = value.as_any().downcast_ref::<LetStatement>() {
                token = v.token.clone();
                ex = Some(Rc::new(LetStatement {
                    token: v.token.clone(),
                    name: v.name.clone(),
                    // FIXME: change this
                    value: Box::new(
                        ExpressionStatement::try_from(Box::new(v.value.as_ref())).unwrap(),
                    ),
                }));
                println!(
                    "wwwww: {:?}",
                    Box::new(ExpressionStatement::try_from(Box::new(v.value.as_ref())).unwrap())
                );
                // ex = Some(Rc::new(IntegerLiteral { token: v.token.clone() ,value: v.value }));
                // return Ok(IntegerLiteral { token: v.token ,value: v.value});
            }
        }
        if v_any.is::<IntegerLiteral>() {
            did_match = true;
            println!("\n\n v_any is IntLiteral: {:?}\n\n", v_any);
            if let Some(v) = value.as_any().downcast_ref::<IntegerLiteral>() {
                ex = Some(Rc::new(IntegerLiteral {
                    token: v.token.clone(),
                    value: v.value,
                }));
                token = v.token.clone();
                // return Ok(IntegerLiteral { token: v.token ,value: v.value});
            }
        }
        if v_any.is::<PrefixExpression>() {
            did_match = true;
            println!("\n\nv_any is PrefixExpression\n\n");
            // FIXME: do here first
            if let Some(v) = value.as_any().downcast_ref::<PrefixExpression>() {
                ex = Some(Rc::new(PrefixExpression {
                    token: v.token.clone(),
                    operator: v.operator.clone(),
                    // FIXME: right
                    right: v.right.clone(),
                }));
                token = v.token.clone();
                // return Ok(IntegerLiteral { token: v.token ,value: v.value});
            }
        }
        if v_any.is::<InfixExpression>() {
            did_match = true;
            println!("\n\nv_any is InfixExpression\n\n");
            if let Some(val) = value.as_any().downcast_ref::<InfixExpression>() {
                token = val.token.clone();
                ex = Some(Rc::new(val.clone()))
            }
        }
        if v_any.is::<BooleanLiteral>() {
            did_match = true;
            if let Some(val) = value.as_any().downcast_ref::<BooleanLiteral>() {
                token = val.token.clone();
                ex = Some(Rc::new(val.clone()));
            }
        }
        if v_any.is::<IfExpression>() {
            did_match = true;
            if let Some(val) = value.as_any().downcast_ref::<IfExpression>() {
                token = val.token.clone();
                ex = Some(Rc::new(val.clone()))
            }
        }
        if v_any.is::<FunctionLiteral>() {
            did_match = true;
            if let Some(val) = value.as_any().downcast_ref::<FunctionLiteral>() {
                // FIXME: 以后都去掉 Rc<RefCell<
                token = val.token.clone();
                ex = Some(Rc::new(val.clone()));
            }
        }
        if did_match {
            assert_ne!(token.token_type, EOF);
            Ok(ExpressionStatement {
                token: token,
                expression: ex,
            })
        } else {
            Err(format!("error cast object {:?}", value))
        }
    }
}

impl std::fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.expression.is_none() {
            return write!(f, "");
        }
        write!(f, "{}", self.expression.as_ref().unwrap())
    }
}

mod test {
    #[allow(unused)]
    use {
        crate::{ast::ExpressionStatement, token::Token, token::EOF},
        std::{cell::RefCell, rc::Rc},
    };

    #[test]
    fn test_to_string() {
        let e = ExpressionStatement {
            token: Token {
                token_type: EOF,
                literal: ";".into(),
            },
            expression: None,
        };
        println!("{}", e);
        assert_eq!(e.to_string(), "")
    }
}

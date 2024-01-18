use {crate::ast::*, crate::token::*, std::rc::Rc};

#[ast_node(Expression)]
#[derive(Hash)]
pub struct CallExpression {
    pub token: Token,
    pub function: Option<Rc<AstExpression>>,
    pub arguments: Option<Vec<Rc<AstExpression>>>,
}

impl std::fmt::Display for CallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{}({})",
                self.function
                    .as_ref()
                    .map_or_else(|| "".into(), |f| f.to_string()),
                self.arguments.as_ref().map_or_else(
                    || "".into(),
                    |val| {
                        val.iter()
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    }
                )
            )
        )
    }
}

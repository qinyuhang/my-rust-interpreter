use crate::*;
use std::rc::Rc;

#[derive(Debug)]
pub struct Program {
    pub statement: Vec<Rc<AstExpression>>,
    // statement: Vec<String>,
}

impl Node for Program {
    fn token_literal(&self) -> Rc<String> {
        if self.statement.len() > 0 {
            self.statement[0].get_expression().token_literal().clone()
        } else {
            Rc::new("".into())
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self
            .statement
            .iter()
            .fold("".to_string(), |acc, b| format!("{acc}{b}"));
        // println!("\n\nProgram::Display: {}\n\n", &x);
        write!(f, "{}", format!("{x}"))
    }
}

#[cfg(test)]
mod test {}

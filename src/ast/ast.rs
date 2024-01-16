pub struct Statement;
pub struct Expression;

trait Node {
    fn token_literal(&self) -> String;
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        todo!();
    }
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        todo!();
    }
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            self.statements.first().unwrap().token_literal()
        } else {
            String::new()
        }
    }
}

use crate::token::Token;
use std::fmt::{Display, Formatter};

pub trait Node {
    fn token_literal(&self) -> String;
}

pub enum Statement {
    LetStatement(LetStatement),
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.token_literal())
    }
}

impl Node for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::LetStatement(stmt) => {
                format!(
                    "let {} = {}",
                    stmt.name.token_literal(),
                    stmt.value.token_literal()
                )
            }
        }
    }
}

#[derive(Clone)]
pub enum Expression {
    Empty,
}

impl Node for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Empty => "empty expression[not yet implemented]".into(),
        }
    }
}

// 标识符
#[derive(Clone)]
pub struct Ident {
    pub token: Token,
}

impl Node for Ident {
    fn token_literal(&self) -> String {
        self.token.token_literal()
    }
}

#[derive(Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Ident,
    pub value: Expression,
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        self.statements
            .first()
            .map_or("".to_string(), |s| s.token_literal())
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_let_stmt() {
        let input = "\
            let x = 5;\
            let y = 10;\
            let foobar = 838383;\
        ";

        let mut lexer = Lexer::new(input.into());
        let mut parser = Parser::new(&mut lexer);
        let program = parser.parse_program();

        let tests = ["x", "y", "foobar"];

        for (i, id) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            println!("{}", stmt)
        }
    }
}

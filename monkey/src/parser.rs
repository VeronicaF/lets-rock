use crate::ast::{Expression, Statement};
use crate::token::Token;
use crate::{ast, lexer, token};

pub struct Parser<'a> {
    lexer: &'a mut lexer::Lexer,
    cur_token: Option<token::Token>,
    next_token: Option<token::Token>,
    errors: Vec<String>,
}

impl Parser<'_> {
    pub fn new(lexer: &mut lexer::Lexer) -> Parser<'_> {
        let mut parser = Parser {
            lexer,
            cur_token: None,
            next_token: None,
            errors: vec![],
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn next_error(&mut self, token: &str) {
        let msg = format!(
            "expected next token to be {token:#?}, got {0:#?} instead",
            self.next_token
        );
        self.errors.push(msg)
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: vec![] };

        while self.cur_token.is_some() {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            };
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        if let Some(t) = self.cur_token.as_ref() {
            match t {
                Token::Let => Some(Statement::LetStatement(self.parse_let_statement()?)),
                _ => {
                    panic!("err: unknown statement: {t:#?}")
                }
            }
        } else {
            None
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
        let let_token = self.cur_token.clone()?;

        if !self.expect_next("Ident") {
            return None;
        }

        let name = ast::Ident {
            token: self.cur_token.clone()?,
        };

        if !self.expect_next("Assign") {
            return None;
        }

        while !self.cur_token_is("Semi") {
            self.next_token();
        }

        Some(ast::LetStatement {
            token: let_token,
            name,
            value: Expression::Empty,
        })
    }

    fn next_token(&mut self) {
        self.cur_token = self.next_token.take();
        self.next_token = self.lexer.next_token();
    }

    fn expect_next(&mut self, expect_token: &str) -> bool {
        if let Some(t) = self.next_token.as_ref() {
            if Into::<&str>::into(t) == expect_token {
                self.next_token();
                true
            } else {
                self.next_error(expect_token);
                false
            }
        } else {
            false
        }
    }

    fn cur_token_is(&self, expect_token: &str) -> bool {
        self.cur_token
            .as_ref()
            .is_some_and(|t| Into::<&str>::into(t) == expect_token)
    }

    fn next_token_is(&self, expect_token: &str) -> bool {
        self.next_token
            .as_ref()
            .is_some_and(|t| Into::<&str>::into(t) == expect_token)
    }
}

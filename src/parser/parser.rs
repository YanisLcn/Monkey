use crate::{
    ast::ast::{
        Expression, Identifier, InfixExpr, LetStatement, PrefixExpr, Program, ReturnStatement,
        Statement,
    },
    lexer::lexer::Lexer,
    token::token::Token,
};

pub struct Parser {
    lexer: Lexer,
    current_tok: Token,
    peek_tok: Token,
    errors: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    LOWEST = 1,
    EQUALS = 2,
    LESSGREATER = 3,
    SUM = 4,
    PRODUCT = 5,
    PREFIX = 6,
    CALL = 7,
}

fn token_to_precedence(token: Token) -> Precedence {
    match token {
        Token::EQ => Precedence::EQUALS,
        Token::NE => Precedence::EQUALS,
        Token::LT => Precedence::LESSGREATER,
        Token::GT => Precedence::LESSGREATER,
        Token::PLUS => Precedence::SUM,
        Token::SUB => Precedence::SUM,
        Token::MUL => Precedence::PRODUCT,
        Token::DIV => Precedence::PRODUCT,
        _ => Precedence::LOWEST,
    }
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            current_tok: Token::ILLEGAL(String::new()),
            peek_tok: Token::ILLEGAL(String::new()),
            errors: Vec::new(),
        }
    }

    pub fn errors(&self) -> Vec<String> {
        self.errors.clone()
    }

    pub fn next_token(&mut self) {
        self.current_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while !self.current_token_is(Token::EOF) {
            match self.parse_statement() {
                Some(statement) => program.push(statement),
                None => (),
            }
            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_tok {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_token(Token::IDENT(String::new())) {
            return None;
        };

        let name: Identifier = Identifier::new(self.current_tok.to_string());

        if !self.expect_token(Token::ASSIGN) {
            return None;
        };

        self.next_token();

        let value = match self.parse_expression(Precedence::LOWEST) {
            None => Expression::Identifier(Identifier { value: "ILLEGAL".to_string() }),
            Some(expr) => expr,
        };

        //TODO!: "We're skipping the expressions until we encounter a semicolon
        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::LetStatement(LetStatement::new(name, value)))
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let value = match self.parse_expression(Precedence::LOWEST) {
            None => Expression::Identifier(Identifier { value: "ILLEGAL".to_string() }),
            Some(expr) => expr,
        };

        //TODO!: "We're skipping the expressions until we encounter a semicolon
        while !self.current_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        Some(Statement::ReturnStatement(ReturnStatement::new(value)))
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr_statement = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(Token::SEMICOLON) {
            self.next_token();
        }

        match expr_statement {
            None => None,
            Some(expr) => Some(Statement::ExpressionStatement(expr)),
        }
    }

    pub fn parse_expression(&mut self, prec: Precedence) -> Option<Expression> {
        match self.parse_prefix(self.current_tok.clone()) {
            None => None,
            Some(left_expr) => {
                let mut expr = left_expr;
                while !self.peek_token_is(Token::SEMICOLON) && prec < self.peek_precedence() {
                    self.next_token();
                    match self.parse_infix(self.current_tok.clone(), expr.clone()) {
                        Some(infix_expr) => {
                            expr = infix_expr;
                        }
                        None => {
                            return Some(expr);
                        }
                    }
                }
                Some(expr)
            }
        }
    }

    pub fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::new(Identifier::new(
            self.current_tok.to_string(),
        )))
    }

    pub fn parse_integer(&mut self) -> Option<Expression> {
        match &self.current_tok {
            Token::INT(i) => Some(Expression::Integer(i.clone().parse::<i32>().unwrap())),
            _ => None,
        }
    }

    pub fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Bool(self.current_token_is(Token::TRUE)))
    }

    pub fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let prefix_token = self.current_tok.clone();
        self.next_token();

        if let Some(expr) = self.parse_expression(Precedence::PREFIX) {
            Some(Expression::Prefix(PrefixExpr::new(prefix_token, expr)))
        } else {
            None
        }
    }

    pub fn parse_prefix(&mut self, token: Token) -> Option<Expression> {
        match token {
            Token::IDENT(_) => self.parse_identifier(),
            Token::INT(_) => self.parse_integer(),
            Token::BANG => self.parse_prefix_expression(),
            Token::SUB => self.parse_prefix_expression(),
            Token::TRUE => self.parse_boolean(),
            Token::FALSE => self.parse_boolean(),
            Token::ILLEGAL(_) => None,
            t => {
                self.peek_errors(format!("No prefix parse function found for {}.", t).to_string());
                None
            }
        }
    }

    pub fn parse_infix_expression(&mut self, left_expr: Expression) -> Option<Expression> {
        let prefix_token = self.current_tok.clone();
        let precedence = self.current_precedence();
        self.next_token();

        if let Some(right_expr) = self.parse_expression(precedence) {
            Some(Expression::Infix(InfixExpr::new(
                prefix_token,
                left_expr,
                right_expr,
            )))
        } else {
            None
        }
    }

    pub fn parse_infix(&mut self, token: Token, expr: Expression) -> Option<Expression> {
        match token {
            Token::EQ => self.parse_infix_expression(expr),
            Token::NE => self.parse_infix_expression(expr),
            Token::LT => self.parse_infix_expression(expr),
            Token::GT => self.parse_infix_expression(expr),
            Token::PLUS => self.parse_infix_expression(expr),
            Token::SUB => self.parse_infix_expression(expr),
            Token::MUL => self.parse_infix_expression(expr),
            Token::DIV => self.parse_infix_expression(expr),
            Token::ILLEGAL(_) => None,
            t => {
                self.peek_errors(format!("No infix parse function found for {}.", t).to_string());
                None
            }
        }
    }

    pub fn current_token_is(&self, token: Token) -> bool {
        self.current_tok == token
    }

    pub fn peek_token_is(&self, token: Token) -> bool {
        self.peek_tok == token
    }

    pub fn expect_token(&mut self, token: Token) -> bool {
        if self.peek_tok == token {
            self.next_token();
            true
        } else {
            self.peek_errors(
                std::format!(
                    "expected token: {}\nreceived token: {}",
                    token,
                    self.peek_tok
                )
                .to_string(),
            );

            while !self.current_token_is(Token::SEMICOLON) {
                self.next_token();
            }
            false
        }
    }

    pub fn peek_precedence(&self) -> Precedence {
        token_to_precedence(self.peek_tok.clone())
    }

    pub fn current_precedence(&self) -> Precedence {
        token_to_precedence(self.current_tok.clone())
    }

    pub fn peek_errors(&mut self, error: String) {
        self.errors.push(error);
    }
}

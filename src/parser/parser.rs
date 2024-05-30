use crate::{
    ast::ast::{
        Arrays, CallExpression, Expression, FnExpression, Identifier, IfExpression, InfixExpr,
        LetStatement, PrefixExpr, Program, ReturnStatement, Statement,
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
        Token::LPAREN => Precedence::CALL,
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

        while !self.current_token_is(&Token::EOF) {
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
        if !self.expect_token(&Token::IDENT(String::new())) {
            return None;
        };

        let name: Identifier = Identifier::new(self.current_tok.to_string());

        if !self.expect_token(&Token::ASSIGN) {
            return None;
        };

        self.next_token();

        let value = match self.parse_expression(Precedence::LOWEST) {
            None => Expression::Identifier(Identifier {
                value: "ILLEGAL".to_string(),
            }),
            Some(expr) => expr,
        };

        //TODO!: "We're skipping the expressions until we encounter a semicolon
        while !self.current_token_is(&Token::SEMICOLON) && !self.current_token_is(&Token::EOF) {
            self.next_token();
        }

        Some(Statement::LetStatement(LetStatement::new(name, value)))
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        let value = match self.parse_expression(Precedence::LOWEST) {
            None => Expression::Identifier(Identifier {
                value: "ILLEGAL".to_string(),
            }),
            Some(expr) => expr,
        };

        //TODO!: "We're skipping the expressions until we encounter a semicolon
        while !self.current_token_is(&Token::SEMICOLON) && !self.current_token_is(&Token::EOF) {
            self.next_token();
        }

        Some(Statement::ReturnStatement(ReturnStatement::new(value)))
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr_statement = self.parse_expression(Precedence::LOWEST);

        if self.peek_token_is(&Token::SEMICOLON) {
            self.next_token();
        }

        match expr_statement {
            None => None,
            Some(expr) => Some(Statement::ExpressionStatement(expr)),
        }
    }

    pub fn parse_expression(&mut self, prec: Precedence) -> Option<Expression> {
        match self.parse_prefix(&self.current_tok.clone()) {
            None => None,
            Some(left_expr) => {
                let mut expr = left_expr;
                while !self.peek_token_is(&Token::SEMICOLON) && prec < self.peek_precedence() {
                    self.next_token();
                    match self.parse_infix(&self.current_tok.clone(), expr.clone()) {
                        Some(infix_expr) => {
                            expr = infix_expr.clone();
                        }
                        None => {
                            return Some(expr.clone());
                        }
                    }
                }
                Some(expr.clone())
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

    pub fn parse_string(&mut self) -> Option<Expression> {
        match &self.current_tok {
            Token::STRING(s) => Some(Expression::String(s.to_string())),
            _ => None,
        }
    }

    pub fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Bool(self.current_token_is(&Token::TRUE)))
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

    pub fn parse_prefix(&mut self, token: &Token) -> Option<Expression> {
        match &token {
            Token::IDENT(_) => self.parse_identifier(),
            Token::INT(_) => self.parse_integer(),
            Token::STRING(_) => self.parse_string(),
            Token::BANG => self.parse_prefix_expression(),
            Token::SUB => self.parse_prefix_expression(),
            Token::TRUE => self.parse_boolean(),
            Token::FALSE => self.parse_boolean(),
            Token::LPAREN => self.parse_grouped_expression(),
            Token::IF => self.parse_if_expression(),
            Token::FUNCTION => self.parse_function_expression(),
            Token::LBRACKET => self.parse_arrays(),
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

    pub fn parse_infix(&mut self, token: &Token, expr: Expression) -> Option<Expression> {
        match token {
            Token::EQ => self.parse_infix_expression(expr),
            Token::NE => self.parse_infix_expression(expr),
            Token::LT => self.parse_infix_expression(expr),
            Token::GT => self.parse_infix_expression(expr),
            Token::PLUS => self.parse_infix_expression(expr),
            Token::SUB => self.parse_infix_expression(expr),
            Token::MUL => self.parse_infix_expression(expr),
            Token::DIV => self.parse_infix_expression(expr),
            Token::LPAREN => self.parse_call_expression(expr),
            Token::ILLEGAL(_) => None,
            t => {
                self.peek_errors(format!("No infix parse function found for {}.", t).to_string());
                None
            }
        }
    }

    pub fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        let expr = self.parse_expression(Precedence::LOWEST);

        if self.expect_token(&Token::RPAREN) {
            expr
        } else {
            None
        }
    }

    pub fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_token(&Token::LPAREN) {
            return None;
        };

        self.next_token();
        let condition = self.parse_expression(Precedence::LOWEST)?;

        if !self.expect_token(&Token::RPAREN) {
            return None;
        };

        if !self.expect_token(&Token::LBRACE) {
            return None;
        };

        let consequence = self.parse_block_statements();

        let alternative = if self.peek_token_is(&Token::ELSE) {
            self.next_token();

            if !self.expect_token(&Token::LBRACE) {
                None
            } else {
                Some(self.parse_block_statements())
            }
        } else {
            None
        };

        return Some(Expression::IfExpression(IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        }));
    }

    pub fn parse_function_expression(&mut self) -> Option<Expression> {
        if !self.expect_token(&Token::LPAREN) {
            return None;
        };

        let params = self.parse_function_parameters();
        let parameters;

        match params {
            Some(p) => {
                parameters = p;
            }
            None => return None,
        }

        if !self.expect_token(&Token::LBRACE) {
            return None;
        };

        let body = self.parse_block_statements();

        Some(Expression::FnExpression(FnExpression { parameters, body }))
    }

    pub fn parse_function_parameters(&mut self) -> Option<Vec<Identifier>> {
        let mut parameters = vec![];

        if self.peek_token_is(&Token::RPAREN) {
            self.next_token();
            return Some(parameters);
        };

        self.next_token();
        parameters.push(Identifier {
            value: self.current_tok.to_string(),
        });

        while self.peek_token_is(&Token::COMMA) {
            self.next_token();
            self.next_token();
            parameters.push(Identifier {
                value: self.current_tok.to_string(),
            });
        }

        if !self.expect_token(&Token::RPAREN) {
            return None;
        }

        Some(parameters)
    }

    pub fn parse_block_statements(&mut self) -> Vec<Statement> {
        let mut block = vec![];

        self.next_token();
        while !self.current_token_is(&Token::RBRACE) && !self.current_token_is(&Token::EOF) {
            let stmt = self.parse_statement();

            match stmt {
                Some(stmt) => block.push(stmt),
                None => (),
            }
            self.next_token();
        }
        block
    }

    pub fn parse_call_expression(&mut self, expr: Expression) -> Option<Expression> {
        match self.parse_call_arguments() {
            Some(arguments) => Some(Expression::CallExpression(CallExpression {
                function: Box::new(expr),
                arguments,
            })),
            None => None,
        }
    }

    fn parse_list(&mut self, end_token: Token) -> Option<Vec<Expression>> {
        let mut args = vec![];

        if self.peek_token_is(&end_token) {
            self.next_token();
            return Some(args);
        };

        self.next_token();

        match self.parse_expression(Precedence::LOWEST) {
            Some(argument) => {
                args.push(argument);
            }
            None => {
                return None;
            }
        }

        while self.peek_token_is(&Token::COMMA) {
            self.next_token();
            self.next_token();

            match self.parse_expression(Precedence::LOWEST) {
                Some(argument) => {
                    args.push(argument);
                }
                None => {
                    return None;
                }
            }
        }

        if !self.expect_token(&end_token) {
            return None;
        }

        Some(args)
    }

    pub fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        self.parse_list(Token::RPAREN)
    }

    fn parse_arrays(&mut self) -> Option<Expression> {
        Some(Expression::Arrays(Arrays {
            elements: self.parse_list(Token::RBRACKET)?,
        }))
    }

    fn compare_tokens(&self, token_a: &Token, token_b: &Token) -> bool {
        match token_a {
            Token::IDENT(_) => matches!(token_b, Token::IDENT(_)),
            Token::INT(_) => matches!(token_b, Token::INT(_)),
            Token::STRING(_) => matches!(token_b, Token::STRING(_)),
            a => *a == *token_b,
        }
    }

    pub fn current_token_is(&self, token: &Token) -> bool {
        self.compare_tokens(&self.current_tok, token)
    }

    pub fn peek_token_is(&self, token: &Token) -> bool {
        self.compare_tokens(&self.peek_tok, token)
    }

    pub fn expect_token(&mut self, token: &Token) -> bool {
        if self.peek_token_is(token) {
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

            while !self.current_token_is(&Token::SEMICOLON) {
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

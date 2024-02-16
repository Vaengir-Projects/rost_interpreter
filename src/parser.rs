use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};

const LOWEST: u8 = 1;
const EQUALS: u8 = 2;
const LESSGREATER: u8 = 3;
const SUM: u8 = 4;
const PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let first_token = lexer.next_token();
        let second_token = lexer.next_token();
        Parser {
            lexer: lexer.clone(),
            cur_token: first_token,
            peek_token: second_token,
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn cur_token_is(&mut self, token_type: TokenType) -> bool {
        self.cur_token.r#type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }
        false
    }

    fn peek_token_is(&mut self, token_type: TokenType) -> bool {
        self.peek_token.r#type == token_type
    }

    fn precedences(token_type: &TokenType) -> u8 {
        match token_type {
            TokenType::Eq => EQUALS,
            TokenType::NotEq => EQUALS,
            TokenType::LT => LESSGREATER,
            TokenType::GT => LESSGREATER,
            TokenType::Plus => SUM,
            TokenType::Minus => SUM,
            TokenType::Slash => PRODUCT,
            TokenType::Asterisk => PRODUCT,
            _ => LOWEST,
        }
    }

    fn cur_precedence(&self) -> u8 {
        Self::precedences(&self.cur_token.r#type)
    }

    fn peek_precedence(&self) -> u8 {
        Self::precedences(&self.peek_token.r#type)
    }

    fn parse_statement(&mut self) -> Statement {
        match self.cur_token.r#type {
            TokenType::Let => Statement::Let(self.parse_let_statement()),
            TokenType::Return => Statement::Return(self.parse_return_statement()),
            _ => Statement::Expression(self.parse_expression_statement()),
        }
    }

    fn parse_let_statement(&mut self) -> LetStatement {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Ident) {
            panic!(
                "  Expected: {:?}\n  Got: {:?}",
                TokenType::Ident,
                self.peek_token.r#type
            );
        }
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        if !self.expect_peek(TokenType::Assign) {
            panic!(
                "  Expected: {:?}\n  Got: {:?}",
                TokenType::Assign,
                self.peek_token.r#type
            );
        }
        // TODO: We're skipping the expressions until we encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        LetStatement {
            token,
            name,
            value: Expression::Default,
        }
    }

    fn parse_return_statement(&mut self) -> ReturnStatement {
        let token = self.cur_token.clone();
        self.next_token();
        // TODO: We're skipping the expressions until we encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        ReturnStatement {
            token,
            return_value: Expression::Default,
        }
    }

    fn parse_expression_statement(&mut self) -> ExpressionStatement {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST);
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token();
        }
        ExpressionStatement { token, expression }
    }

    fn parse_expression(&mut self, precedence: u8) -> Expression {
        let prefix = match &self.cur_token.r#type {
            TokenType::Ident => self.parse_identifier(),
            TokenType::INT => self.parse_integer_literal(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            _ => panic!(
                "Prefix: The TokenType: {:?} has no function (yet)",
                self.cur_token.r#type
            ),
        };
        let mut left_expression = prefix.clone();
        while !self.peek_token_is(TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = match &self.peek_token.r#type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Slash
                | TokenType::Asterisk
                | TokenType::Eq
                | TokenType::NotEq
                | TokenType::LT
                | TokenType::GT => self.parse_infix_expression(prefix.clone()),
                _ => panic!(
                    "Infix: The TokenType: {:?} has no function (yet)",
                    self.cur_token.r#type
                ),
            };
            left_expression = infix.clone();
        }
        left_expression
    }

    fn parse_identifier(&self) -> Expression {
        Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_integer_literal(&self) -> Expression {
        let value: i64 = self
            .cur_token
            .literal
            .parse()
            .expect("Error while parsing the integer literal to i64");
        Expression::IntegerLiteral(IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        })
    }

    fn parse_prefix_expression(&mut self) -> Expression {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = Box::new(self.parse_expression(PREFIX));
        Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right,
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Expression {
        self.next_token();
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let left = Box::new(left);
        let precedence = self.cur_precedence();
        self.next_token();
        let right = Box::new(self.parse_expression(precedence));
        Expression::InfixExpression(InfixExpression {
            token,
            left,
            operator,
            right,
        })
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while !self.cur_token_is(TokenType::EOF) {
            let statement = self.parse_statement();
            program.statements.push(statement);
            self.next_token();
        }
        program
    }
}

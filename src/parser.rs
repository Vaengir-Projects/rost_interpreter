use crate::{
    ast::{Expression, Identifier, LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

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

    fn parse_statement(&mut self) -> Statement {
        match self.cur_token.r#type {
            TokenType::Let => Statement::Let(self.parse_let_statement()),
            TokenType::Return => Statement::Return(self.parse_return_statement()),
            _ => panic!("Not a valid statement."),
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

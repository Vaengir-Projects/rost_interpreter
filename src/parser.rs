use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};
use anyhow::anyhow;

const LOWEST: u8 = 1;
const _EQUALS: u8 = 2;
const _LESSGREATER: u8 = 3;
const _SUM: u8 = 4;
const _PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const _CALL: u8 = 7;
const _INDEX: u8 = 8;

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> anyhow::Result<Parser> {
        let cur_token = lexer.next_token()?;
        let peek_token = lexer.next_token()?;
        Ok(Parser {
            lexer: lexer.clone(),
            cur_token,
            peek_token,
        })
    }

    fn next_token(&mut self) -> anyhow::Result<()> {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()?;
        Ok(())
    }

    pub fn parse_program(&mut self) -> anyhow::Result<Program> {
        let mut program: Program = Program {
            statements: Vec::new(),
        };

        while self.cur_token.r#type != TokenType::EOF {
            let statement: Box<dyn Statement> = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token()?;
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> anyhow::Result<Box<dyn Statement>> {
        match &self.cur_token.r#type {
            TokenType::Let => Ok(self.parse_let_statement()?),
            TokenType::Return => Ok(self.parse_return_statement()?),
            _ => Ok(self.parse_expression_statement()?),
        }
    }

    fn parse_let_statement(&mut self) -> anyhow::Result<Box<LetStatement>> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Ident)? {
            return Err(anyhow!("Expected next TokenType::Ident"));
        }
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        if !self.expect_peek(TokenType::Assign)? {
            return Err(anyhow!("Expected next TokenType::Assign"));
        }
        // TODO: We're skipping expressions until we encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }
        Ok(Box::new(LetStatement {
            token,
            name,
            value: Box::new(Identifier {
                token: Token {
                    r#type: TokenType::Ident,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            }),
        }))
    }

    fn parse_return_statement(&mut self) -> anyhow::Result<Box<ReturnStatement>> {
        let token = self.cur_token.clone();
        self.next_token()?;
        // TODO: We're skipping expressions until we encounter a semicolon
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }
        Ok(Box::new(ReturnStatement {
            token,
            return_value: Box::new(Identifier {
                token: Token {
                    r#type: TokenType::Ident,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            }),
        }))
    }

    fn parse_expression_statement(&mut self) -> anyhow::Result<Box<ExpressionStatement>> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(&LOWEST)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }
        Ok(Box::new(ExpressionStatement { token, expression }))
    }

    fn parse_expression(&mut self, _precedence: &u8) -> anyhow::Result<Box<dyn Expression>> {
        let prefix = match &self.cur_token.r#type {
            TokenType::Ident => self.parse_identifier()?,
            TokenType::Int => self.parse_integer_literal()?,
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression()?,
            e => return Err(anyhow!("No prefix function implemented for {:?}", e)),
        };
        let left_expr = prefix;
        Ok(left_expr)
    }

    fn parse_identifier(&mut self) -> anyhow::Result<Box<dyn Expression>> {
        Ok(Box::new(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> anyhow::Result<Box<IntegerLiteral>> {
        let token = self.cur_token.clone();
        let value: i64 = self.cur_token.literal.parse()?;
        Ok(Box::new(IntegerLiteral { token, value }))
    }

    fn parse_prefix_expression(&mut self) -> anyhow::Result<Box<PrefixExpression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.as_bytes()[0];
        self.next_token()?;
        let right = self.parse_expression(&PREFIX)?;
        Ok(Box::new(PrefixExpression {
            token,
            operator,
            right,
        }))
    }

    fn cur_token_is(&self, token_type: TokenType) -> bool {
        self.cur_token.r#type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.r#type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> anyhow::Result<bool> {
        if self.peek_token_is(token_type) {
            self.next_token()?;
            return Ok(true);
        }
        Ok(false)
    }
}

use crate::ast::{Expression, Program, Statement};
use crate::token::TokenType;
use crate::{lexer::Lexer, token::Token};
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
            let statement: Statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token()?;
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> anyhow::Result<Statement> {
        match &self.cur_token.r#type {
            TokenType::Let => Ok(self.parse_let_statement()?),
            TokenType::Return => Ok(self.parse_return_statement()?),
            _ => Ok(self.parse_expression_statement()?),
        }
    }

    fn parse_expression(&mut self, _precedence: &u8) -> anyhow::Result<Expression> {
        let prefix = match &self.cur_token.r#type {
            TokenType::Ident => self.parse_identifier()?,
            TokenType::Int => self.parse_integer_literal()?,
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression()?,
            e => return Err(anyhow!("No prefix function implemented for {:?}", e)),
        };
        let left_expr = prefix;
        Ok(left_expr)
    }

    fn parse_let_statement(&mut self) -> anyhow::Result<Statement> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::Ident)? {
            return Err(anyhow!("Expected next TokenType::Ident"));
        }
        let name = Expression::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        if !self.expect_peek(TokenType::Assign)? {
            return Err(anyhow!("Expected next TokenType::Assign"));
        }
        // TODO: Parse Expressions
        // self.next_token()?;
        // let value = self.parse_expression(&LOWEST)?;
        // if self.peek_token_is(TokenType::Semicolon) {
        //     self.next_token()?;
        // }
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }
        Ok(Statement::Let {
            token,
            name,
            value: Expression::Default,
        })
    }

    fn parse_return_statement(&mut self) -> anyhow::Result<Statement> {
        let token = self.cur_token.clone();
        self.next_token()?;
        // TODO: Parse Expressions
        // let return_value = self.parse_expression(&LOWEST)?;
        // if self.peek_token_is(TokenType::Semicolon) {
        //     self.next_token()?;
        // }
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }
        Ok(Statement::Return {
            token,
            return_value: Expression::Default,
        })
    }

    fn parse_expression_statement(&mut self) -> anyhow::Result<Statement> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(&LOWEST)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }
        Ok(Statement::Expression { token, expression })
    }

    fn parse_identifier(&mut self) -> anyhow::Result<Expression> {
        Ok(Expression::Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        })
    }

    fn parse_integer_literal(&mut self) -> anyhow::Result<Expression> {
        let token = self.cur_token.clone();
        let value: i64 = self.cur_token.literal.parse()?;
        Ok(Expression::IntegerLiteral { token, value })
    }

    fn parse_prefix_expression(&mut self) -> anyhow::Result<Expression> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.as_bytes()[0];
        self.next_token()?;
        let right = Box::new(self.parse_expression(&PREFIX)?);
        Ok(Expression::PrefixExpression {
            token,
            operator,
            right,
        })
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

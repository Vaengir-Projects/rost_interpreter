use crate::{
    ast::{
        BlockStatement, Boolean, CallExpression, Expression, ExpressionStatement, FunctionLiteral,
        Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression,
        Program, ReturnStatement, Statement,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};
use anyhow::anyhow;

const LOWEST: u8 = 1;
const EQUALS: u8 = 2;
const LESSGREATER: u8 = 3;
const SUM: u8 = 4;
const PRODUCT: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;
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
        self.next_token()?;
        let value = self.parse_expression(&LOWEST)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }
        Ok(Box::new(LetStatement { token, name, value }))
    }

    fn parse_return_statement(&mut self) -> anyhow::Result<Box<ReturnStatement>> {
        let token = self.cur_token.clone();
        self.next_token()?;
        let return_value = self.parse_expression(&LOWEST)?;
        if self.peek_token_is(TokenType::Semicolon) {
            self.next_token()?;
        }
        Ok(Box::new(ReturnStatement {
            token,
            return_value,
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

    fn parse_expression(&mut self, precedence: &u8) -> anyhow::Result<Box<dyn Expression>> {
        let prefix = match &self.cur_token.r#type {
            TokenType::Ident => self.parse_identifier()?,
            TokenType::Int => self.parse_integer_literal()?,
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression()?,
            TokenType::True | TokenType::False => self.parse_boolean()?,
            TokenType::LParen => self.parse_grouped_expression()?,
            TokenType::If => self.parse_if_expression()?,
            TokenType::Function => self.parse_function_literal()?,
            e => return Err(anyhow!("No prefix function implemented for {:?}", e)),
        };
        let mut left_expr = prefix;
        while !self.peek_token_is(TokenType::Semicolon) && *precedence < self.peek_precedence() {
            let infix: Box<dyn Expression> = match &self.peek_token.r#type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Slash
                | TokenType::Asterisk
                | TokenType::Eq
                | TokenType::NotEq
                | TokenType::LessThan
                | TokenType::GreaterThan => self.parse_infix_expression(left_expr)?,
                TokenType::LParen => self.parse_call_expression(left_expr)?,
                e => return Err(anyhow!("No infix function implemented for {:?}", e)),
            };
            left_expr = infix;
        }
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

    fn parse_infix_expression(
        &mut self,
        left: Box<dyn Expression>,
    ) -> anyhow::Result<Box<InfixExpression>> {
        self.next_token()?;
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone().as_bytes().to_vec();
        let precedence = self.cur_precedence();
        self.next_token()?;
        let right = self.parse_expression(&precedence)?;
        Ok(Box::new(InfixExpression {
            token,
            left,
            operator,
            right,
        }))
    }

    fn parse_boolean(&mut self) -> anyhow::Result<Box<Boolean>> {
        Ok(Box::new(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> anyhow::Result<Box<dyn Expression>> {
        self.next_token()?;
        let exp = self.parse_expression(&LOWEST)?;
        if !self.expect_peek(TokenType::RParen)? {
            return Err(anyhow!(
                "Expected: TokenType::RParen\nGot: {:?}",
                self.peek_token
            ));
        }
        Ok(exp)
    }

    fn parse_if_expression(&mut self) -> anyhow::Result<Box<IfExpression>> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::LParen)? {
            return Err(anyhow!(
                "Expected: TokenType::LParen\nGot: {:?}",
                self.peek_token
            ));
        }
        self.next_token()?;
        let condition = self.parse_expression(&LOWEST)?;
        if !self.expect_peek(TokenType::RParen)? || !self.expect_peek(TokenType::LBrace)? {
            return Err(anyhow!(
                "Expected: TokenType::RParen, then TokenType::LBrace\nGot: {:?}",
                self.peek_token
            ));
        }
        let consequence = self.parse_block_statement()?;
        let alternative = None;
        let mut if_expression = IfExpression {
            token,
            condition,
            consequence,
            alternative,
        };
        if self.peek_token_is(TokenType::Else) {
            self.next_token()?;
            if !self.expect_peek(TokenType::LBrace)? {
                return Err(anyhow!(
                    "Expected: TokenType:RBrace\nGot: {:?}",
                    self.peek_token
                ));
            }
            if_expression.alternative = Some(self.parse_block_statement()?);
        }
        Ok(Box::new(if_expression))
    }

    fn parse_block_statement(&mut self) -> anyhow::Result<BlockStatement> {
        let token = self.cur_token.clone();
        self.next_token()?;
        let mut statements = Vec::new();
        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::EOF) {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token()?;
        }
        Ok(BlockStatement { token, statements })
    }

    fn parse_function_literal(&mut self) -> anyhow::Result<Box<FunctionLiteral>> {
        let token = self.cur_token.clone();
        if !self.expect_peek(TokenType::LParen)? {
            return Err(anyhow!(
                "Expected: TokenType::LParen\nGot: {:?}",
                self.peek_token
            ));
        }
        let parameters = self.parse_function_parameters()?;
        if !self.expect_peek(TokenType::LBrace)? {
            return Err(anyhow!(
                "Expected: TokenType::LBrace\nGot: {:?}",
                self.peek_token
            ));
        }
        let body = self.parse_block_statement()?;
        Ok(Box::new(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> anyhow::Result<Vec<Identifier>> {
        let mut identifiers: Vec<Identifier> = Vec::new();
        if self.peek_token_is(TokenType::RParen) {
            self.next_token()?;
            return Ok(identifiers);
        }
        self.next_token()?;
        identifiers.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });
        while self.peek_token_is(TokenType::Comma) {
            self.next_token()?;
            self.next_token()?;
            identifiers.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }
        if !self.expect_peek(TokenType::RParen)? {
            panic!(
                "Next TokenType should be 'RParen'\nGot: {:?}",
                self.cur_token
            );
        }
        Ok(identifiers)
    }

    fn parse_call_expression(
        &mut self,
        function: Box<dyn Expression>,
    ) -> anyhow::Result<Box<CallExpression>> {
        let token = self.cur_token.clone();
        let arguments = self.parse_call_arguments()?;
        Ok(Box::new(CallExpression {
            token,
            function,
            arguments,
        }))
    }

    fn parse_call_arguments(&mut self) -> anyhow::Result<Vec<Box<dyn Expression>>> {
        self.next_token()?;
        let mut arguments: Vec<Box<dyn Expression>> = Vec::new();
        if self.peek_token_is(TokenType::RParen) {
            self.next_token()?;
            return Ok(arguments);
        }
        self.next_token()?;
        arguments.push(self.parse_expression(&LOWEST)?);
        while self.peek_token_is(TokenType::Comma) {
            self.next_token()?;
            self.next_token()?;
            arguments.push(self.parse_expression(&LOWEST)?);
        }
        if !self.expect_peek(TokenType::RParen)? {
            return Err(anyhow!(
                "Next TokenType should be 'RParen'\nGot: {:?}",
                self.peek_token
            ));
        }
        Ok(arguments)
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

    fn precedences(token_type: &TokenType) -> u8 {
        match token_type {
            TokenType::Eq => EQUALS,
            TokenType::NotEq => EQUALS,
            TokenType::LessThan => LESSGREATER,
            TokenType::GreaterThan => LESSGREATER,
            TokenType::Plus => SUM,
            TokenType::Minus => SUM,
            TokenType::Slash => PRODUCT,
            TokenType::Asterisk => PRODUCT,
            TokenType::LParen => CALL,
            // TokenType::LBracket => INDEX,
            _ => LOWEST,
        }
    }

    fn cur_precedence(&self) -> u8 {
        Self::precedences(&self.cur_token.r#type)
    }

    fn peek_precedence(&self) -> u8 {
        Self::precedences(&self.peek_token.r#type)
    }
}

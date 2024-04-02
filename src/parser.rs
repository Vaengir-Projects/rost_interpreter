use crate::{
    ast::{
        ArrayLiteral, BlockStatement, Boolean, CallExpression, Expression, ExpressionStatement,
        FunctionLiteral, Identifier, IfExpression, InfixExpression, IntegerLiteral, LetStatement,
        PrefixExpression, Program, ReturnStatement, Statement, StringLiteral,
    },
    lexer::Lexer,
    token::{Token, TokenType},
};
use std::fmt::Display;

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

    fn expect_peek(&mut self, token_type: &TokenType) -> bool {
        if self.peek_token_is(token_type) {
            self.next_token();
            return true;
        }
        false
    }

    fn peek_token_is(&mut self, token_type: &TokenType) -> bool {
        self.peek_token.r#type == *token_type
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
            TokenType::LParen => CALL,
            _ => LOWEST,
        }
    }

    fn cur_precedence(&self) -> u8 {
        Self::precedences(&self.cur_token.r#type)
    }

    fn peek_precedence(&self) -> u8 {
        Self::precedences(&self.peek_token.r#type)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cur_token.r#type {
            TokenType::Let => {
                let let_statement = self.parse_let_statement()?;
                Ok(Statement::Let(let_statement))
            }
            TokenType::Return => {
                let return_statement = self.parse_return_statement()?;
                Ok(Statement::Return(return_statement))
            }
            _ => {
                let expression_statement = self.parse_expression_statement()?;
                Ok(Statement::Expression(expression_statement))
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, ParserError> {
        let token = self.cur_token.clone();
        if !self.expect_peek(&TokenType::Ident) {
            return Err(ParserError::StatementError(format!(
                " Expected: {:?}\n  Got: {:?}",
                TokenType::Ident,
                self.peek_token.r#type,
            )));
        }
        let name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        if !self.expect_peek(&TokenType::Assign) {
            return Err(ParserError::StatementError(format!(
                "  Expected: {:?}\n  Got: {:?}",
                TokenType::Assign,
                self.peek_token.r#type
            )));
        }
        self.next_token();
        let value = self.parse_expression(LOWEST)?;
        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }
        Ok(LetStatement { token, name, value })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParserError> {
        let token = self.cur_token.clone();
        self.next_token();
        let return_value = self.parse_expression(LOWEST)?;
        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }
        Ok(ReturnStatement {
            token,
            return_value,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<ExpressionStatement, ParserError> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST)?;
        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }
        Ok(ExpressionStatement { token, expression })
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Expression, ParserError> {
        let prefix = match &self.cur_token.r#type {
            TokenType::Ident => {
                let i = self.parse_identifier()?;
                Ok(i)
            }
            TokenType::Int => {
                let i = self.parse_integer_literal()?;
                Ok(i)
            }
            TokenType::Bang | TokenType::Minus => {
                let p = self.parse_prefix_expression()?;
                Ok(p)
            }
            TokenType::True | TokenType::False => {
                let b = self.parse_boolean()?;
                Ok(b)
            }
            TokenType::LParen => {
                let lp = self.parse_grouped_expression()?;
                Ok(lp)
            }
            TokenType::If => {
                let i = self.parse_if_expression()?;
                Ok(i)
            }
            TokenType::Function => {
                let f = self.parse_function_literal()?;
                Ok(f)
            }
            TokenType::String => {
                let s = self.parse_string_literal()?;
                Ok(s)
            }
            TokenType::LBracket => {
                let token = self.cur_token.clone();
                let elements = self.parse_expression_list(&TokenType::RBracket)?;
                Ok(Expression::ArrayLiteral(ArrayLiteral { token, elements }))
            }
            _ => Err(ParserError::ExpressionError(format!(
                "Prefix: The TokenType: {:?} has no function (yet)",
                self.cur_token.r#type
            ))),
        };
        let mut left_expression = prefix.clone()?;
        while !self.peek_token_is(&TokenType::Semicolon) && precedence < self.peek_precedence() {
            let infix = match &self.peek_token.r#type {
                TokenType::Plus
                | TokenType::Minus
                | TokenType::Slash
                | TokenType::Asterisk
                | TokenType::Eq
                | TokenType::NotEq
                | TokenType::LT
                | TokenType::GT => {
                    let i = self.parse_infix_expression(left_expression.clone())?;
                    Ok(i)
                }
                TokenType::LParen => {
                    let c = self.parse_call_expression(left_expression.clone())?;
                    Ok(c)
                }
                _ => {
                    return Err(ParserError::InfixExpression(format!(
                        "Infix: The TokenType: {:?} has no function (yet)",
                        self.cur_token.r#type
                    )))
                }
            };
            left_expression = infix.clone()?;
        }
        Ok(left_expression)
    }

    fn parse_identifier(&self) -> Result<Expression, ParserError> {
        Ok(Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&self) -> Result<Expression, ParserError> {
        let value: i64 = self
            .cur_token
            .literal
            .parse()
            .expect("Error while parsing the integer literal to i64");
        Ok(Expression::IntegerLiteral(IntegerLiteral {
            token: self.cur_token.clone(),
            value,
        }))
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParserError> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = Box::new(self.parse_expression(PREFIX)?);
        Ok(Expression::PrefixExpression(PrefixExpression {
            token,
            operator,
            right,
        }))
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParserError> {
        self.next_token();
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let left = Box::new(left);
        let precedence = self.cur_precedence();
        self.next_token();
        let right = Box::new(self.parse_expression(precedence)?);
        Ok(Expression::InfixExpression(InfixExpression {
            token,
            left,
            operator,
            right,
        }))
    }

    fn parse_boolean(&mut self) -> Result<Expression, ParserError> {
        Ok(Expression::Boolean(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParserError> {
        self.next_token();
        let expression = self.parse_expression(LOWEST)?;
        if !self.expect_peek(&TokenType::RParen) {
            return Err(ParserError::GroupedExpression(String::from(
                "Can't match parens",
            )));
        }
        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParserError> {
        let token = self.cur_token.clone();
        if !self.expect_peek(&TokenType::LParen) {
            return Err(ParserError::IfExpression(String::from(
                "Next TokenType should be 'LParen'",
            )));
        }
        self.next_token();
        let condition = Box::new(self.parse_expression(LOWEST)?);
        if !self.expect_peek(&TokenType::RParen) || !self.expect_peek(&TokenType::LBrace) {
            return Err(ParserError::IfExpression(String::from(
                "Next TokenTypes should be 'RParen' and then 'LBrace'",
            )));
        }
        let consequence = self.parse_block_statement()?;
        let mut if_expression = IfExpression {
            token,
            condition,
            consequence,
            alternative: None,
        };
        if self.peek_token_is(&TokenType::Else) {
            self.next_token();
            if !self.expect_peek(&TokenType::LBrace) {
                return Err(ParserError::IfExpression(String::from(
                    "Next Tokentype should be 'LBrace'",
                )));
            }
            if_expression.alternative = Some(self.parse_block_statement()?);
        }
        Ok(Expression::IfExpression(if_expression))
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        let token = self.cur_token.clone();
        let mut statements = Vec::new();
        self.next_token();
        while !self.cur_token_is(TokenType::RBrace) && !self.cur_token_is(TokenType::EOF) {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }
        Ok(BlockStatement { token, statements })
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParserError> {
        let token = self.cur_token.clone();
        if !self.expect_peek(&TokenType::LParen) {
            return Err(ParserError::FunctionLiteral(String::from(
                "Next TokenType should be 'LParen'",
            )));
        }
        let parameters = self.parse_function_parameters();
        if !self.expect_peek(&TokenType::LBrace) {
            return Err(ParserError::FunctionLiteral(String::from(
                "Next TokenType should be 'LBrace'",
            )));
        }
        let body = self.parse_block_statement()?;
        Ok(Expression::FunctionLiteral(FunctionLiteral {
            token,
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut identifiers: Vec<Identifier> = Vec::new();
        if self.peek_token_is(&TokenType::RParen) {
            self.next_token();
            return identifiers;
        }
        self.next_token();
        identifiers.push(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });
        while self.peek_token_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            identifiers.push(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            });
        }
        if !self.expect_peek(&TokenType::RParen) {
            panic!(
                "Next TokenType should be 'RParen'\nGot: {:?}",
                self.cur_token
            );
        }
        identifiers
    }

    fn parse_call_expression(&mut self, func: Expression) -> Result<Expression, ParserError> {
        let token = self.cur_token.clone();
        let function = Box::new(func);
        self.next_token();
        let arguments = self.parse_expression_list(&TokenType::RParen)?;
        Ok(Expression::CallExpression(CallExpression {
            token,
            function,
            arguments,
        }))
    }

    fn parse_string_literal(&mut self) -> Result<Expression, ParserError> {
        Ok(Expression::StringLiteral(StringLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_expression_list(&mut self, end: &TokenType) -> Result<Vec<Expression>, ParserError> {
        let mut list: Vec<Expression> = Vec::new();
        if self.peek_token_is(end) {
            self.next_token();
            return Ok(list);
        }
        self.next_token();
        list.push(self.parse_expression(LOWEST)?);
        while self.peek_token_is(&TokenType::Comma) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(LOWEST)?);
        }
        if !self.expect_peek(end) {
            return Err(ParserError::ExpressionError(format!(
                "Next TokenType should be '{:?}'\nGot: Peek: {:?} - Cur: {:?}",
                end, self.peek_token, self.cur_token
            )));
        }
        Ok(list)
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut program = Program {
            statements: Vec::new(),
        };

        while !self.cur_token_is(TokenType::EOF) {
            let statement = self.parse_statement()?;
            program.statements.push(statement);
            self.next_token();
        }
        Ok(program)
    }
}

#[derive(Debug, Clone)]
pub enum ParserError {
    StatementError(String),
    ReturnError(String),
    ExpressionError(String),
    GroupedExpression(String),
    IfExpression(String),
    FunctionLiteral(String),
    InfixExpression(String),
    CallArguments(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::StatementError(s) => write!(f, "Statement Error:\n {}", s),
            ParserError::ReturnError(r) => write!(f, "Return Error:\n {}", r),
            ParserError::ExpressionError(e) => write!(f, "Expression Error:\n {}", e),
            ParserError::GroupedExpression(g) => write!(f, "GroupedExpression Error:\n {}", g),
            ParserError::IfExpression(i) => write!(f, "IfExpression Error:\n {}", i),
            ParserError::FunctionLiteral(fl) => write!(f, "FunctionLiteral Error:\n {}", fl),
            ParserError::InfixExpression(i) => write!(f, "InfixExpression Error:\n {}", i),
            ParserError::CallArguments(c) => write!(f, "CallArguments Error:\n {}", c),
        }
    }
}

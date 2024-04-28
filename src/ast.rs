use crate::token::{self, Token};
use std::fmt::Display;

pub trait NodeTrait: Display {
    fn token_literal(&self) -> String;
}

pub trait StatementTrait: NodeTrait {}

pub trait ExpressionTrait: NodeTrait {}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl NodeTrait for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            self.statements[0].token_literal()
        } else {
            String::from("")
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Statement {
    Let {
        token: Token,
        name: Expression,
        value: Expression,
    },
    Return {
        token: Token,
        return_value: Expression,
    },
    Expression {},
}

impl NodeTrait for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.literal.clone(),
            Statement::Return { token, .. } => token.literal.clone(),
            Statement::Expression {} => todo!(),
        }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Let { name, value, .. } => {
                write!(f, "{} {} = {};", self.token_literal(), name, value)
            }
            Statement::Return { return_value, .. } => {
                write!(f, "{} {};", self.token_literal(), return_value)
            }
            Statement::Expression {} => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier { token: Token, value: String },
    IntegerLiteral {},
    PrefixExpression {},
    InfixExpression {},
    Boolean {},
    IfExpression {},
    BlockStatement {},
    FunctionLiteral {},
    CallExpression {},
    StringLiteral {},
    ArrayLiteral {},
    IndexExpression {},
    Default,
}

impl NodeTrait for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier { token, .. } => token.literal.clone(),
            Expression::IntegerLiteral {} => todo!(),
            Expression::PrefixExpression {} => todo!(),
            Expression::InfixExpression {} => todo!(),
            Expression::Boolean {} => todo!(),
            Expression::IfExpression {} => todo!(),
            Expression::BlockStatement {} => todo!(),
            Expression::FunctionLiteral {} => todo!(),
            Expression::CallExpression {} => todo!(),
            Expression::StringLiteral {} => todo!(),
            Expression::ArrayLiteral {} => todo!(),
            Expression::IndexExpression {} => todo!(),
            Expression::Default => todo!(),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier { token, value } => write!(f, "{}", value),
            Expression::IntegerLiteral {} => todo!(),
            Expression::PrefixExpression {} => todo!(),
            Expression::InfixExpression {} => todo!(),
            Expression::Boolean {} => todo!(),
            Expression::IfExpression {} => todo!(),
            Expression::BlockStatement {} => todo!(),
            Expression::FunctionLiteral {} => todo!(),
            Expression::CallExpression {} => todo!(),
            Expression::StringLiteral {} => todo!(),
            Expression::ArrayLiteral {} => todo!(),
            Expression::IndexExpression {} => todo!(),
            Expression::Default => todo!(),
        }
    }
}

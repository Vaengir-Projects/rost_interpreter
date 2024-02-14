use crate::token::Token;

pub trait NodeTrait {
    fn token_literal(&self) -> String;
}

pub trait StatementTrait: NodeTrait {
    fn statement_node();
}

trait ExpressionTrait: NodeTrait {
    fn expression_node(&self);
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl NodeTrait for Program {
    fn token_literal(&self) -> String {
        if !self.statements.is_empty() {
            return self.statements[0].token_literal();
        }
        String::from("")
    }
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
}

impl NodeTrait for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let(ls) => ls.token_literal(),
            Statement::Return(rs) => rs.token_literal(),
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Default,
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl NodeTrait for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl StatementTrait for LetStatement {
    fn statement_node() {}
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl NodeTrait for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl StatementTrait for ReturnStatement {
    fn statement_node() {}
}

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl NodeTrait for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl ExpressionTrait for Identifier {
    fn expression_node(&self) {}
}

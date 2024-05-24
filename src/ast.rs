use crate::token::Token;
use std::{collections::HashMap, fmt::Display, hash::Hash};

pub trait NodeTrait: Display {
    fn token_literal(&self) -> String;
}

#[derive(Debug)]
pub enum Node<'a> {
    Statement(&'a Statement),
    Expression(&'a Expression),
    Program(&'a Program),
}

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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
    Expression {
        token: Token,
        expression: Expression,
    },
}

impl NodeTrait for Statement {
    fn token_literal(&self) -> String {
        match self {
            Statement::Let { token, .. } => token.literal.clone(),
            Statement::Return { token, .. } => token.literal.clone(),
            Statement::Expression { token, .. } => token.literal.clone(),
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
            Statement::Expression { expression, .. } => {
                write!(f, "{}", expression)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Identifier {
        token: Token,
        value: String,
    },
    IntegerLiteral {
        token: Token,
        value: i64,
    },
    PrefixExpression {
        token: Token,
        operator: u8,
        right: Box<Expression>,
    },
    InfixExpression {
        token: Token,
        left: Box<Expression>,
        operator: Vec<u8>,
        right: Box<Expression>,
    },
    Boolean {
        token: Token,
        value: bool,
    },
    IfExpression {
        token: Token,
        condition: Box<Expression>,
        consequence: Box<Expression>,
        alternative: Option<Box<Expression>>,
    },
    BlockStatement {
        token: Token,
        statements: Vec<Statement>,
    },
    FunctionLiteral {
        token: Token,
        parameters: Vec<Expression>,
        body: Box<Expression>,
    },
    CallExpression {
        token: Token,
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
    StringLiteral {
        token: Token,
        value: Vec<u8>,
    },
    ArrayLiteral {
        token: Token,
        elements: Vec<Expression>,
    },
    IndexExpression {
        token: Token,
        left: Box<Expression>,
        index: Box<Expression>,
    },
    HashLiteral {
        token: Token,
        pairs: HashMap<Box<Expression>, Box<Expression>>,
    },
    Default,
}

impl NodeTrait for Expression {
    fn token_literal(&self) -> String {
        match self {
            Expression::Identifier { token, .. } => token.literal.clone(),
            Expression::IntegerLiteral { token, .. } => token.literal.clone(),
            Expression::PrefixExpression { token, .. } => token.literal.clone(),
            Expression::InfixExpression { token, .. } => token.literal.clone(),
            Expression::Boolean { token, .. } => token.literal.clone(),
            Expression::IfExpression { token, .. } => token.literal.clone(),
            Expression::BlockStatement { token, .. } => token.literal.clone(),
            Expression::FunctionLiteral { token, .. } => token.literal.clone(),
            Expression::CallExpression { token, .. } => token.literal.clone(),
            Expression::StringLiteral { token, .. } => token.literal.clone(),
            Expression::ArrayLiteral { token, .. } => token.literal.clone(),
            Expression::IndexExpression { token, .. } => token.literal.clone(),
            Expression::HashLiteral { token, .. } => token.literal.clone(),
            Expression::Default => todo!(),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier { value, .. } => write!(f, "{}", value),
            Expression::IntegerLiteral { value, .. } => write!(f, "{}", value),
            Expression::PrefixExpression {
                operator, right, ..
            } => write!(f, "({}{})", *operator as char, right),
            Expression::InfixExpression {
                left,
                operator,
                right,
                ..
            } => write!(
                f,
                "({} {} {})",
                left,
                String::from_utf8(operator.clone())
                    .expect("Couldn't convert operator bytes to String"),
                right
            ),
            Expression::Boolean { value, .. } => write!(f, "{}", value),
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
                ..
            } => match &alternative {
                Some(alternative) => {
                    write!(f, "if {} {} else {}", condition, consequence, alternative)
                }
                None => write!(f, "if {} {}", condition, consequence),
            },
            Expression::BlockStatement { statements, .. } => {
                for statement in statements {
                    write!(f, "{}", statement)?;
                }
                Ok(())
            }
            Expression::FunctionLiteral {
                parameters, body, ..
            } => {
                let parameters = parameters
                    .iter()
                    .map(|p| match p {
                        Expression::Identifier { value, .. } => value.to_string(),
                        e => panic!(
                            "Parameter needs to be an Expression::Identifier\nGot: {}",
                            e
                        ),
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "Here: {}({}) {}", self.token_literal(), parameters, body)
            }
            Expression::CallExpression {
                function,
                arguments,
                ..
            } => {
                let arguments = arguments
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{}({})", function, arguments,)
            }
            Expression::StringLiteral { value, .. } => {
                write!(f, "{}", String::from_utf8_lossy(value))
            }
            Expression::ArrayLiteral { elements, .. } => {
                let elements = elements
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{}]", elements)
            }
            Expression::IndexExpression { left, index, .. } => write!(f, "({}[{}])", left, index),
            Expression::HashLiteral { pairs, .. } => {
                let mut pairs_str = Vec::new();
                for (k, v) in pairs {
                    pairs_str.push(format!("{}: {}", k, v));
                }
                write!(f, "{{{}}}", pairs_str.join(", "))
            }
            Expression::Default => todo!(),
        }
    }
}

impl Hash for Expression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Expression::Identifier { token, value } => {
                token.hash(state);
                value.hash(state);
            }
            Expression::IntegerLiteral { token, value } => {
                token.hash(state);
                value.hash(state);
            }
            Expression::PrefixExpression {
                token,
                operator,
                right,
            } => {
                token.hash(state);
                operator.hash(state);
                right.hash(state);
            }
            Expression::InfixExpression {
                token,
                left,
                operator,
                right,
            } => {
                token.hash(state);
                left.hash(state);
                operator.hash(state);
                right.hash(state);
            }
            Expression::Boolean { token, value } => {
                token.hash(state);
                value.hash(state);
            }
            Expression::IfExpression {
                token,
                condition,
                consequence,
                alternative,
            } => {
                token.hash(state);
                condition.hash(state);
                consequence.hash(state);
                alternative.hash(state);
            }
            Expression::BlockStatement { token, statements } => {
                token.hash(state);
                statements.hash(state);
            }
            Expression::FunctionLiteral {
                token,
                parameters,
                body,
            } => {
                token.hash(state);
                parameters.hash(state);
                body.hash(state);
            }
            Expression::CallExpression {
                token,
                function,
                arguments,
            } => {
                token.hash(state);
                function.hash(state);
                arguments.hash(state);
            }
            Expression::StringLiteral { token, value } => {
                token.hash(state);
                value.hash(state);
            }
            Expression::ArrayLiteral { token, elements } => {
                token.hash(state);
                elements.hash(state);
            }
            Expression::IndexExpression { token, left, index } => {
                token.hash(state);
                left.hash(state);
                index.hash(state);
            }
            Expression::HashLiteral { token, pairs } => {
                token.hash(state);
                for (k, v) in pairs {
                    k.hash(state);
                    v.hash(state);
                }
            }
            Expression::Default => todo!(),
        }
    }
}

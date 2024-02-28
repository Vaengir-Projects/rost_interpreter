use crate::{
    ast::{Expression, ExpressionStatement, LetStatement, Program, ReturnStatement, Statement},
    object::{Integer, Object},
};
use std::fmt::Display;

pub trait Eval {
    fn on_eval(&self) -> Result<Object, EvaluationError>;
}

impl Eval for Program {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        eval_statements(&self.statements)
    }
}

impl Eval for ExpressionStatement {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        eval(self.expression.clone())
    }
}

impl Eval for Expression {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        match self {
            Expression::IntegerLiteral(i) => Ok(Object::Integer(Integer { value: i.value })),
            e => Err(EvaluationError::MatchError(format!(
                "Not yet implemented: {}",
                e
            ))),
        }
    }
}

impl Eval for Statement {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        match self {
            Statement::Let(ls) => {
                let obj = eval(ls.clone())?;
                Ok(obj)
            }
            Statement::Return(rs) => {
                let obj = eval(rs.clone())?;
                Ok(obj)
            }
            Statement::Expression(es) => {
                let obj = eval(es.clone())?;
                Ok(obj)
            }
        }
    }
}

impl Eval for LetStatement {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        todo!()
    }
}

impl Eval for ReturnStatement {
    fn on_eval(&self) -> Result<Object, EvaluationError> {
        todo!()
    }
}

pub fn eval<T: Eval + std::fmt::Debug>(node: T) -> Result<Object, EvaluationError> {
    Ok(node.on_eval()?)
}

fn eval_statements(statements: &[Statement]) -> Result<Object, EvaluationError> {
    let mut result: Object = Object::Null;
    for statement in statements {
        result = eval(statement.clone())?;
    }
    Ok(result)
}

#[derive(Debug, Clone)]
pub enum EvaluationError {
    EvalError(String),
    MatchError(String),
}

impl Display for EvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvaluationError::EvalError(e) => write!(f, "EvaluationError: {}", e),
            EvaluationError::MatchError(m) => write!(f, "MatchError: {}", m),
        }
    }
}

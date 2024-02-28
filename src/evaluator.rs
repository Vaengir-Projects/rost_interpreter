use crate::{
    ast::{Expression, ExpressionStatement, LetStatement, Program, ReturnStatement, Statement},
    object::{Integer, Object},
};

pub trait Eval {
    fn on_eval(&self) -> Object;
}

impl Eval for Program {
    fn on_eval(&self) -> Object {
        eval_statements(&self.statements)
    }
}

impl Eval for ExpressionStatement {
    fn on_eval(&self) -> Object {
        eval(self.expression.clone())
    }
}

impl Eval for Expression {
    fn on_eval(&self) -> Object {
        match self {
            Expression::IntegerLiteral(i) => Object::Integer(Integer { value: i.value }),
            e => panic!("Not yet implemented: {}", e),
        }
    }
}

impl Eval for Statement {
    fn on_eval(&self) -> Object {
        match self {
            Statement::Let(ls) => eval(ls.clone()),
            Statement::Return(rs) => eval(rs.clone()),
            Statement::Expression(es) => eval(es.clone()),
        }
    }
}

impl Eval for LetStatement {
    fn on_eval(&self) -> Object {
        todo!()
    }
}

impl Eval for ReturnStatement {
    fn on_eval(&self) -> Object {
        todo!()
    }
}

pub fn eval<T: Eval + std::fmt::Debug>(node: T) -> Object {
    node.on_eval()
}

fn eval_statements(statements: &[Statement]) -> Object {
    let mut result: Object = Object::Null;
    for statement in statements {
        result = eval(statement.clone())
    }
    result
}

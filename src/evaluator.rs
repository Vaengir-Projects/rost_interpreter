use crate::{
    ast::{Expression, Node, Statement},
    object::Object,
};

const NULL: Object = Object::Null;
const TRUE: Object = Object::Boolean { value: true };
const FALSE: Object = Object::Boolean { value: false };

#[derive(Debug)]
pub struct Evaluator;

impl Evaluator {
    pub fn eval(node: Node) -> anyhow::Result<Object> {
        match node {
            Node::Program(p) => Evaluator::eval_statements(&p.statements),
            Node::Statement(s) => match s {
                crate::ast::Statement::Let { token, name, value } => todo!(),
                crate::ast::Statement::Return {
                    token,
                    return_value,
                } => todo!(),
                crate::ast::Statement::Expression { expression, .. } => {
                    Evaluator::eval(Node::Expression(expression))
                }
            },
            Node::Expression(e) => match e {
                Expression::IntegerLiteral { value, .. } => Ok(Object::Integer { value: *value }),
                Expression::Identifier { token, value } => todo!(),
                Expression::PrefixExpression {
                    token,
                    operator,
                    right,
                } => todo!(),
                Expression::InfixExpression {
                    token,
                    left,
                    operator,
                    right,
                } => todo!(),
                Expression::Boolean { value, .. } => Ok(native_bool_to_bool_struct(&value)),
                Expression::IfExpression {
                    token,
                    condition,
                    consequence,
                    alternative,
                } => todo!(),
                Expression::BlockStatement { token, statements } => todo!(),
                Expression::FunctionLiteral {
                    token,
                    parameters,
                    body,
                } => todo!(),
                Expression::CallExpression {
                    token,
                    function,
                    arguments,
                } => todo!(),
                Expression::StringLiteral {} => todo!(),
                Expression::ArrayLiteral {} => todo!(),
                Expression::IndexExpression {} => todo!(),
                Expression::Default => todo!(),
            },
        }
    }

    fn eval_statements(statements: &[Statement]) -> anyhow::Result<Object> {
        let mut result: Object = NULL;
        for statement in statements {
            result = Evaluator::eval(Node::Statement(statement))?
        }
        Ok(result)
    }
}

fn native_bool_to_bool_struct(input: &bool) -> Object {
    if *input {
        return TRUE;
    }
    FALSE
}

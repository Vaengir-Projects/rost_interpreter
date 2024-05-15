use crate::{
    ast::{Expression, Node, Program, Statement},
    object::{Environment, Object, ObjectTrait},
};
use anyhow::{anyhow, Context};
use std::ops::Deref;

const NULL: Object = Object::Null;
const TRUE: Object = Object::Boolean { value: true };
const FALSE: Object = Object::Boolean { value: false };

#[derive(Debug)]
pub struct Evaluator;

impl Evaluator {
    pub fn eval(node: Node, env: &mut Environment) -> anyhow::Result<Object> {
        match node {
            Node::Program(p) => Evaluator::eval_program(p, env),
            Node::Statement(s) => match s {
                crate::ast::Statement::Let { name, value, .. } => {
                    let val = Evaluator::eval(Node::Expression(value), env)?;
                    if let Expression::Identifier { value, .. } = name {
                        return Ok(env.set(value, val));
                    };
                    unreachable!()
                }
                crate::ast::Statement::Return { return_value, .. } => {
                    let value = Box::new(Evaluator::eval(Node::Expression(return_value), env)?);
                    Ok(Object::ReturnValue { value })
                }
                crate::ast::Statement::Expression { expression, .. } => {
                    Evaluator::eval(Node::Expression(expression), env)
                }
            },
            Node::Expression(e) => match e {
                Expression::IntegerLiteral { value, .. } => Ok(Object::Integer { value: *value }),
                Expression::Identifier { value, .. } => Evaluator::eval_identifier(value, env),
                Expression::PrefixExpression {
                    operator, right, ..
                } => {
                    let right = Evaluator::eval(Node::Expression(right.deref()), env)?;
                    Ok(Evaluator::eval_prefix_expression(operator, right)?)
                }
                Expression::InfixExpression {
                    left,
                    operator,
                    right,
                    ..
                } => {
                    let left = Evaluator::eval(Node::Expression(left.deref()), env)?;
                    let right = Evaluator::eval(Node::Expression(right.deref()), env)?;
                    Ok(Evaluator::eval_infix_expression(operator, left, right)?)
                }
                Expression::Boolean { value, .. } => Ok(native_bool_to_bool_struct(value)),
                Expression::IfExpression {
                    condition,
                    consequence,
                    alternative,
                    ..
                } => Ok(Evaluator::eval_if_expression(
                    condition.deref(),
                    consequence.deref(),
                    alternative,
                    env,
                )?),
                Expression::BlockStatement { statements, .. } => {
                    Evaluator::eval_block_statement(statements, env)
                }
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

    fn eval_program(program: &Program, env: &mut Environment) -> anyhow::Result<Object> {
        let mut result: Object = NULL;
        for statement in &program.statements {
            result = Evaluator::eval(Node::Statement(statement), env)?;
            if let Object::ReturnValue { value } = result {
                return Ok(*value);
            };
        }
        Ok(result)
    }

    fn eval_block_statement(block: &[Statement], env: &mut Environment) -> anyhow::Result<Object> {
        let mut result = Object::Null;
        for statement in block {
            result = Evaluator::eval(Node::Statement(statement), env)?;
            if let Object::ReturnValue { .. } = result {
                return Ok(result);
            }
        }
        Ok(result)
    }

    fn eval_prefix_expression(operator: &u8, right: Object) -> anyhow::Result<Object> {
        match operator {
            b'!' => Ok(Evaluator::eval_bang_operator_expression(right)),
            b'-' => Ok(Evaluator::eval_minus_operator_expression(right)?),
            _ => Err(anyhow!("{}{}", operator, right)),
        }
    }

    fn eval_bang_operator_expression(right: Object) -> Object {
        match right {
            TRUE => FALSE,
            FALSE => TRUE,
            NULL => TRUE,
            _ => FALSE,
        }
    }

    fn eval_minus_operator_expression(right: Object) -> anyhow::Result<Object> {
        let integer = match right {
            Object::Integer { value } => value,
            _ => return Err(anyhow!("-{}", right.r#type())),
        };
        Ok(Object::Integer { value: -integer })
    }

    fn eval_infix_expression(
        operator: &[u8],
        left: Object,
        right: Object,
    ) -> anyhow::Result<Object> {
        if left.r#type() != right.r#type() {
            return Err(anyhow!(
                "{} {} {}",
                left.r#type(),
                String::from_utf8_lossy(operator),
                right.r#type()
            ));
        }
        match (&left, &right) {
            (Object::Integer { value: v1 }, Object::Integer { value: v2 }) => {
                Ok(Evaluator::eval_integer_infix_expression(operator, v1, v2)?)
            }
            // (Object::String(s), Object::String()) => Ok(eval_string_infix_expression(
            //     operator,
            //     s.clone(),
            //     s2.clone(),
            // )?),
            _ => match operator {
                b"==" => Ok(native_bool_to_bool_struct(&(left == right))),
                b"!=" => Ok(native_bool_to_bool_struct(&(left != right))),
                _ => Err(anyhow!(
                    "{} {} {}",
                    left.r#type(),
                    String::from_utf8_lossy(operator),
                    right.r#type()
                )),
            },
        }
    }

    fn eval_integer_infix_expression(
        operator: &[u8],
        left: &i64,
        right: &i64,
    ) -> anyhow::Result<Object> {
        match operator {
            b"+" => Ok(Object::Integer {
                value: left + right,
            }),
            b"-" => Ok(Object::Integer {
                value: left - right,
            }),
            b"*" => Ok(Object::Integer {
                value: left * right,
            }),
            b"/" => Ok(Object::Integer {
                value: left / right,
            }),
            b"<" => Ok(native_bool_to_bool_struct(&(left < right))),
            b">" => Ok(native_bool_to_bool_struct(&(left > right))),
            b"==" => Ok(native_bool_to_bool_struct(&(left == right))),
            b"!=" => Ok(native_bool_to_bool_struct(&(left != right))),
            _ => Err(anyhow!(
                "{} {} {}",
                left,
                String::from_utf8_lossy(operator),
                right
            )),
        }
    }

    fn eval_if_expression(
        condition: &Expression,
        consequence: &Expression,
        alternative: &Option<Box<Expression>>,
        env: &mut Environment,
    ) -> anyhow::Result<Object> {
        let condition = Evaluator::eval(Node::Expression(condition), env)?;
        if is_truthy(condition) {
            return Evaluator::eval(Node::Expression(consequence), env);
        } else if let Some(alternative) = alternative.as_ref() {
            return Evaluator::eval(Node::Expression(alternative.deref()), env);
        }
        Ok(NULL)
    }

    fn eval_identifier(node: &str, env: &mut Environment) -> anyhow::Result<Object> {
        env.get(node)
            .with_context(|| format!("Identifier not found: {}", node))
    }
}

fn native_bool_to_bool_struct(input: &bool) -> Object {
    if *input {
        return TRUE;
    }
    FALSE
}

fn is_truthy(object: Object) -> bool {
    match object {
        Object::Null => false,
        TRUE => true,
        FALSE => false,
        _ => true,
    }
}

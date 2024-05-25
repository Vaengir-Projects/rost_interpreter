use crate::{
    ast::{Expression, Node, Program, Statement},
    builtin::BuiltInFunction,
    object::{Environment, Object, ObjectTrait},
};
use anyhow::anyhow;
use std::{collections::HashMap, ops::Deref};

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
                    parameters, body, ..
                } => Ok(Object::Function {
                    parameters: parameters.to_vec(),
                    body: body.deref().clone(),
                    env: env.clone(),
                }),
                Expression::CallExpression {
                    function,
                    arguments,
                    ..
                } => {
                    let function = Evaluator::eval(Node::Expression(function), env)?;
                    let args = Evaluator::eval_expressions(arguments, env)?;
                    Evaluator::apply_function(function, &args)
                }
                Expression::StringLiteral { value, .. } => Ok(Object::String {
                    value: value.to_vec(),
                }),
                Expression::ArrayLiteral { elements, .. } => {
                    let elements = Evaluator::eval_expressions(elements, env)?;
                    Ok(Object::Array { elements })
                }
                Expression::IndexExpression { left, index, .. } => {
                    let left = Evaluator::eval(Node::Expression(left), env)?;
                    let index = Evaluator::eval(Node::Expression(index), env)?;
                    Evaluator::eval_index_expression(left, index)
                }
                Expression::HashLiteral { pairs, .. } => {
                    Ok(Evaluator::eval_hash_literal(pairs, env)?)
                }
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
            (Object::String { value: v1 }, Object::String { value: v2 }) => {
                Ok(Evaluator::eval_string_infix_expression(operator, v1, v2)?)
            }
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
        if let Ok(val) = env.get(node) {
            return Ok(val);
        }
        if let Ok(builtin) = BuiltInFunction::is_builtin(node) {
            return Ok(builtin);
        }
        Err(anyhow!("Identifier not found: {}", node))
    }

    fn eval_expressions(
        expressions: &[Expression],
        env: &mut Environment,
    ) -> anyhow::Result<Vec<Object>> {
        let mut result: Vec<Object> = Vec::new();
        for expression in expressions {
            let evaluated = Evaluator::eval(Node::Expression(expression), env)?;
            result.push(evaluated);
        }
        Ok(result)
    }

    fn eval_index_expression(left: Object, index: Object) -> anyhow::Result<Object> {
        match (&left, &index) {
            (Object::Array { elements }, Object::Integer { value }) => {
                Evaluator::eval_array_index_expression(elements.clone(), *value)
            }
            (Object::Hash { pairs }, _) => Evaluator::eval_hash_index_expression(pairs, index),
            _ => Err(anyhow!(
                "Index operator not supported for: {}",
                left.r#type()
            )),
        }
    }

    fn eval_array_index_expression(array: Vec<Object>, index: i64) -> anyhow::Result<Object> {
        let index: usize = index.try_into()?;
        let max = array.len();
        if index >= max {
            return Err(anyhow!("Index out of bounds"));
        }
        Ok(array[index].clone())
    }

    fn apply_function(func: Object, args: &[Object]) -> anyhow::Result<Object> {
        match func {
            Object::Function {
                parameters,
                body,
                env,
            } => {
                let mut extended_env = Evaluator::extend_function_env(&parameters, args, env);
                let evaluated = Evaluator::eval(Node::Expression(&body), &mut extended_env)?;
                Ok(Evaluator::unwrap_return_value(evaluated))
            }
            Object::BuiltIn { func } => func.run_builtin(args),
            e => Err(anyhow!(
                "Expected an Object::Function or Object::BuiltIn\nGot: {}",
                e
            )),
        }
    }

    fn extend_function_env(
        parameters: &[Expression],
        args: &[Object],
        env: Environment,
    ) -> Environment {
        let mut env = Environment::new(Some(Box::new(env)));
        for (i, param) in parameters.iter().enumerate() {
            if let Expression::Identifier { value, .. } = param {
                env.set(value, args[i].clone());
            }
        }
        env
    }

    fn unwrap_return_value(object: Object) -> Object {
        if let Object::ReturnValue { value } = object {
            return *value;
        }
        object
    }

    fn eval_string_infix_expression(
        operator: &[u8],
        left: &[u8],
        right: &[u8],
    ) -> anyhow::Result<Object> {
        if operator != b"+" {
            return Err(anyhow!(
                "STRING {} STRING",
                String::from_utf8_lossy(operator)
            ));
        }
        Ok(Object::String {
            value: [left, right].concat(),
        })
    }

    fn eval_hash_literal(
        mapped_expressions: &HashMap<Box<Expression>, Box<Expression>>,
        env: &mut Environment,
    ) -> anyhow::Result<Object> {
        let mut pairs: HashMap<Object, Object> = HashMap::new();
        for (k, v) in mapped_expressions {
            let key = Evaluator::eval(Node::Expression(k), env)?;
            let value = Evaluator::eval(Node::Expression(v), env)?;
            pairs.insert(key, value);
        }
        Ok(Object::Hash { pairs })
    }

    fn eval_hash_index_expression(
        hash: &HashMap<Object, Object>,
        index: Object,
    ) -> anyhow::Result<Object> {
        if let Object::Function { .. } = index {
            return Err(anyhow!("Unusable as hash key: FUNCTION"));
        };
        let pair = hash
            .get(&index)
            .ok_or(anyhow!("Couldn't find {} in {:?}", index, hash))?;
        Ok(pair.clone())
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

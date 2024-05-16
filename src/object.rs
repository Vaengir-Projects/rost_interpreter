use crate::ast::Expression;
use anyhow::anyhow;
use std::{collections::HashMap, fmt::Display};

pub trait ObjectTrait: Display {
    fn r#type(&self) -> String;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Integer {
        value: i64,
    },
    Boolean {
        value: bool,
    },
    ReturnValue {
        value: Box<Object>,
    },
    Function {
        parameters: Vec<Expression>,
        body: Expression,
        env: Environment,
    },
    String {},
    BuiltIn {},
    Null,
}

impl ObjectTrait for Object {
    fn r#type(&self) -> String {
        match self {
            Object::Integer { .. } => String::from("INTEGER"),
            Object::Boolean { .. } => String::from("BOOLEAN"),
            Object::ReturnValue { .. } => String::from("RETURN_VALUE"),
            Object::Function { .. } => String::from("FUNCTION"),
            Object::String {} => todo!(),
            Object::BuiltIn {} => todo!(),
            Object::Null => String::from("NULL"),
        }
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer { value } => write!(f, "{}", value),
            Object::Boolean { value } => write!(f, "{}", value),
            Object::ReturnValue { value } => write!(f, "{}", value),
            Object::Function {
                parameters, body, ..
            } => {
                let parameters = parameters
                    .iter()
                    .map(|p| {
                        if let Expression::Identifier { value, .. } = p {
                            value.to_string()
                        } else {
                            String::new()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "fn({}) {{\n  {}\n}}", parameters, body)
            }
            Object::String {} => todo!(),
            Object::BuiltIn {} => todo!(),
            Object::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new(outer: Option<Box<Environment>>) -> Environment {
        Environment {
            store: HashMap::new(),
            outer,
        }
    }

    pub fn get(&self, name: &str) -> anyhow::Result<Object> {
        match self.store.get(name) {
            Some(value) => Ok(value.clone()),
            None => {
                if self.outer.is_some() {
                    Ok(self.outer.as_ref().unwrap().get(name)?)
                } else {
                    Err(anyhow!("Couldn't find {}", name))
                }
            }
        }
    }

    pub fn set(&mut self, name: &str, val: Object) -> Object {
        self.store.insert(name.to_string(), val.clone());
        val
    }

    pub fn new_enclosed_environment(outer: &mut Environment) -> Environment {
        Environment::new(Some(Box::new(outer.clone())))
    }
}

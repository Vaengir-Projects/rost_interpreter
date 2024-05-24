use crate::{ast::Expression, builtin::BuiltInFunction};
use anyhow::anyhow;
use std::{collections::HashMap, fmt::Display, hash::Hash};

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
    String {
        value: Vec<u8>,
    },
    BuiltIn {
        func: BuiltInFunction,
    },
    Array {
        elements: Vec<Object>,
    },
    Hash {
        pairs: HashMap<Object, Object>,
    },
    Null,
}

impl ObjectTrait for Object {
    fn r#type(&self) -> String {
        match self {
            Object::Integer { .. } => String::from("INTEGER"),
            Object::Boolean { .. } => String::from("BOOLEAN"),
            Object::ReturnValue { .. } => String::from("RETURN_VALUE"),
            Object::Function { .. } => String::from("FUNCTION"),
            Object::String { .. } => String::from("STRING"),
            Object::BuiltIn { .. } => String::from("BUILTIN"),
            Object::Array { .. } => String::from("ARRAY"),
            Object::Hash { .. } => String::from("HASH"),
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
            Object::String { value } => write!(f, "{}", String::from_utf8_lossy(value)),
            Object::BuiltIn { .. } => write!(f, "builtin function"),
            Object::Array { elements } => {
                let elements = elements
                    .iter()
                    .map(|p| format!("{}", p))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{}]", elements)
            }
            Object::Hash { pairs } => {
                let pairs = pairs
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "{{{}}}", pairs)
            }
            Object::Null => write!(f, "null"),
        }
    }
}

impl Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Object::Integer { value } => value.hash(state),
            Object::Boolean { value } => value.hash(state),
            Object::ReturnValue { value } => value.hash(state),
            Object::Function {
                parameters,
                body,
                env,
            } => {
                parameters.hash(state);
                body.hash(state);
                env.hash(state);
            }
            Object::String { value } => value.hash(state),
            Object::BuiltIn { func } => func.hash(state),
            Object::Array { elements } => elements.hash(state),
            Object::Hash { pairs } => {
                for (k, v) in pairs {
                    k.hash(state);
                    v.hash(state);
                }
            }
            Object::Null => todo!(),
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

impl Hash for Environment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Environment { store, outer } => {
                for (k, v) in store {
                    k.hash(state);
                    v.hash(state);
                }
                outer.hash(state);
            }
        }
    }
}

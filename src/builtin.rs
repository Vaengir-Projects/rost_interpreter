use crate::object::Object;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BuiltInFunction {
    LenFunction,
    FirstFunction,
    LastFunction,
    RestFunction,
    PushFunction,
}

impl BuiltInFunction {
    pub fn is_builtin(name: &str) -> anyhow::Result<Object> {
        match name {
            "len" => Ok(Object::BuiltIn {
                func: BuiltInFunction::LenFunction,
            }),
            "first" => Ok(Object::BuiltIn {
                func: BuiltInFunction::FirstFunction,
            }),
            "last" => Ok(Object::BuiltIn {
                func: BuiltInFunction::LastFunction,
            }),
            "rest" => Ok(Object::BuiltIn {
                func: BuiltInFunction::RestFunction,
            }),
            "push" => Ok(Object::BuiltIn {
                func: BuiltInFunction::PushFunction,
            }),
            &_ => Err(anyhow!("Not a builtin function")),
        }
    }

    pub fn run_builtin(&self, args: &[Object]) -> anyhow::Result<Object> {
        match self {
            BuiltInFunction::LenFunction => {
                if args.len() != 1 {
                    return Err(anyhow!(
                        "Wrong number of arguments.\nExpected: 1\nGot: {}",
                        args.len()
                    ));
                }
                match &args[0] {
                    Object::String { value } => Ok(Object::Integer {
                        value: value.len() as i64,
                    }),
                    Object::Array { elements } => Ok(Object::Integer {
                        value: elements.len() as i64,
                    }),
                    e => Err(anyhow!(
                        "Wrong kind of argument.\nExpected: String || Array\nGot: {}",
                        e
                    )),
                }
            }
            BuiltInFunction::FirstFunction => {
                if args.len() != 1 {
                    return Err(anyhow!(
                        "Wrong number of arguments.\nExpected: 1\nGot: {}",
                        args.len()
                    ));
                }
                match &args[0] {
                    Object::Array { elements } => Ok(elements[0].clone()),
                    e => Err(anyhow!(
                        "Wrong kind of argument.\nExpected: Array\nGot: {}",
                        e
                    )),
                }
            }
            BuiltInFunction::LastFunction => {
                if args.len() != 1 {
                    return Err(anyhow!(
                        "Wrong number of arguments.\nExpected: 1\nGot: {}",
                        args.len()
                    ));
                }
                match &args[0] {
                    Object::Array { elements } => Ok(elements[elements.len() - 1].clone()),
                    e => Err(anyhow!(
                        "Wrong kind of argument.\nExpected: Array\nGot: {}",
                        e
                    )),
                }
            }
            BuiltInFunction::RestFunction => {
                if args.len() != 1 {
                    return Err(anyhow!(
                        "Wrong number of arguments.\nExpected: 1\nGot: {}",
                        args.len()
                    ));
                }
                match &args[0] {
                    Object::Array { elements } => {
                        if elements.is_empty() {
                            return Ok(Object::Array {
                                elements: elements[1..].to_vec(),
                            });
                        }
                        Err(anyhow!("Index out of bounds"))
                    }
                    e => Err(anyhow!(
                        "Wrong kind of argument.\nExpected: Array\nGot: {}",
                        e
                    )),
                }
            }
            BuiltInFunction::PushFunction => {
                if args.len() != 2 {
                    return Err(anyhow!(
                        "Wrong number of arguments.\nExpected: 1\nGot: {}",
                        args.len()
                    ));
                }
                match &args[0] {
                    Object::Array { elements } => {
                        let mut elements = elements.clone();
                        elements.push(args[1].clone());
                        Ok(Object::Array { elements })
                    }
                    e => Err(anyhow!(
                        "Wrong kind of argument.\nExpected: Array\nGot: {}",
                        e
                    )),
                }
            }
        }
    }
}

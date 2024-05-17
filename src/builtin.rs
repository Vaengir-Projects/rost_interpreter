use crate::object::Object;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BuiltInFunction {
    LenFunction,
}

impl BuiltInFunction {
    pub fn is_builtin(name: &str) -> anyhow::Result<Object> {
        match name {
            "len" => Ok(Object::BuiltIn {
                func: BuiltInFunction::LenFunction,
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
                    e => Err(anyhow!(
                        "Wrong kind of argument.\nExpected: String\nGot: {}",
                        e
                    )),
                }
            }
        }
    }
}

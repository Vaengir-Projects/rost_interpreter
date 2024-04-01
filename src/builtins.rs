use crate::ast::Identifier;
use crate::evaluator::EvaluationError;
use crate::object::{BuiltIn, Integer, Object, ObjectTrait};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuiltInFunction {
    LenFunction,
}

pub fn is_builtin(name: Identifier) -> Result<Object, EvaluationError> {
    match name.value.as_str() {
        "len" => Ok(Object::BuiltIn(BuiltIn {
            func: BuiltInFunction::LenFunction,
        })),
        &_ => Err(EvaluationError::IdentError(String::from(
            "Not a builtin function",
        ))),
    }
}

impl BuiltIn {
    pub fn run_builtin(&self, args: &[Object]) -> Result<Object, EvaluationError> {
        match self.func {
            BuiltInFunction::LenFunction => {
                if args.len() != 1 {
                    return Err(EvaluationError::BuiltInError(format!(
                        "Wrong number of arguments.\nExpected: 1\nGot: {}",
                        args.len()
                    )));
                }
                match &args[0] {
                    Object::String(s) => Ok(Object::Integer(Integer {
                        value: s.value.len() as i64,
                    })),
                    e => Err(EvaluationError::BuiltInError(format!(
                        "Wrong kind of argument.\nExpected: String\nGot: {}",
                        e.r#type()
                    ))),
                }
            }
        }
    }
}

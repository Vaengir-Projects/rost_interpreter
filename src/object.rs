use std::fmt::Display;

pub trait ObjectTrait: Display {
    fn r#type(&self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Integer { value: i64 },
    Boolean { value: bool },
    ReturnValue { value: Box<Object> },
    Function {},
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
            Object::Function {} => todo!(),
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
            Object::Function {} => todo!(),
            Object::String {} => todo!(),
            Object::BuiltIn {} => todo!(),
            Object::Null => write!(f, "null"),
        }
    }
}

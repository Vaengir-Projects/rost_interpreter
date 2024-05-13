use std::fmt::Display;

trait ObjectTrait: Display {
    fn r#type(&self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Integer { value: i64 },
    Boolean { value: bool },
    PrefixExpression {},
    ReturnValue {},
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
            Object::PrefixExpression {} => todo!(),
            Object::ReturnValue {} => todo!(),
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
            Object::PrefixExpression {} => todo!(),
            Object::ReturnValue {} => todo!(),
            Object::Function {} => todo!(),
            Object::String {} => todo!(),
            Object::BuiltIn {} => todo!(),
            Object::Null => write!(f, "null"),
        }
    }
}

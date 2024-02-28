use std::fmt::Display;

enum ObjectType {
    Integer,
    Boolean,
    Null,
}

trait ObjectTrait: Display {
    fn r#type() -> ObjectType;
}

struct Integer {
    value: i64,
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ObjectTrait for Integer {
    fn r#type() -> ObjectType {
        ObjectType::Integer
    }
}

struct Boolean {
    value: bool,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ObjectTrait for Boolean {
    fn r#type() -> ObjectType {
        ObjectType::Boolean
    }
}

struct Null {}

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl ObjectTrait for Null {
    fn r#type() -> ObjectType {
        ObjectType::Null
    }
}

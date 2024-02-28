use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    Null,
}

trait ObjectTrait: Display {
    fn r#type(&mut self) -> Object;
}

#[derive(Debug, Clone)]
pub struct Integer {
    pub value: i64,
}

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ObjectTrait for Integer {
    fn r#type(&mut self) -> Object {
        Object::Integer(Integer { value: self.value })
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ObjectTrait for Boolean {
    fn r#type(&mut self) -> Object {
        Object::Boolean(Boolean { value: self.value })
    }
}

#[derive(Debug, Clone)]
pub struct Null {}

impl Display for Null {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}

impl ObjectTrait for Null {
    fn r#type(&mut self) -> Object {
        Object::Null
    }
}

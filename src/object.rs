use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    PrefixExpression(PrefixExpression),
    ReturnValue(ReturnValue),
    Null,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::PrefixExpression(p) => write!(f, "{}", p),
            Object::ReturnValue(rv) => write!(f, "{}", rv),
            Object::Null => write!(f, "null"),
        }
    }
}

trait ObjectTrait: Display {
    fn r#type(&mut self) -> Object;
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixExpression {
    pub value: bool,
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ObjectTrait for PrefixExpression {
    fn r#type(&mut self) -> Object {
        Object::Boolean(Boolean { value: self.value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnValue {
    pub value: Box<Object>,
}

impl Display for ReturnValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ObjectTrait for ReturnValue {
    fn r#type(&mut self) -> Object {
        Object::ReturnValue(ReturnValue {
            value: self.value.clone(),
        })
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

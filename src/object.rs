use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Object {
    Integer(Integer),
    Boolean(Boolean),
    PrefixExpression(PrefixExpression),
    ReturnValue(ReturnValue),
    Null,
}

impl ObjectTrait for Object {
    fn r#type(&self) -> String {
        match self {
            Object::Integer(i) => i.r#type(),
            Object::Boolean(b) => b.r#type(),
            Object::PrefixExpression(p) => p.r#type(),
            Object::ReturnValue(rv) => rv.r#type(),
            Object::Null => panic!("See if this ever happens"),
        }
    }
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

pub trait ObjectTrait: Display {
    fn r#type(&self) -> String;
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
    fn r#type(&self) -> String {
        String::from("INTEGER")
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
    fn r#type(&self) -> String {
        String::from("BOOLEAN")
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
    fn r#type(&self) -> String {
        String::from("PREFIXEXPRESSION")
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
    fn r#type(&self) -> String {
        String::from("RETURNVALUE")
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
    fn r#type(&self) -> String {
        String::from("NULL")
    }
}

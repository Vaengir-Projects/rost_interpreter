#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn build(token_type: TokenType, literal: &str) -> Token {
        Token {
            r#type: token_type,
            literal: String::from(literal),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    EOF,
    Ident,
    INT,
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eq,
    NotEq,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

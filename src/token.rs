use anyhow::Context;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    EOF,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GreaterThan,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Eq,
    NotEq,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn build(token_type: TokenType, literal: &[u8]) -> anyhow::Result<Token> {
        Ok(Token {
            r#type: token_type,
            literal: String::from_utf8(literal.to_vec())
                .context("Couldn't convert u8 to String")?,
        })
    }
}

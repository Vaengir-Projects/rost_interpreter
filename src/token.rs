#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    EQ,
    NotEq,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

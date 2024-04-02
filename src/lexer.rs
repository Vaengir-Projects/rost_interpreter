use anyhow::anyhow;

use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    char: u8,
}

impl Lexer {
    pub fn new(input: &[u8]) -> Lexer {
        let mut lexer = Lexer {
            input: input.to_vec(),
            position: 0,
            read_position: 0,
            char: b'\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> anyhow::Result<Token> {
        let token = match self.char {
            b'=' => Ok(Token::build(TokenType::Assign, &[self.char])?),
            b';' => Ok(Token::build(TokenType::Semicolon, &[self.char])?),
            b'(' => Ok(Token::build(TokenType::LParen, &[self.char])?),
            b')' => Ok(Token::build(TokenType::RParen, &[self.char])?),
            b',' => Ok(Token::build(TokenType::Comma, &[self.char])?),
            b'+' => Ok(Token::build(TokenType::Plus, &[self.char])?),
            b'{' => Ok(Token::build(TokenType::LBrace, &[self.char])?),
            b'}' => Ok(Token::build(TokenType::RBrace, &[self.char])?),
            b'\0' => Ok(Token {
                r#type: TokenType::EOF,
                literal: String::from(""),
            }),
            _ => {
                if self.char.is_ascii_alphabetic() || self.char == b'_' {
                    let literal = self.read_identifier();
                }
                return Err(anyhow!(
                    "This char isn't implemented (yet): {}",
                    self.char as char
                ));
            }
        };
        self.read_char();
        token
    }
}

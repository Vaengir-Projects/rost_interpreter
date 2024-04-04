use crate::token::{Token, TokenType};
use anyhow::{anyhow, Context};
use std::collections::HashMap;

#[derive(Debug, Clone)]
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
            self.char = b'\0';
        } else {
            self.char = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> anyhow::Result<Token> {
        self.skip_whitespace();
        let token = match self.char {
            b'=' => {
                if self.peek_char() == b'=' {
                    let char = self.char;
                    self.read_char();
                    let literal = &[char, self.char];
                    Ok(Token::build(TokenType::Eq, literal)?)
                } else {
                    Ok(Token::build(TokenType::Assign, &[self.char])?)
                }
            }
            b';' => Ok(Token::build(TokenType::Semicolon, &[self.char])?),
            b'(' => Ok(Token::build(TokenType::LParen, &[self.char])?),
            b')' => Ok(Token::build(TokenType::RParen, &[self.char])?),
            b',' => Ok(Token::build(TokenType::Comma, &[self.char])?),
            b'+' => Ok(Token::build(TokenType::Plus, &[self.char])?),
            b'{' => Ok(Token::build(TokenType::LBrace, &[self.char])?),
            b'}' => Ok(Token::build(TokenType::RBrace, &[self.char])?),
            b'-' => Ok(Token::build(TokenType::Minus, &[self.char])?),
            b'!' => {
                if self.peek_char() == b'=' {
                    let char = self.char;
                    self.read_char();
                    let literal = &[char, self.char];
                    Ok(Token::build(TokenType::NotEq, literal)?)
                } else {
                    Ok(Token::build(TokenType::Bang, &[self.char])?)
                }
            }
            b'*' => Ok(Token::build(TokenType::Asterisk, &[self.char])?),
            b'/' => Ok(Token::build(TokenType::Slash, &[self.char])?),
            b'<' => Ok(Token::build(TokenType::LessThan, &[self.char])?),
            b'>' => Ok(Token::build(TokenType::GreaterThan, &[self.char])?),
            b'\0' => Ok(Token {
                r#type: TokenType::EOF,
                literal: String::from(""),
            }),
            _ => {
                if self.char.is_ascii_alphabetic() || self.char == b'_' {
                    let literal = self.read_identifier()?;
                    let token_type = self.look_up_ident(&literal);
                    return Token::build(token_type, literal.as_bytes());
                } else if self.char.is_ascii_digit() {
                    let literal = self.read_number()?;
                    let token_type = TokenType::Int;
                    return Token::build(token_type, literal.as_bytes());
                }
                return Err(anyhow!(
                    "This char isn't implemented (yet): '{}'",
                    self.char as char
                ));
            }
        };
        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> anyhow::Result<String> {
        let position = self.position;
        while self.char.is_ascii_alphabetic() || self.char == b'_' {
            self.read_char();
        }
        String::from_utf8(self.input[position..self.position].to_vec())
            .context("Couldn't build a String from the given Bytes")
    }

    fn read_number(&mut self) -> anyhow::Result<String> {
        let position = self.position;
        while self.char.is_ascii_digit() {
            self.read_char();
        }
        String::from_utf8(self.input[position..self.position].to_vec())
            .context("Couldn't build a String from the given Bytes")
    }

    fn look_up_ident(&self, ident: &str) -> TokenType {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert(String::from("fn"), TokenType::Function);
        keywords.insert(String::from("let"), TokenType::Let);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("return"), TokenType::Return);

        if let Some(token_type) = keywords.get(ident) {
            return token_type.clone();
        }
        TokenType::Ident
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return b'\0';
        }
        self.input[self.read_position]
    }
}

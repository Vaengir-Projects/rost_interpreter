use std::collections::HashMap;

use crate::token::{Token, TokenType};

#[derive(Debug, PartialEq)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    char: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let input: Vec<char> = input.chars().collect();
        let mut lexer = Lexer {
            input: input.clone(),
            position: 0,
            read_position: 0,
            char: '\0',
        };
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = '\0';
        } else {
            self.char = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.char.is_alphabetic() || self.char == '_' {
            self.read_char();
        }
        self.input.iter().collect::<String>()[position..self.position].to_string()
    }

    fn loop_up_ident(&self, ident: &str) -> TokenType {
        let mut keywords: HashMap<String, TokenType> = HashMap::new();
        keywords.insert(String::from("fn"), TokenType::FUNCTION);
        keywords.insert(String::from("let"), TokenType::LET);

        if let Some(token_type) = keywords.get(ident) {
            return token_type.clone();
        }
        TokenType::IDENT
    }

    fn eat_whitespace(&mut self) {
        while self.char == ' ' || self.char == '\t' || self.char == '\n' || self.char == '\r' {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.char.is_digit(10) {
            self.read_char();
        }
        self.input.iter().collect::<String>()[position..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        let token = match self.char {
            '=' => Token::build(TokenType::ASSIGN, &self.char.to_string()),
            ';' => Token::build(TokenType::SEMICOLON, &self.char.to_string()),
            '(' => Token::build(TokenType::LPAREN, &self.char.to_string()),
            ')' => Token::build(TokenType::RPAREN, &self.char.to_string()),
            ',' => Token::build(TokenType::COMMA, &self.char.to_string()),
            '+' => Token::build(TokenType::PLUS, &self.char.to_string()),
            '{' => Token::build(TokenType::LBRACE, &self.char.to_string()),
            '}' => Token::build(TokenType::RBRACE, &self.char.to_string()),
            '\0' => Token::build(TokenType::EOF, ""),
            _ => {
                if self.char.is_alphabetic() || self.char == '_' {
                    let literal = self.read_identifier();
                    return Token::build(self.loop_up_ident(&literal), &literal);
                } else if self.char.is_digit(10) {
                    return Token::build(TokenType::INT, &self.read_number());
                } else {
                    Token::build(TokenType::ILLEGAL, &self.char.to_string())
                }
            }
        };
        self.read_char();
        token
    }
}

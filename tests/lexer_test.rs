#[cfg(test)]
mod tests {
    use rost_interpreter::lexer::Lexer;
    use rost_interpreter::token::Token;
    use rost_interpreter::token::TokenType;

    #[test]
    fn test_lexer_symbols() {
        let input: &str = "=+(){},;";
        let expected_tokens: Vec<Token> = vec![
            Token {
                r#type: TokenType::ASSIGN,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::PLUS,
                literal: String::from('+'),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: String::from('('),
            },
            Token {
                r#type: TokenType::RPAREN,
                literal: String::from(')'),
            },
            Token {
                r#type: TokenType::LBRACE,
                literal: String::from('{'),
            },
            Token {
                r#type: TokenType::RBRACE,
                literal: String::from('}'),
            },
            Token {
                r#type: TokenType::COMMA,
                literal: String::from(','),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::EOF,
                literal: String::from(""),
            },
        ];
        let mut lexer = Lexer::new(input);
        for token in expected_tokens {
            let tok = lexer.next_token();
            assert_eq!(tok, token);
        }
    }

    #[test]
    fn test_lexer_source_code() {
        let input = "\
let five = 5;
let ten = 10;

let add = fn(x,y) {
    x + y;
};

let result = add(five, ten);";

        let expected_tokens: Vec<Token> = vec![
            Token {
                r#type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("five"),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::INT,
                literal: String::from("5"),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("ten"),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::INT,
                literal: String::from("10"),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("add"),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::FUNCTION,
                literal: String::from("fn"),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: String::from('('),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("x"),
            },
            Token {
                r#type: TokenType::COMMA,
                literal: String::from(','),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("y"),
            },
            Token {
                r#type: TokenType::RPAREN,
                literal: String::from(')'),
            },
            Token {
                r#type: TokenType::LBRACE,
                literal: String::from('{'),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("x"),
            },
            Token {
                r#type: TokenType::PLUS,
                literal: String::from('+'),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("y"),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::RBRACE,
                literal: String::from('}'),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::LET,
                literal: String::from("let"),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("result"),
            },
            Token {
                r#type: TokenType::ASSIGN,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("add"),
            },
            Token {
                r#type: TokenType::LPAREN,
                literal: String::from('('),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("five"),
            },
            Token {
                r#type: TokenType::COMMA,
                literal: String::from(','),
            },
            Token {
                r#type: TokenType::IDENT,
                literal: String::from("ten"),
            },
            Token {
                r#type: TokenType::RPAREN,
                literal: String::from(')'),
            },
            Token {
                r#type: TokenType::SEMICOLON,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::EOF,
                literal: String::from(""),
            },
        ];
        let mut lexer = Lexer::new(input);
        for token in expected_tokens {
            let tok = lexer.next_token();
            dbg!(&tok);
            assert_eq!(tok, token);
        }
    }
}

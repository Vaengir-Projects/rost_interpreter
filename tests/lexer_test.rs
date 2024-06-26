#[cfg(test)]
mod tests {
    use rost_interpreter::{
        lexer::Lexer,
        token::{Token, TokenType},
    };

    #[test]
    fn test_lexer_symbols() {
        let input: &str = "=+(){},;!-/*<>";
        let expected_tokens: Vec<Token> = vec![
            Token {
                r#type: TokenType::Assign,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::Plus,
                literal: String::from('+'),
            },
            Token {
                r#type: TokenType::LParen,
                literal: String::from('('),
            },
            Token {
                r#type: TokenType::RParen,
                literal: String::from(')'),
            },
            Token {
                r#type: TokenType::LBrace,
                literal: String::from('{'),
            },
            Token {
                r#type: TokenType::RBrace,
                literal: String::from('}'),
            },
            Token {
                r#type: TokenType::Comma,
                literal: String::from(','),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::Bang,
                literal: String::from('!'),
            },
            Token {
                r#type: TokenType::Minus,
                literal: String::from('-'),
            },
            Token {
                r#type: TokenType::Slash,
                literal: String::from('/'),
            },
            Token {
                r#type: TokenType::Asterisk,
                literal: String::from('*'),
            },
            Token {
                r#type: TokenType::LT,
                literal: String::from('<'),
            },
            Token {
                r#type: TokenType::GT,
                literal: String::from('>'),
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
        let input = r#"let five = 5;
let ten = 10;

let add = fn(x,y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
"foobar"
"foo bar"
[1,2];"#;

        let expected_tokens: Vec<Token> = vec![
            Token {
                r#type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                r#type: TokenType::Assign,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("ten"),
            },
            Token {
                r#type: TokenType::Assign,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("add"),
            },
            Token {
                r#type: TokenType::Assign,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::Function,
                literal: String::from("fn"),
            },
            Token {
                r#type: TokenType::LParen,
                literal: String::from('('),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("x"),
            },
            Token {
                r#type: TokenType::Comma,
                literal: String::from(','),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("y"),
            },
            Token {
                r#type: TokenType::RParen,
                literal: String::from(')'),
            },
            Token {
                r#type: TokenType::LBrace,
                literal: String::from('{'),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("x"),
            },
            Token {
                r#type: TokenType::Plus,
                literal: String::from('+'),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("y"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::RBrace,
                literal: String::from('}'),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::Let,
                literal: String::from("let"),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("result"),
            },
            Token {
                r#type: TokenType::Assign,
                literal: String::from('='),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("add"),
            },
            Token {
                r#type: TokenType::LParen,
                literal: String::from('('),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("five"),
            },
            Token {
                r#type: TokenType::Comma,
                literal: String::from(','),
            },
            Token {
                r#type: TokenType::Ident,
                literal: String::from("ten"),
            },
            Token {
                r#type: TokenType::RParen,
                literal: String::from(')'),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::Bang,
                literal: String::from('!'),
            },
            Token {
                r#type: TokenType::Minus,
                literal: String::from('-'),
            },
            Token {
                r#type: TokenType::Slash,
                literal: String::from('/'),
            },
            Token {
                r#type: TokenType::Asterisk,
                literal: String::from('*'),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                r#type: TokenType::LT,
                literal: String::from('<'),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                r#type: TokenType::GT,
                literal: String::from('>'),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::If,
                literal: String::from("if"),
            },
            Token {
                r#type: TokenType::LParen,
                literal: String::from('('),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("5"),
            },
            Token {
                r#type: TokenType::LT,
                literal: String::from('<'),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                r#type: TokenType::RParen,
                literal: String::from(')'),
            },
            Token {
                r#type: TokenType::LBrace,
                literal: String::from('{'),
            },
            Token {
                r#type: TokenType::Return,
                literal: String::from("return"),
            },
            Token {
                r#type: TokenType::True,
                literal: String::from("true"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::RBrace,
                literal: String::from('}'),
            },
            Token {
                r#type: TokenType::Else,
                literal: String::from("else"),
            },
            Token {
                r#type: TokenType::LBrace,
                literal: String::from('{'),
            },
            Token {
                r#type: TokenType::Return,
                literal: String::from("return"),
            },
            Token {
                r#type: TokenType::False,
                literal: String::from("false"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::RBrace,
                literal: String::from('}'),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                r#type: TokenType::Eq,
                literal: String::from("=="),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("10"),
            },
            Token {
                r#type: TokenType::NotEq,
                literal: String::from("!="),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("9"),
            },
            Token {
                r#type: TokenType::Semicolon,
                literal: String::from(';'),
            },
            Token {
                r#type: TokenType::String,
                literal: String::from("foobar"),
            },
            Token {
                r#type: TokenType::String,
                literal: String::from("foo bar"),
            },
            Token {
                r#type: TokenType::LBracket,
                literal: String::from('['),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("1"),
            },
            Token {
                r#type: TokenType::Comma,
                literal: String::from(','),
            },
            Token {
                r#type: TokenType::Int,
                literal: String::from("2"),
            },
            Token {
                r#type: TokenType::RBracket,
                literal: String::from(']'),
            },
            Token {
                r#type: TokenType::Semicolon,
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

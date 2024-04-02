use rost_interpreter::{lexer::Lexer, token::TokenType};

#[test]
fn test_next_type() {
    let input = b"=+(){},;";
    #[derive(Debug)]
    struct Test {
        expected_type: TokenType,
        expected_literal: String,
    }
    let tests: Vec<Test> = vec![
        Test {
            expected_type: TokenType::Assign,
            expected_literal: String::from("="),
        },
        Test {
            expected_type: TokenType::Plus,
            expected_literal: String::from("+"),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: String::from("("),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: String::from(")"),
        },
        Test {
            expected_type: TokenType::LBrace,
            expected_literal: String::from("{"),
        },
        Test {
            expected_type: TokenType::RBrace,
            expected_literal: String::from("}"),
        },
        Test {
            expected_type: TokenType::Comma,
            expected_literal: String::from(","),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: String::from(";"),
        },
        Test {
            expected_type: TokenType::EOF,
            expected_literal: String::from(""),
        },
    ];
    let mut lexer = Lexer::new(input);
    for test in tests {
        let tok = lexer.next_token().unwrap();
        dbg!(&tok);
        assert_eq!(tok.r#type, test.expected_type);
        assert_eq!(tok.literal, test.expected_literal);
    }
}

#[test]
fn test_next_token() {
    let input = b"let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);";
    struct Test {
        expected_type: TokenType,
        expected_literal: String,
    }
    let tests: Vec<Test> = vec![
        Test {
            expected_type: TokenType::Let,
            expected_literal: String::from("let"),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("five"),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: String::from('='),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: String::from(';'),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: String::from("let"),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("ten"),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: String::from('='),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: String::from("10"),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: String::from(';'),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: String::from("let"),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("add"),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: String::from('='),
        },
        Test {
            expected_type: TokenType::Function,
            expected_literal: String::from("fn"),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: String::from('('),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("x"),
        },
        Test {
            expected_type: TokenType::Comma,
            expected_literal: String::from(','),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("y"),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: String::from(')'),
        },
        Test {
            expected_type: TokenType::LBrace,
            expected_literal: String::from('{'),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("x"),
        },
        Test {
            expected_type: TokenType::Plus,
            expected_literal: String::from('+'),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("y"),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: String::from(';'),
        },
        Test {
            expected_type: TokenType::RBrace,
            expected_literal: String::from('}'),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: String::from(';'),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: String::from("let"),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("result"),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: String::from('='),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("add"),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: String::from('('),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("five"),
        },
        Test {
            expected_type: TokenType::Comma,
            expected_literal: String::from(','),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: String::from("ten"),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: String::from(')'),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: String::from(';'),
        },
        Test {
            expected_type: TokenType::EOF,
            expected_literal: String::from(""),
        },
    ];
    let mut lexer = Lexer::new(input);
    for test in tests {
        let tok = lexer.next_token().unwrap();
        dbg!(&tok);
        assert_eq!(tok.r#type, test.expected_type);
        assert_eq!(tok.literal, test.expected_literal);
    }
}

use rost_interpreter::{lexer::Lexer, token::TokenType};

#[test]
fn next_type() {
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
fn next_token() {
    let input: &[u8] = b"let five = 5;
let ten = 10;
let add = fn(x, y) {
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
\"foobar\"
\"foo bar\"
";
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
            expected_type: TokenType::Bang,
            expected_literal: String::from('!'),
        },
        Test {
            expected_type: TokenType::Minus,
            expected_literal: String::from('-'),
        },
        Test {
            expected_type: TokenType::Slash,
            expected_literal: String::from('/'),
        },
        Test {
            expected_type: TokenType::Asterisk,
            expected_literal: String::from('*'),
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
            expected_type: TokenType::Int,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: TokenType::LessThan,
            expected_literal: String::from('<'),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: String::from("10"),
        },
        Test {
            expected_type: TokenType::GreaterThan,
            expected_literal: String::from('>'),
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
            expected_type: TokenType::If,
            expected_literal: String::from("if"),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: String::from('('),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: String::from("5"),
        },
        Test {
            expected_type: TokenType::LessThan,
            expected_literal: String::from('<'),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: String::from("10"),
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
            expected_type: TokenType::Return,
            expected_literal: String::from("return"),
        },
        Test {
            expected_type: TokenType::True,
            expected_literal: String::from("true"),
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
            expected_type: TokenType::Else,
            expected_literal: String::from("else"),
        },
        Test {
            expected_type: TokenType::LBrace,
            expected_literal: String::from('{'),
        },
        Test {
            expected_type: TokenType::Return,
            expected_literal: String::from("return"),
        },
        Test {
            expected_type: TokenType::False,
            expected_literal: String::from("false"),
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
            expected_type: TokenType::Int,
            expected_literal: String::from("10"),
        },
        Test {
            expected_type: TokenType::Eq,
            expected_literal: String::from("=="),
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
            expected_type: TokenType::Int,
            expected_literal: String::from("10"),
        },
        Test {
            expected_type: TokenType::NotEq,
            expected_literal: String::from("!="),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: String::from("9"),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: String::from(';'),
        },
        Test {
            expected_type: TokenType::String,
            expected_literal: String::from("foobar"),
        },
        Test {
            expected_type: TokenType::String,
            expected_literal: String::from("foo bar"),
        },
        // Test {
        //     expected_type: TokenType::LBracket,
        //     expected_literal: String::from('['),
        // },
        // Test {
        //     expected_type: TokenType::Int,
        //     expected_literal: String::from("1"),
        // },
        // Test {
        //     expected_type: TokenType::Comma,
        //     expected_literal: String::from(','),
        // },
        // Test {
        //     expected_type: TokenType::Int,
        //     expected_literal: String::from("2"),
        // },
        // Test {
        //     expected_type: TokenType::RBracket,
        //     expected_literal: String::from(']'),
        // },
        // Test {
        //     expected_type: TokenType::Semicolon,
        //     expected_literal: String::from(';'),
        // },
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

use rost_interpreter::ast::{Identifier, LetStatement, Program};
use rost_interpreter::token::{Token, TokenType};

#[test]
fn test_display() {
    let program = Program {
        statements: vec![Box::new(LetStatement {
            token: Token {
                r#type: TokenType::Let,
                literal: String::from("let"),
            },
            name: Identifier {
                token: Token {
                    r#type: TokenType::Ident,
                    literal: String::from("myVar"),
                },
                value: String::from("myVar"),
            },
            value: Box::new(Identifier {
                token: Token {
                    r#type: TokenType::Ident,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            }),
        })],
    };
    assert_eq!(format!("{}", program), format!("let myVar = anotherVar;"));
}

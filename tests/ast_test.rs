use rost_interpreter::{
    ast::{Expression, Program, Statement},
    token::{Token, TokenType},
};

#[test]
fn display() {
    let program = Program {
        statements: vec![Statement::Let {
            token: Token {
                r#type: TokenType::Let,
                literal: String::from("let"),
            },
            name: Expression::Identifier {
                token: Token {
                    r#type: TokenType::Ident,
                    literal: String::from("myVar"),
                },
                value: String::from("myVar"),
            },
            value: Expression::Identifier {
                token: Token {
                    r#type: TokenType::Ident,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            },
        }],
    };
    assert_eq!(format!("{}", program), format!("let myVar = anotherVar;"));
}

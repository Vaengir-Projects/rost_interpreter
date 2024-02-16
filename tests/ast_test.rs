#[cfg(test)]
mod tests {
    use rost_interpreter::{
        ast::{Expression, Identifier, LetStatement, Program, Statement},
        token::{Token, TokenType},
    };

    #[test]
    fn test_display_trait() {
        let program = Program {
            statements: vec![Statement::Let(LetStatement {
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
                value: Expression::Identifier(Identifier {
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
}

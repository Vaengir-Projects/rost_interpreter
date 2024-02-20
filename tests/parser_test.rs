#[cfg(test)]
mod tests {
    use rost_interpreter::{
        ast::{Expression, NodeTrait, Program, Statement},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_let_statements() {
        let input: &str = "\
let x = 5;
let y = 10;
let foobar = 838383;";

        let expected_identifiers: Vec<String> =
            vec![String::from("x"), String::from("y"), String::from("foobar")];

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program: Program = parser.parse_program();
        if program.statements.len() != 3 {
            panic!(
                "Program.Statements doesn't contain 3 statements. Got: {}",
                program
            );
        }
        for (i, ident) in expected_identifiers.iter().enumerate() {
            let statement = &program.statements[i];
            if !test_let_statement(statement, ident) {
                break;
            }
        }
    }

    fn test_let_statement(statement: &Statement, name: &str) -> bool {
        if statement.token_literal() != "let" {
            panic!(
                "statement.token_literal not 'let'. Got {}",
                statement.token_literal()
            );
        }

        let let_statement = match statement {
            Statement::Let(let_statement) => let_statement,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Let\nGot: {}",
                e
            ),
        };

        if let_statement.name.value != name {
            panic!(
                "Expected name value doesn't match:\nExpected: {}\nGot: {}",
                let_statement.name.value, name
            );
        }
        if let_statement.name.token_literal() != name {
            panic!(
                "Expected name value doesn't match:\nExpected: {}\nGot: {}",
                let_statement.name.token_literal(),
                name
            );
        }
        true
    }

    #[test]
    fn test_return_statement() {
        let input: &str = "\
return 5;
return 10;
return 993322;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if program.statements.len() != 3 {
            panic!(
                "Program.Statements doesn't contain 3 statements. Got: {}",
                program
            );
        }
        for statement in program.statements {
            let return_statement = match statement {
                Statement::Return(return_statement) => return_statement,
                e => panic!(
                    "Not the right kind of Statement. Expected: Statement::Return\nGot: {}",
                    e
                ),
            };
            if return_statement.token_literal() != "return" {
                panic!(
                    "statement.token_literal not 'return'. Got {}",
                    return_statement.token_literal()
                );
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if program.statements.len() != 1 {
            panic!(
                "Program.Statements doesn't contain 1 statement. Got: {}",
                program
            );
        }
        let expression_statement = match &program.statements[0] {
            Statement::Expression(es) => es,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                e
            ),
        };
        let identifier = match &expression_statement.expression {
            Expression::Identifier(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                e
            ),
        };
        assert_eq!(identifier.value, String::from("foobar"));
        assert_eq!(identifier.token_literal(), String::from("foobar"));
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if program.statements.len() != 1 {
            panic!(
                "Program.Statements doesn't contain 1 statement. Got: {}",
                program
            );
        }
        let int_statement = match &program.statements[0] {
            Statement::Expression(is) => is,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                e
            ),
        };
        let int = match &int_statement.expression {
            Expression::IntegerLiteral(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::IntegerLiteral\nGot: {}",
                e
            ),
        };
        assert_eq!(int.value, 5);
        assert_eq!(int.token_literal(), String::from("5"));
    }

    #[test]
    fn test_prefix_expression() {
        #[derive(Debug)]
        struct PrefixTest {
            input: String,
            operator: String,
            integer_value: i64,
        }

        let prefix_tests: Vec<PrefixTest> = vec![
            PrefixTest {
                input: String::from("!5;"),
                operator: String::from("!"),
                integer_value: 5,
            },
            PrefixTest {
                input: String::from("-15;"),
                operator: String::from("-"),
                integer_value: 15,
            },
        ];

        for prefix_test in prefix_tests {
            let lexer = Lexer::new(&prefix_test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if program.statements.len() != 1 {
                panic!(
                    "Program.Statements doesn't contain 1 statement. Got: {}",
                    program
                );
            }
            let prefix_statement = match &program.statements[0] {
                Statement::Expression(ps) => ps,
                e => panic!(
                    "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                    e
                ),
            };
            dbg!(&prefix_statement, &prefix_test);
            let prefix = match &prefix_statement.expression {
                Expression::PrefixExpression(p) => p,
                e => panic!(
                    "Not the right kind of Expression. Expected: Expression::PrefixExpression\nGot: {}",
                    e
                ),
            };
            assert_eq!(prefix.operator, prefix_test.operator);
            assert!(test_integer_literal(
                *prefix.right.clone(),
                prefix_test.integer_value
            ));
        }
    }

    #[test]
    fn test_infix_expression() {
        #[derive(Debug)]
        struct InfixTest {
            input: String,
            left_value: i64,
            operator: String,
            right_value: i64,
        }

        let infix_tests: Vec<InfixTest> = vec![
            InfixTest {
                input: String::from("5 + 5;"),
                left_value: 5,
                operator: String::from("+"),
                right_value: 5,
            },
            InfixTest {
                input: String::from("5 - 5;"),
                left_value: 5,
                operator: String::from("-"),
                right_value: 5,
            },
            InfixTest {
                input: String::from("5 * 5;"),
                left_value: 5,
                operator: String::from("*"),
                right_value: 5,
            },
            InfixTest {
                input: String::from("5 / 5;"),
                left_value: 5,
                operator: String::from("/"),
                right_value: 5,
            },
            InfixTest {
                input: String::from("5 > 5;"),
                left_value: 5,
                operator: String::from(">"),
                right_value: 5,
            },
            InfixTest {
                input: String::from("5 < 5;"),
                left_value: 5,
                operator: String::from("<"),
                right_value: 5,
            },
            InfixTest {
                input: String::from("5 == 5;"),
                left_value: 5,
                operator: String::from("=="),
                right_value: 5,
            },
            InfixTest {
                input: String::from("5 != 5;"),
                left_value: 5,
                operator: String::from("!="),
                right_value: 5,
            },
        ];

        for infix_test in infix_tests {
            let lexer = Lexer::new(&infix_test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            if program.statements.len() != 1 {
                panic!(
                    "Program.Statements doesn't contain 1 statement. Got: {}",
                    program
                );
            }
            let infix_statement = match &program.statements[0] {
                Statement::Expression(ps) => ps,
                e => panic!(
                    "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                    e
                ),
            };
            dbg!(&infix_statement, &infix_test);
            let infix = match &infix_statement.expression {
                Expression::InfixExpression(i) => i,
                e => panic!(
                    "Not the right kind of Expression. Expected: Expression::InfixExpression\nGot: {}",
                    e
                ),
            };
            assert!(test_integer_literal(
                *infix.left.clone(),
                infix_test.left_value
            ));
            assert_eq!(infix.operator, infix_test.operator);
            assert!(test_integer_literal(
                *infix.right.clone(),
                infix_test.right_value
            ));
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        struct Test {
            input: String,
            expected: String,
        }
        let tests: Vec<Test> = vec![
            Test {
                input: String::from("-a * b"),
                expected: String::from("((-a) * b)"),
            },
            Test {
                input: String::from("!-a"),
                expected: String::from("(!(-a))"),
            },
            Test {
                input: String::from("a + b + c"),
                expected: String::from("((a + b) + c)"),
            },
            Test {
                input: String::from("a + b - c"),
                expected: String::from("((a + b) - c)"),
            },
            Test {
                input: String::from("a * b * c"),
                expected: String::from("((a * b) * c)"),
            },
            Test {
                input: String::from("a * b / c"),
                expected: String::from("((a * b) / c)"),
            },
            Test {
                input: String::from("a + b / c"),
                expected: String::from("(a + (b / c))"),
            },
            Test {
                input: String::from("a + b * c + d / e - f"),
                expected: String::from("(((a + (b * c)) + (d / e)) - f)"),
            },
            Test {
                input: String::from("3 + 4; -5 * 5"),
                expected: String::from("(3 + 4)((-5) * 5)"),
            },
            Test {
                input: String::from("5 > 4 == 3 < 4"),
                expected: String::from("((5 > 4) == (3 < 4))"),
            },
            Test {
                input: String::from("5 < 4 != 3 > 4"),
                expected: String::from("((5 < 4) != (3 > 4))"),
            },
            Test {
                input: String::from("3 + 4 * 5 == 3 * 1 + 4 * 5"),
                expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
            },
            Test {
                input: String::from("true"),
                expected: String::from("true"),
            },
            Test {
                input: String::from("false"),
                expected: String::from("false"),
            },
            Test {
                input: String::from("3 > 5 == false"),
                expected: String::from("((3 > 5) == false)"),
            },
            Test {
                input: String::from("3 < 5 == true"),
                expected: String::from("((3 < 5) == true)"),
            },
        ];

        for test in tests {
            let lexer = Lexer::new(&test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            dbg!(&program);
            assert_eq!(format!("{}", program), test.expected);
        }
    }

    fn test_integer_literal(integer_literal: Expression, value: i64) -> bool {
        let int = match &integer_literal {
            Expression::IntegerLiteral(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::IntegerLiteral\nGot: {}",
                e
            ),
        };
        if int.value != value || int.token_literal() != format!("{}", value) {
            return false;
        }
        true
    }

    #[test]
    fn test_boolean_expression() {
        struct Test {
            input: String,
            expected: bool,
        }
        let tests: Vec<Test> = vec![
            Test {
                input: String::from("true;"),
                expected: true,
            },
            Test {
                input: String::from("false;"),
                expected: false,
            },
        ];

        for test in tests {
            let lexer = Lexer::new(&test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            dbg!(&program);
            if program.statements.len() != 1 {
                panic!(
                    "Program.Statements doesn't contain 1 statement. Got: {}",
                    program
                );
            }
            let bool_expression = match &program.statements[0] {
                Statement::Expression(b) => b,
                e => panic!(
                    "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                    e
                ),
            };
            let bool = match &bool_expression.expression {
                Expression::Boolean(b) => b,
                e => panic!(
                    "Not the right kind of Expression. Expected: Expression::Boolean\nGot: {}",
                    e
                ),
            };
            assert_eq!(bool.value, test.expected);
        }
    }
}

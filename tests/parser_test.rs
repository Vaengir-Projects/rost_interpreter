#[cfg(test)]
mod tests {
    use rost_interpreter::{
        ast::{Expression, ExpressionStatement, NodeTrait, Program, Statement},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_let_statements() {
        #[derive(Debug)]
        enum Used {
            Int(i64),
            Bool(bool),
            String(String),
        }
        #[derive(Debug)]
        struct Test {
            input: String,
            expected_identifier: String,
            expected_value: Used,
        }
        let tests: Vec<Test> = vec![
            Test {
                input: String::from("let x = 5;"),
                expected_identifier: String::from("x"),
                expected_value: Used::Int(5),
            },
            Test {
                input: String::from("let y = true;"),
                expected_identifier: String::from("y"),
                expected_value: Used::Bool(true),
            },
            Test {
                input: String::from("let foobar = y;"),
                expected_identifier: String::from("foobar"),
                expected_value: Used::String(String::from("y")),
            },
        ];
        for test in tests {
            let lexer = Lexer::new(&test.input);
            let mut parser = Parser::new(lexer);
            let program: Program = parser.parse_program().unwrap();
            if program.statements.len() != 1 {
                panic!(
                    "Program.Statements doesn't contain 1 statements. Got: {}",
                    program
                );
            }
            let statement = match &program.statements[0] {
                Statement::Let(l) => l,
                e => panic!(
                    "Not the right kind of Statement. Expected: Statement::Let\nGot: {}",
                    e
                ),
            };
            assert_eq!(statement.name.value, test.expected_identifier);
            assert_eq!(statement.name.token_literal(), test.expected_identifier);
            dbg!(&statement.value);
            match test.expected_value {
                Used::Int(i) => {
                    let return_value = match &statement.value {
                        Expression::IntegerLiteral(i) => i,
                        e => panic!(
                            "Not the right kind of Expression. Expected: Expression::IntegerLiteral\nGot: {}",
                            e
                        ),
                    };
                    assert_eq!(return_value.value, i)
                }
                Used::Bool(b) => {
                    let return_value = match &statement.value {
                        Expression::Boolean(b) => b,
                        e => panic!(
                            "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                            e
                        ),
                    };
                    assert_eq!(return_value.value, b)
                }
                Used::String(s) => {
                    let return_value = match &statement.value {
                        Expression::Identifier(i) => i,
                        e => panic!(
                            "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                            e
                        ),
                    };
                    assert_eq!(return_value.value, s)
                }
            }
        }
    }

    #[test]
    fn test_return_statement() {
        #[derive(Debug)]
        enum Used {
            Int(i64),
            Bool(bool),
            String(String),
        }
        #[derive(Debug)]
        struct Test {
            input: String,
            expected_value: Used,
        }
        let tests: Vec<Test> = vec![
            Test {
                input: String::from("return 5;"),
                expected_value: Used::Int(5),
            },
            Test {
                input: String::from("return true;"),
                expected_value: Used::Bool(true),
            },
            Test {
                input: String::from("return foobar;"),
                expected_value: Used::String(String::from("foobar")),
            },
        ];
        for test in tests {
            let lexer = Lexer::new(&test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program().unwrap();
            if program.statements.len() != 1 {
                panic!(
                    "Program.Statements doesn't contain 1 statements. Got: {}",
                    program
                );
            }
            let return_statement = match &program.statements[0] {
                Statement::Return(r) => r,
                e => panic!(
                    "Not the right kind of Statement. Expected: Statement::Return\nGot: {}",
                    e
                ),
            };
            match test.expected_value {
                Used::Int(i) => {
                    let return_value = match &return_statement.return_value {
                        Expression::IntegerLiteral(i) => i,
                        e => panic!(
                            "Not the right kind of Expression. Expected: Expression::IntegerLiteral\nGot: {}",
                            e
                        ),
                    };
                    assert_eq!(return_value.value, i)
                }
                Used::Bool(b) => {
                    let return_value = match &return_statement.return_value {
                        Expression::Boolean(b) => b,
                        e => panic!(
                            "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                            e
                        ),
                    };
                    assert_eq!(return_value.value, b)
                }
                Used::String(s) => {
                    let return_value = match &return_statement.return_value {
                        Expression::Identifier(i) => i,
                        e => panic!(
                            "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                            e
                        ),
                    };
                    assert_eq!(return_value.value, s)
                }
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
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
        let program = parser.parse_program().unwrap();
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
            let program = parser.parse_program().unwrap();
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
            let program = parser.parse_program().unwrap();
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
            Test {
                input: String::from("1 + (2 + 3) + 4"),
                expected: String::from("((1 + (2 + 3)) + 4)"),
            },
            Test {
                input: String::from("(5 + 5) * 2"),
                expected: String::from("((5 + 5) * 2)"),
            },
            Test {
                input: String::from("2 / (5 + 5)"),
                expected: String::from("(2 / (5 + 5))"),
            },
            Test {
                input: String::from("-(5 + 5)"),
                expected: String::from("(-(5 + 5))"),
            },
            Test {
                input: String::from("!(true == true)"),
                expected: String::from("(!(true == true))"),
            },
            Test {
                input: String::from("a + add(b * c) + d"),
                expected: String::from("((a + add((b * c))) + d)"),
            },
            Test {
                input: String::from("add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))"),
                expected: String::from("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
            },
            Test {
                input: String::from("add(a + b + c * d / f + g)"),
                expected: String::from("add((((a + b) + ((c * d) / f)) + g))"),
            },
        ];

        for test in tests {
            let lexer = Lexer::new(&test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program().unwrap();
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
            let program = parser.parse_program().unwrap();
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

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        dbg!(&program);
        if program.statements.len() != 1 {
            panic!(
                "Program.Statements doesn't contain 1 statement. Got: {}",
                program
            );
        };
        let if_expression = match &program.statements[0] {
            Statement::Expression(i) => i,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                e
            ),
        };
        let if_expression = match &if_expression.expression {
            Expression::IfExpression(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::IfExpression\nGot: {}",
                e
            ),
        };
        let condition = match *if_expression.condition.clone() {
            Expression::InfixExpression(i) => i,
            e => panic!("The condition isn't an InfixExpression\nGot: {}", e),
        };
        let con_left = match *condition.left {
            Expression::Identifier(i) => i,
            e => panic!("Not an Identifier\nGot: {}", e),
        };
        let con_right = match *condition.right {
            Expression::Identifier(i) => i,
            e => panic!("Not an Identifier\nGot: {}", e),
        };
        assert_eq!(con_left.value, String::from("x"));
        assert_eq!(condition.operator, String::from("<"));
        assert_eq!(con_right.value, String::from("y"));
        if if_expression.consequence.statements.len() != 1 {
            panic!("Consequence should only be one statement");
        }
        let statement = match if_expression.consequence.statements[0].clone() {
            Statement::Expression(e) => e,
            e => panic!("Expected a Statement::Expression\nGot: {}", e),
        };
        let statement = match statement.expression {
            Expression::Identifier(i) => i,
            e => panic!("Not an Identifier\nGot: {}", e),
        };
        assert_eq!(statement.value, String::from("x"));
        match if_expression.alternative {
            None => (),
            _ => panic!("The alternative statements where not None"),
        }
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        dbg!(&program);
        if program.statements.len() != 1 {
            panic!(
                "Program.Statements doesn't contain 1 statement. Got: {}",
                program
            );
        };
        let if_expression = match &program.statements[0] {
            Statement::Expression(i) => i,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                e
            ),
        };
        let if_expression = match &if_expression.expression {
            Expression::IfExpression(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::IfExpression\nGot: {}",
                e
            ),
        };
        let condition = match *if_expression.condition.clone() {
            Expression::InfixExpression(i) => i,
            e => panic!("The condition isn't an InfixExpression\nGot: {}", e),
        };
        let con_left = match *condition.left {
            Expression::Identifier(i) => i,
            e => panic!("Not an Identifier\nGot: {}", e),
        };
        let con_right = match *condition.right {
            Expression::Identifier(i) => i,
            e => panic!("Not an Identifier\nGot: {}", e),
        };
        assert_eq!(con_left.value, String::from("x"));
        assert_eq!(condition.operator, String::from("<"));
        assert_eq!(con_right.value, String::from("y"));
        if if_expression.consequence.statements.len() != 1 {
            panic!("Consequence should only be one statement");
        }
        let statement = match if_expression.consequence.statements[0].clone() {
            Statement::Expression(e) => e,
            e => panic!("Expected a Statement::Expression\nGot: {}", e),
        };
        let statement = match statement.expression {
            Expression::Identifier(i) => i,
            e => panic!("Not an Identifier\nGot: {}", e),
        };
        assert_eq!(statement.value, String::from("x"));
        let else_block = match &if_expression.alternative {
            Some(e) => e,
            None => panic!("Alternative statements are None"),
        };
        if else_block.statements.len() != 1 {
            panic!(
                "Else_block.statements doesn't contain 1 statement. Got: {}",
                else_block
            );
        };
        let else_statement = match &else_block.statements[0] {
            Statement::Expression(e) => e,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                e
            ),
        };
        let else_statement = match &else_statement.expression {
            Expression::Identifier(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                e
            ),
        };
        assert_eq!(else_statement.value, String::from("y"));
    }

    #[test]
    fn test_function_literal() {
        let input = "fn(x, y) { x + y; }";
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        dbg!(&program);
        if program.statements.len() != 1 {
            panic!(
                "Program.Statements doesn't contain 1 statement. Got: {}",
                program
            );
        };
        let function_expression = match &program.statements[0] {
            Statement::Expression(i) => i,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                e
            ),
        };
        let function_literal = match &function_expression.expression {
            Expression::FunctionLiteral(f) => f,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::FunctionLiteral\nGot: {}",
                e
            ),
        };
        if function_literal.parameters.len() != 2 {
            panic!(
                "FunctionLiteral.parameters doesn't contain 2 parameters. Got: {}",
                program
            );
        }
        println!("{}", function_literal);
        assert_eq!(function_literal.parameters[0].value, String::from("x"));
        assert_eq!(function_literal.parameters[1].value, String::from("y"));
        dbg!(&function_literal.body);
        let statement = match function_literal.body.statements[0].clone() {
            Statement::Expression(e) => e,
            e => panic!("Expected a Statement::Expression\nGot: {}", e),
        };
        let body = match statement.expression {
            Expression::InfixExpression(i) => i,
            e => panic!("The condition isn't an InfixExpression\nGot: {}", e),
        };
        let body_left = match *body.left {
            Expression::Identifier(i) => i,
            e => panic!("Not an Identifier\nGot: {}", e),
        };
        assert_eq!(body_left.value, "x");
        assert_eq!(body.operator, "+");
        let body_right = match *body.right {
            Expression::Identifier(i) => i,
            e => panic!("Not an Identifier\nGot: {}", e),
        };
        assert_eq!(body_right.value, "y");
    }

    #[test]
    fn test_call_expression() {
        let input = "add(1, 2 * 3, 4 + 5);";
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        dbg!(&program);
        if program.statements.len() != 1 {
            panic!(
                "Program.Statements doesn't contain 1 statement. Got: {}",
                program
            );
        };
        let call_expression = match &program.statements[0] {
            Statement::Expression(i) => i,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                e
            ),
        };
        let call_literal = match &call_expression.expression {
            Expression::CallExpression(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::CallExpression\nGot: {}",
                e
            ),
        };
        let call_function = match *call_literal.function.clone() {
            Expression::Identifier(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                e
            ),
        };
        assert_eq!(call_function.value, "add");
        if call_literal.arguments.len() != 3 {
            panic!(
                "Wrong length of arguments. Expected: 3\nGot: {}",
                call_literal.arguments.len()
            );
        }
        let first_arg = match &call_literal.arguments[0] {
            Expression::IntegerLiteral(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::IntegerLiteral\nGot: {}",
                e
            ),
        };
        assert_eq!(first_arg.value, 1);
        let second_arg = match &call_literal.arguments[1] {
            Expression::InfixExpression(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::InfixExpression\nGot: {}",
                e
            ),
        };
        let second_left = match *second_arg.left.clone() {
            Expression::IntegerLiteral(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                e
            ),
        };
        let second_right = match *second_arg.right.clone() {
            Expression::IntegerLiteral(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                e
            ),
        };
        assert_eq!(second_left.value, 2);
        assert_eq!(second_arg.operator, "*");
        assert_eq!(second_right.value, 3);
        let third_arg = match &call_literal.arguments[2] {
            Expression::InfixExpression(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::InfixExpression\nGot: {}",
                e
            ),
        };
        let third_left = match *third_arg.left.clone() {
            Expression::IntegerLiteral(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                e
            ),
        };
        let third_right = match *third_arg.right.clone() {
            Expression::IntegerLiteral(i) => i,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::Identifier\nGot: {}",
                e
            ),
        };
        assert_eq!(third_left.value, 4);
        assert_eq!(third_arg.operator, "+");
        assert_eq!(third_right.value, 5);
    }

    #[test]
    fn test_string_literal() {
        let input = r#""hello world";"#;
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let statement = match &program.statements[0] {
            Statement::Expression(es) => es,
            e => panic!(
                "Not the right kind of Statement. Expected: Statement::Expression\nGot: {}",
                e
            ),
        };
        let literal = match &statement.expression {
            Expression::StringLiteral(sl) => sl,
            e => panic!(
                "Not the right kind of Expression. Expected: Expression::StringLiteral\nGot: {}",
                e
            ),
        };
        assert_eq!(literal.value, String::from("hello world"));
    }
}

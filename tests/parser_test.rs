use rost_interpreter::{
    ast::{Expression, NodeTrait, Statement},
    lexer::Lexer,
    parser::Parser,
};

fn test_integer_literal(integer_expression: &Expression, expected_value: i64) {
    match integer_expression {
        Expression::IntegerLiteral { value, .. } => {
            assert_eq!(value, &expected_value);
        }
        e => panic!("Expected: IntegerLiteral\nGot: {:?}", e),
    }
}

fn test_let_statement(statement: &Statement, expected_name: &str) {
    assert_eq!(statement.token_literal(), String::from("let"));
    match statement {
        Statement::Let { name, .. } => match name {
            Expression::Identifier { ref value, .. } => {
                assert_eq!(name.token_literal(), expected_name);
                assert_eq!(value, expected_name);
            }
            e => panic!("Name is not an ExpressionIdentifier\nGot: {:?}", e),
        },
        e => panic!("Not a Statement::Let\nGot: {:?}", e),
    }
}

#[test]
fn let_statements() {
    let input = b"
let x = 5;
let y = 10;
let foobar = 838383;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    assert_eq!(program.statements.len(), 3);
    let tests = [String::from("x"), String::from("y"), String::from("foobar")];
    for (i, test) in tests.iter().enumerate() {
        let statement = &program.statements[i];
        test_let_statement(statement, test);
    }
}

#[test]
fn return_statements() {
    let input = b"
return 5;
return 10;
return 993322;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    assert_eq!(program.statements.len(), 3);
    for statement in program.statements {
        match statement {
            Statement::Return { .. } => {
                assert_eq!(statement.token_literal(), String::from("return"));
            }
            e => panic!("Not a Statement::Return\nGot: {:?}", e),
        }
    }
}

#[test]
fn identifier_expression() {
    let input = b"foobar;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Expression { expression, .. } => match expression {
            Expression::Identifier { value, .. } => {
                assert_eq!(value, "foobar");
            }
            e => panic!("Expected Expression::Identifier\nGot: {}", e),
        },
        e => panic!("Not a Statement::Expression\nGot: {}", e),
    }
}

#[test]
fn integer_literal() {
    let input = b"5;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Expression { expression, .. } => match expression {
            Expression::IntegerLiteral { value, .. } => {
                assert_eq!(value, &5);
            }
            e => panic!("Expected Expression::IntegerLiteral\nGot: {}", e),
        },
        e => panic!("Not a Statement::Expression\nGot: {}", e),
    }
}

#[test]
fn parsing_prefix_expression() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        operator: u8,
        integer_value: i64,
    }
    let tests = vec![
        Test {
            input: b"!5;".to_vec(),
            operator: b'!',
            integer_value: 5,
        },
        Test {
            input: b"-15".to_vec(),
            operator: b'-',
            integer_value: 15,
        },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::PrefixExpression {
                    operator, right, ..
                } => {
                    assert_eq!(operator, &test.operator);
                    test_integer_literal(right, test.integer_value);
                }
                e => panic!("Expected: PrefixExpression\nGot: {:?}", e),
            },
            e => panic!("Expected: ExpressionStatement\nGot: {:?}", e),
        }
    }
}

#[test]
fn test_parsing_infix_expression() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        left_value: i64,
        operator: Vec<u8>,
        right_value: i64,
    }
    let tests = vec![
        Test {
            input: b"5 + 5;".to_vec(),
            left_value: 5,
            operator: b"+".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 - 5;".to_vec(),
            left_value: 5,
            operator: b"-".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 * 5;".to_vec(),
            left_value: 5,
            operator: b"*".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 / 5;".to_vec(),
            left_value: 5,
            operator: b"/".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 > 5;".to_vec(),
            left_value: 5,
            operator: b">".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 < 5;".to_vec(),
            left_value: 5,
            operator: b"<".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 == 5;".to_vec(),
            left_value: 5,
            operator: b"==".to_vec(),
            right_value: 5,
        },
        Test {
            input: b"5 != 5;".to_vec(),
            left_value: 5,
            operator: b"!=".to_vec(),
            right_value: 5,
        },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();

        assert_eq!(program.statements.len(), 1);
        let expression_statement = match &program.statements[0] {
            Statement::Expression { expression, .. } => expression,
            e => panic!("Expected: Statement::Expression\nGot: {:?}", e),
        };
        match expression_statement {
            Expression::InfixExpression {
                left,
                operator,
                right,
                ..
            } => {
                test_integer_literal(left, test.left_value);
                assert_eq!(operator, &test.operator);
                test_integer_literal(right, test.right_value);
            }
            e => panic!("Expected: Expression::InfixExpression\nGot: {:?}", e),
        };
    }
}

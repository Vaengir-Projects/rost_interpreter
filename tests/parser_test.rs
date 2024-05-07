use rost_interpreter::{
    ast::{Expression, NodeTrait, Statement},
    lexer::Lexer,
    parser::Parser,
};
use std::ops::Deref;

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
fn parsing_infix_expression() {
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

#[test]
fn operator_precedence_parsing() {
    struct Test {
        input: Vec<u8>,
        expected: String,
    }
    let tests: Vec<Test> = vec![
        Test {
            input: b"-a * b".to_vec(),
            expected: String::from("((-a) * b)"),
        },
        Test {
            input: b"!-a".to_vec(),
            expected: String::from("(!(-a))"),
        },
        Test {
            input: b"a + b + c".to_vec(),
            expected: String::from("((a + b) + c)"),
        },
        Test {
            input: b"a + b - c".to_vec(),
            expected: String::from("((a + b) - c)"),
        },
        Test {
            input: b"a * b * c".to_vec(),
            expected: String::from("((a * b) * c)"),
        },
        Test {
            input: b"a * b / c".to_vec(),
            expected: String::from("((a * b) / c)"),
        },
        Test {
            input: b"a + b / c".to_vec(),
            expected: String::from("(a + (b / c))"),
        },
        Test {
            input: b"a + b * c + d / e - f".to_vec(),
            expected: String::from("(((a + (b * c)) + (d / e)) - f)"),
        },
        Test {
            input: b"3 + 4; -5 * 5".to_vec(),
            expected: String::from("(3 + 4)((-5) * 5)"),
        },
        Test {
            input: b"5 > 4 == 3 < 4".to_vec(),
            expected: String::from("((5 > 4) == (3 < 4))"),
        },
        Test {
            input: b"5 < 4 != 3 > 4".to_vec(),
            expected: String::from("((5 < 4) != (3 > 4))"),
        },
        Test {
            input: b"3 + 4 * 5 == 3 * 1 + 4 * 5".to_vec(),
            expected: String::from("((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))"),
        },
        Test {
            input: b"true".to_vec(),
            expected: String::from("true"),
        },
        Test {
            input: b"false".to_vec(),
            expected: String::from("false"),
        },
        Test {
            input: b"3 > 5 == false".to_vec(),
            expected: String::from("((3 > 5) == false)"),
        },
        Test {
            input: b"3 < 5 == true".to_vec(),
            expected: String::from("((3 < 5) == true)"),
        },
        Test {
            input: b"1 + (2 + 3) + 4".to_vec(),
            expected: String::from("((1 + (2 + 3)) + 4)"),
        },
        Test {
            input: b"(5 + 5) * 2".to_vec(),
            expected: String::from("((5 + 5) * 2)"),
        },
        Test {
            input: b"2 / (5 + 5)".to_vec(),
            expected: String::from("(2 / (5 + 5))"),
        },
        Test {
            input: b"-(5 + 5)".to_vec(),
            expected: String::from("(-(5 + 5))"),
        },
        Test {
            input: b"!(true == true)".to_vec(),
            expected: String::from("(!(true == true))"),
        },
        // Test {
        //     input: b"a + add(b * c) + d".to_vec(),
        //     expected: String::from("((a + add((b * c))) + d)"),
        // },
        // Test {
        //     input: b"add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))".to_vec(),
        //     expected: String::from("add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))"),
        // },
        // Test {
        //     input: b"add(a + b + c * d / f + g)".to_vec(),
        //     expected: String::from("add((((a + b) + ((c * d) / f)) + g))"),
        // },
        // Test {
        //     input: b"a * [1, 2, 3, 4][b * c] * d".to_vec(),
        //     expected: String::from("((a * ([1, 2, 3, 4][(b * c)])) * d)"),
        // },
        // Test {
        //     input: b"add(a * b[2], b[1], 2 * [1, 2][1])".to_vec(),
        //     expected: String::from("add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))"),
        // },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        dbg!(&program);
        assert_eq!(format!("{}", program), test.expected);
    }
}

#[test]
fn boolean_expression() {
    struct Test {
        input: Vec<u8>,
        expected: bool,
    }
    let tests: Vec<Test> = vec![
        Test {
            input: b"true;".to_vec(),
            expected: true,
        },
        Test {
            input: b"false;".to_vec(),
            expected: false,
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
            Expression::Boolean { value, .. } => {
                assert_eq!(value, &test.expected);
            }
            e => panic!("Expected: Expression::Boolean\nGot: {:?}", e),
        }
    }
}

#[test]
fn if_expression() {
    let input = b"if (x < y) { x }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    dbg!(&program);
    assert_eq!(program.statements.len(), 1);
    let expression_statement = match &program.statements[0] {
        Statement::Expression { expression, .. } => expression,
        e => panic!("Expected: Statement::Expression\nGot: {:?}", e),
    };
    let (condition, consequence, alternative) = match expression_statement {
        Expression::IfExpression {
            condition,
            consequence,
            alternative,
            ..
        } => (condition, consequence, alternative),
        e => panic!("Expected: Expression::IfExpression\nGot: {:?}", e),
    };
    let (left, operator, right) = match condition.deref() {
        Expression::InfixExpression {
            left,
            operator,
            right,
            ..
        } => (left, operator, right),
        e => panic!("Expected: Expression::InfixExpression\nGot: {:?}", e),
    };
    let con_left = match left.deref() {
        Expression::Identifier { value, .. } => value,
        e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
    };
    let con_right = match right.deref() {
        Expression::Identifier { value, .. } => value,
        e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
    };
    assert_eq!(con_left, "x");
    assert_eq!(operator, b"<");
    assert_eq!(con_right, "y");
    let statements = match consequence.deref() {
        Expression::BlockStatement { statements, .. } => statements,
        e => panic!("Expected: Expression::BlockStatement\nGot: {:?}", e),
    };
    assert_eq!(statements.len(), 1);
    let statement = match &statements[0] {
        Statement::Expression { expression, .. } => expression,
        e => panic!("Expected: Statement::Expression\nGot: {:?}", e),
    };
    match statement {
        Expression::Identifier { value, .. } => {
            assert_eq!(value, "x");
        }
        e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
    };
    match alternative {
        None => (),
        _ => panic!("The alternative statements where not None"),
    }
}

#[test]
fn function_literal_parsing() {
    let input = b"fn(x, y) { x + y; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();
    dbg!(&program);
    assert_eq!(program.statements.len(), 1);
    let expression_statement = match &program.statements[0] {
        Statement::Expression { expression, .. } => expression,
        e => panic!("Expected: Statement::Expression\nGot: {:?}", e),
    };
    let (body, parameters) = match expression_statement {
        Expression::FunctionLiteral {
            parameters, body, ..
        } => (body, parameters),
        e => panic!("Expected: Expression::FunctionLiteral\nGot: {:?}", e),
    };
    assert_eq!(parameters.len(), 2);
    match &parameters[0] {
        Expression::Identifier { value, .. } => assert_eq!(value, "x"),
        e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
    }
    match &parameters[1] {
        Expression::Identifier { value, .. } => assert_eq!(value, "y"),
        e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
    }
    let statements = match body.deref() {
        Expression::BlockStatement { statements, .. } => statements,
        e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
    };
    assert_eq!(statements.len(), 1);
    let body_statement = match &statements[0] {
        Statement::Expression { expression, .. } => expression,
        e => panic!("Expected: Statement::Expression\nGot: {:?}", e),
    };
    let (left, operator, right) = match body_statement {
        Expression::InfixExpression {
            left,
            operator,
            right,
            ..
        } => (left, operator, right),
        e => panic!("Expected: Expression::InfixExpression\nGot: {:?}", e),
    };
    match left.deref() {
        Expression::Identifier { value, .. } => assert_eq!(value, "x"),
        e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
    }
    assert_eq!(operator, b"+");
    match right.deref() {
        Expression::Identifier { value, .. } => assert_eq!(value, "y"),
        e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
    }
}

#[test]
fn function_parameter_parsing() {
    struct Test {
        input: Vec<u8>,
        expected_params: Vec<u8>,
    }
    let tests: Vec<Test> = vec![
        Test {
            input: b"fn() {};".to_vec(),
            expected_params: b"".to_vec(),
        },
        Test {
            input: b"fn(x) {};".to_vec(),
            expected_params: b"x".to_vec(),
        },
        Test {
            input: b"fn(x, y, z) {};".to_vec(),
            expected_params: b"xyz".to_vec(),
        },
    ];

    for test in tests {
        let lexer = Lexer::new(&test.input);
        let mut parser = Parser::new(lexer).unwrap();
        let program = parser.parse_program().unwrap();
        let expression_statement = match &program.statements[0] {
            Statement::Expression { expression, .. } => expression,
            e => panic!("Expected: Statement::Expression\nGot: {:?}", e),
        };
        let function_params = match expression_statement {
            Expression::FunctionLiteral { parameters, .. } => parameters,
            e => panic!("Expected: Expression::FunctionLiteral\nGot: {:?}", e),
        };
        assert_eq!(function_params.len(), test.expected_params.len());
        for (i, ident) in test.expected_params.iter().enumerate() {
            match &function_params[i] {
                Expression::Identifier { value, .. } => assert_eq!(&value.as_bytes()[0], ident),
                e => panic!("Expected: Expression::Identifier\nGot: {:?}", e),
            }
        }
    }
}

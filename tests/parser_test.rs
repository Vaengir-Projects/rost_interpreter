use rost_interpreter::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Node,
        PrefixExpression, ReturnStatement, Statement,
    },
    lexer::Lexer,
    parser::Parser,
};

fn test_let_statement(statement: &dyn Statement, name: &str) {
    if let Some(statement) = statement.as_any().downcast_ref::<LetStatement>() {
        assert_eq!(statement.token_literal(), String::from("let"));
        assert_eq!(statement.name.value, name);
        assert_eq!(statement.name.token_literal(), name);
    } else {
        panic!("Expected: LetStatement\nGot: {:?}", statement);
    }
}

fn test_integer_literal(integer_expression: &dyn Expression, value: i64) {
    if let Some(integer_literal) = integer_expression.as_any().downcast_ref::<IntegerLiteral>() {
        assert_eq!(integer_literal.value, value);
        assert_eq!(integer_literal.token_literal(), value.to_string());
    } else {
        panic!("Expected: IntegerLiteral\nGot: {:?}", integer_expression);
    }
}

#[test]
fn test_let_statements() {
    let input = b"
let x = 5;
let y = 10;
let foobar = 838383;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    assert_eq!(program.statements.len(), 3);
    test_let_statement(&*program.statements[0], "x");
    test_let_statement(&*program.statements[1], "y");
    test_let_statement(&*program.statements[2], "foobar");
}

#[test]
fn test_return_statements() {
    let input = b"
return 5;
return 10;
return 993322;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    assert_eq!(program.statements.len(), 3);
    for statement in program.statements {
        if let Some(return_statement) = statement.as_any().downcast_ref::<ReturnStatement>() {
            assert_eq!(return_statement.token_literal(), "return");
        } else {
            panic!("Expected: ReturnStatement\nGot: {:?}", statement);
        }
    }
}

#[test]
fn test_identifier_expression() {
    let input = b"foobar";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    if let Some(expression_statement) = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
    {
        if let Some(identifier) = expression_statement
            .expression
            .as_any()
            .downcast_ref::<Identifier>()
        {
            assert_eq!(identifier.value, "foobar");
            assert_eq!(identifier.token_literal(), "foobar");
        } else {
            panic!("Expected: Identifier\nGot: {:?}", expression_statement);
        }
    } else {
        panic!(
            "Expected: ExpressionStatement\nGot: {:?}",
            program.statements[0]
        );
    }
}

#[test]
fn test_integer_literal_expression() {
    let input = b"5;";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer).unwrap();
    let program = parser.parse_program().unwrap();

    assert_eq!(program.statements.len(), 1);
    if let Some(expression_statement) = program.statements[0]
        .as_any()
        .downcast_ref::<ExpressionStatement>()
    {
        if let Some(integer_literal) = expression_statement
            .expression
            .as_any()
            .downcast_ref::<IntegerLiteral>()
        {
            assert_eq!(integer_literal.value, 5);
            assert_eq!(integer_literal.token_literal(), "5");
        } else {
            panic!("Expected: Identifier\nGot: {:?}", expression_statement);
        }
    } else {
        panic!(
            "Expected: ExpressionStatement\nGot: {:?}",
            program.statements[0]
        );
    }
}

#[test]
fn test_parsing_prefix_expressions() {
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
        if let Some(expression_statement) = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>()
        {
            if let Some(prefix_expression) = expression_statement
                .expression
                .as_any()
                .downcast_ref::<PrefixExpression>()
            {
                assert_eq!(prefix_expression.operator, test.operator);
                test_integer_literal(&*prefix_expression.right, test.integer_value);
            } else {
                panic!(
                    "Expected: PrefixExpression\nGot: {:?}",
                    expression_statement
                );
            }
        } else {
            panic!(
                "Expected: ExpressionStatement\nGot: {:?}",
                program.statements[0]
            );
        }
    }
}

use rost_interpreter::{
    ast::Node, evaluator::Evaluator, lexer::Lexer, object::Object, parser::Parser,
};

fn test_eval(input: &[u8]) -> anyhow::Result<Object> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    Evaluator::eval(Node::Program(&program))
}

fn integer_object(object: Object, expected: i64) {
    match object {
        Object::Integer { value } => assert_eq!(value, expected),
        e => panic!("Expected Object::Integer\nGot: {:?}", e),
    }
}

fn test_boolean_object(object: Object, expected: bool) {
    match object {
        Object::Boolean { value } => assert_eq!(value, expected),
        e => panic!("Expected Object::Boolean\nGot: {:?}", e),
    }
}

fn test_null_object(object: Object) {
    assert_eq!(object, Object::Null);
}

#[test]
fn eval_integer_expression() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        expected: i64,
    }
    let tests = vec![
        Test {
            input: b"5".to_vec(),
            expected: 5,
        },
        Test {
            input: b"10".to_vec(),
            expected: 10,
        },
        // Test {
        //     input: b"-5".to_vec(),
        //     expected: -5,
        // },
        // Test {
        //     input: b"-10".to_vec(),
        //     expected: -10,
        // },
        // Test {
        //     input: b"5 + 5 + 5 + 5 - 10".to_vec(),
        //     expected: 10,
        // },
        // Test {
        //     input: b"2 * 2 * 2 * 2 * 2".to_vec(),
        //     expected: 32,
        // },
        // Test {
        //     input: b"-50 + 100 + -50".to_vec(),
        //     expected: 0,
        // },
        // Test {
        //     input: b"5 * 2 + 10".to_vec(),
        //     expected: 20,
        // },
        // Test {
        //     input: b"5 + 2 * 10".to_vec(),
        //     expected: 25,
        // },
        // Test {
        //     input: b"20 + 2 * -10".to_vec(),
        //     expected: 0,
        // },
        // Test {
        //     input: b"50 / 2 * 2 + 10".to_vec(),
        //     expected: 60,
        // },
        // Test {
        //     input: b"2 * (5 + 10)".to_vec(),
        //     expected: 30,
        // },
        // Test {
        //     input: b"3 * 3 * 3 + 10".to_vec(),
        //     expected: 37,
        // },
        // Test {
        //     input: b"3 * (3 * 3) + 10".to_vec(),
        //     expected: 37,
        // },
        // Test {
        //     input: b"(5 + 10 * 2 + 15 / 3) * 2 + -10".to_vec(),
        //     expected: 50,
        // },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input).unwrap();
        integer_object(evaluated, test.expected);
    }
}

#[test]
fn eval_boolean_expression() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        expected: bool,
    }
    let tests = vec![
        Test {
            input: b"true".to_vec(),
            expected: true,
        },
        Test {
            input: b"false".to_vec(),
            expected: false,
        },
        // Test {
        //     input: b"1 < 2".to_vec(),
        //     expected: true,
        // },
        // Test {
        //     input: b"1 > 2".to_vec(),
        //     expected: false,
        // },
        // Test {
        //     input: b"1 < 1".to_vec(),
        //     expected: false,
        // },
        // Test {
        //     input: b"1 > 1".to_vec(),
        //     expected: false,
        // },
        // Test {
        //     input: b"1 == 1".to_vec(),
        //     expected: true,
        // },
        // Test {
        //     input: b"1 != 1".to_vec(),
        //     expected: false,
        // },
        // Test {
        //     input: b"1 == 2".to_vec(),
        //     expected: false,
        // },
        // Test {
        //     input: b"1 != 2".to_vec(),
        //     expected: true,
        // },
        // Test {
        //     input: b"true == true".to_vec(),
        //     expected: true,
        // },
        // Test {
        //     input: b"false == false".to_vec(),
        //     expected: true,
        // },
        // Test {
        //     input: b"true == false".to_vec(),
        //     expected: false,
        // },
        // Test {
        //     input: b"true != false".to_vec(),
        //     expected: true,
        // },
        // Test {
        //     input: b"false != true".to_vec(),
        //     expected: true,
        // },
        // Test {
        //     input: b"(1 < 2) == true".to_vec(),
        //     expected: true,
        // },
        // Test {
        //     input: b"(1 < 2) == false".to_vec(),
        //     expected: false,
        // },
        // Test {
        //     input: b"(1 > 2) == true".to_vec(),
        //     expected: false,
        // },
        // Test {
        //     input: b"(1 > 2) == false".to_vec(),
        //     expected: true,
        // },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input).unwrap();
        test_boolean_object(evaluated, test.expected);
    }
}

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

fn null_object(object: Object) {
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
        Test {
            input: b"-5".to_vec(),
            expected: -5,
        },
        Test {
            input: b"-10".to_vec(),
            expected: -10,
        },
        Test {
            input: b"5 + 5 + 5 + 5 - 10".to_vec(),
            expected: 10,
        },
        Test {
            input: b"2 * 2 * 2 * 2 * 2".to_vec(),
            expected: 32,
        },
        Test {
            input: b"-50 + 100 + -50".to_vec(),
            expected: 0,
        },
        Test {
            input: b"5 * 2 + 10".to_vec(),
            expected: 20,
        },
        Test {
            input: b"5 + 2 * 10".to_vec(),
            expected: 25,
        },
        Test {
            input: b"20 + 2 * -10".to_vec(),
            expected: 0,
        },
        Test {
            input: b"50 / 2 * 2 + 10".to_vec(),
            expected: 60,
        },
        Test {
            input: b"2 * (5 + 10)".to_vec(),
            expected: 30,
        },
        Test {
            input: b"3 * 3 * 3 + 10".to_vec(),
            expected: 37,
        },
        Test {
            input: b"3 * (3 * 3) + 10".to_vec(),
            expected: 37,
        },
        Test {
            input: b"(5 + 10 * 2 + 15 / 3) * 2 + -10".to_vec(),
            expected: 50,
        },
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
        Test {
            input: b"1 < 2".to_vec(),
            expected: true,
        },
        Test {
            input: b"1 > 2".to_vec(),
            expected: false,
        },
        Test {
            input: b"1 < 1".to_vec(),
            expected: false,
        },
        Test {
            input: b"1 > 1".to_vec(),
            expected: false,
        },
        Test {
            input: b"1 == 1".to_vec(),
            expected: true,
        },
        Test {
            input: b"1 != 1".to_vec(),
            expected: false,
        },
        Test {
            input: b"1 == 2".to_vec(),
            expected: false,
        },
        Test {
            input: b"1 != 2".to_vec(),
            expected: true,
        },
        Test {
            input: b"true == true".to_vec(),
            expected: true,
        },
        Test {
            input: b"false == false".to_vec(),
            expected: true,
        },
        Test {
            input: b"true == false".to_vec(),
            expected: false,
        },
        Test {
            input: b"true != false".to_vec(),
            expected: true,
        },
        Test {
            input: b"false != true".to_vec(),
            expected: true,
        },
        Test {
            input: b"(1 < 2) == true".to_vec(),
            expected: true,
        },
        Test {
            input: b"(1 < 2) == false".to_vec(),
            expected: false,
        },
        Test {
            input: b"(1 > 2) == true".to_vec(),
            expected: false,
        },
        Test {
            input: b"(1 > 2) == false".to_vec(),
            expected: true,
        },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input).unwrap();
        test_boolean_object(evaluated, test.expected);
    }
}

#[test]
fn test_bang_operator() {
    struct Test {
        input: Vec<u8>,
        expected: bool,
    }
    let tests = vec![
        Test {
            input: b"!true".to_vec(),
            expected: false,
        },
        Test {
            input: b"!false".to_vec(),
            expected: true,
        },
        Test {
            input: b"!5".to_vec(),
            expected: false,
        },
        Test {
            input: b"!!true".to_vec(),
            expected: true,
        },
        Test {
            input: b"!!false".to_vec(),
            expected: false,
        },
        Test {
            input: b"!!5".to_vec(),
            expected: true,
        },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input).unwrap();
        test_boolean_object(evaluated, test.expected);
    }
}

#[test]
fn test_if_else_expressions() {
    #[derive(Debug)]
    enum Res {
        Good(i64),
        NoGood,
    }
    struct Test {
        input: Vec<u8>,
        expected: Res,
    }
    let tests = vec![
        Test {
            input: b"if (true) { 10 }".to_vec(),
            expected: Res::Good(10),
        },
        Test {
            input: b"if (false) { 10 }".to_vec(),
            expected: Res::NoGood,
        },
        Test {
            input: b"if (1) { 10 }".to_vec(),
            expected: Res::Good(10),
        },
        Test {
            input: b"if (1 < 2) { 10 }".to_vec(),
            expected: Res::Good(10),
        },
        Test {
            input: b"if (1 > 2) { 10 }".to_vec(),
            expected: Res::NoGood,
        },
        Test {
            input: b"if (1 > 2) { 10 } else { 20 }".to_vec(),
            expected: Res::Good(20),
        },
        Test {
            input: b"if (1 < 2) { 10 } else { 20 }".to_vec(),
            expected: Res::Good(10),
        },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input).unwrap();
        match test.expected {
            Res::Good(i) => integer_object(evaluated, i),
            Res::NoGood => null_object(evaluated),
        }
    }
}

// #[test]
// fn test_return_statements() {
//     struct Test {
//         input: Vec<u8>,
//         expected: i64,
//     }
//     let tests = vec![
//         Test {
//             input: b"return 10;".to_vec(),
//             expected: 10,
//         },
//         Test {
//             input: b"return 10; 9;".to_vec(),
//             expected: 10,
//         },
//         Test {
//             input: b"return 2 * 5; 9;".to_vec(),
//             expected: 10,
//         },
//         Test {
//             input: b"9; return 2 * 5; 9;".to_vec(),
//             expected: 10,
//         },
//         Test {
//             input: b"if (10 > 1) { if (10 > 1) { return 10; } return 1;}".to_vec(),
//             expected: 10,
//         },
//         Test {
//             input: b"let f = fn(x) { return x; x + 10; }; f(10);".to_vec(),
//             expected: 10,
//         },
//         Test {
//             input: b"let f = fn(x) { let result = x + 10; return result; return 10; }; f(10);"
//                 .to_vec(),
//             expected: 20,
//         },
//     ];
//     for test in tests {
//         let evaluated = test_eval(&test.input).unwrap();
//         integer_object(evaluated, test.expected);
//     }
// }
//
// #[test]
// fn test_error_handling() {
//     struct Test {
//         input: Vec<u8>,
//         expected_message: EvaluationError,
//     }
//     let tests = vec![
//         Test {
//             input: b"5 + true;".to_vec(),
//             expected_message: EvaluationError::TypeError(String::from("INTEGER + BOOLEAN")),
//         },
//         Test {
//             input: b"5 + true; 5;".to_vec(),
//             expected_message: EvaluationError::TypeError(String::from("INTEGER + BOOLEAN")),
//         },
//         Test {
//             input: b"-true".to_vec(),
//             expected_message: EvaluationError::OperatorError(String::from("-BOOLEAN")),
//         },
//         Test {
//             input: b"true + false;".to_vec(),
//             expected_message: EvaluationError::OperatorError(String::from("BOOLEAN + BOOLEAN")),
//         },
//         Test {
//             input: b"5; true + false; 5".to_vec(),
//             expected_message: EvaluationError::OperatorError(String::from("BOOLEAN + BOOLEAN")),
//         },
//         Test {
//             input: b"if (10 > 1) { true + false; }".to_vec(),
//             expected_message: EvaluationError::OperatorError(String::from("BOOLEAN + BOOLEAN")),
//         },
//         Test {
//             input: b"if (10 > 1) {
//                     if (10 > 1) {
//                     return true + false;
//                     }
//                     return 1;
//                     }"
//             .to_vec(),
//             expected_message: EvaluationError::OperatorError(String::from("BOOLEAN + BOOLEAN")),
//         },
//         Test {
//             input: b"foobar".to_vec(),
//             expected_message: EvaluationError::IdentError(String::from(
//                 "Identifier not found: foobar",
//             )),
//         },
//         Test {
//             input: b"\"Hello\" - \"World\"".to_vec(),
//             expected_message: EvaluationError::OperatorError(String::from("STRING - STRING")),
//         },
//     ];
//     for test in tests {
//         let evaluated = test_eval(&test.input);
//         let err_object = match evaluated {
//             Ok(o) => panic!("Not an Error\nGot: {}", o),
//             Err(e) => e,
//         };
//         assert_eq!(err_object, test.expected_message);
//     }
// }
//
// #[test]
// fn test_let_statement() {
//     struct Test {
//         input: Vec<u8>,
//         expected: i64,
//     }
//     let tests = vec![
//         Test {
//             input: b"let a = 5; a;".to_vec(),
//             expected: 5,
//         },
//         Test {
//             input: b"let a = 5 * 5; a;".to_vec(),
//             expected: 25,
//         },
//         Test {
//             input: b"let a = 5; let b = a; b;".to_vec(),
//             expected: 5,
//         },
//         Test {
//             input: b"let a = 5; let b = a; let c = a + b + 5; c;".to_vec(),
//             expected: 15,
//         },
//     ];
//     for test in tests {
//         integer_object(test_eval(&test.input).unwrap(), test.expected);
//     }
// }
//
// #[test]
// fn test_function_object() {
//     let input: &[u8] = b"fn(x) { x + 2; };";
//     let evaluated = test_eval(input).unwrap();
//     let func = match evaluated {
//         Object::Function(f) => f,
//         e => panic!("Expected a Object::Function\nGot: {}", e),
//     };
//     assert_eq!(func.parameters.len(), 1);
//     assert_eq!(func.parameters[0].value, "x");
//     assert_eq!(format!("{}", func.body), "(x + 2)");
// }
//
// #[test]
// fn test_function_application() {
//     struct Test {
//         input: Vec<u8>,
//         expected: i64,
//     }
//     let tests = vec![
//         Test {
//             input: b"let identity = fn(x) { x; }; identity(5);".to_vec(),
//             expected: 5,
//         },
//         Test {
//             input: b"let identity = fn(x) { return x; }; identity(5);".to_vec(),
//             expected: 5,
//         },
//         Test {
//             input: b"let double = fn(x) { x * 2; }; double(5);".to_vec(),
//             expected: 10,
//         },
//         Test {
//             input: b"let add = fn(x, y) { x + y; }; add(5, 5);".to_vec(),
//             expected: 10,
//         },
//         Test {
//             input: b"let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));".to_vec(),
//             expected: 20,
//         },
//         Test {
//             input: b"fn(x) { x; }(5)".to_vec(),
//             expected: 5,
//         },
//     ];
//     for test in tests {
//         integer_object(test_eval(&test.input).unwrap(), test.expected)
//     }
// }
//
// #[test]
// fn test_closures() {
//     let input: &[u8] =
//         b"let newAdder = fn(x) { fn(y) { x + y }; }; let addTwo = newAdder(2); addTwo(2);";
//     integer_object(test_eval(input).unwrap(), 4)
// }
//
// #[test]
// fn test_string_literal() {
//     let input = b"\"Hello World!\"";
//     let evaluated = test_eval(input).unwrap();
//     let str = match evaluated {
//         Object::String(s) => s,
//         e => panic!("Expected a Object::Function\nGot: {}", e),
//     };
//     assert_eq!(str.value, "Hello World!");
// }
//
// #[test]
// fn test_string_concatenation() {
//     let input = b"\"Hello\" + \" \" + \"World!\"";
//     let evaluated = test_eval(input).unwrap();
//     let str = match evaluated {
//         Object::String(s) => s,
//         e => panic!("Expected a Object::Function\nGot: {}", e),
//     };
//     assert_eq!(str.value, String::from("Hello World!"));
// }
//
// #[test]
// fn test_builtin_functions() {
//     #[derive(Debug)]
//     struct Test {
//         input: String,
//         expected: Result<i64, EvaluationError>,
//     }
//     let tests = vec![
//         Test {
//             input: String::from(r#"len("")"#),
//             expected: Ok(0),
//         },
//         Test {
//             input: String::from(r#"len("four")"#),
//             expected: Ok(4),
//         },
//         Test {
//             input: String::from(r#"len("hello world")"#),
//             expected: Ok(11),
//         },
//         Test {
//             input: String::from(r#"len(1)"#),
//             expected: Err(EvaluationError::BuiltInError(String::from(
//                 "Wrong kind of argument.\nExpected: String\nGot: INTEGER",
//             ))),
//         },
//         Test {
//             input: String::from(r#"len("one", "two")"#),
//             expected: Err(EvaluationError::BuiltInError(String::from(
//                 "Wrong number of arguments.\nExpected: 1\nGot: 2",
//             ))),
//         },
//     ];
//     for test in tests {
//         let evaluated = test_eval(&test.input);
//         match test.expected {
//             Ok(i) => test_integer_object(evaluated.unwrap(), i),
//             Err(e) => {
//                 assert_eq!(evaluated, Err(e));
//             }
//         }
//     }
// }

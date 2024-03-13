#[cfg(test)]
mod tests {
    use rost_interpreter::{
        evaluator::{eval, EvaluationError},
        lexer::Lexer,
        object::{Environment, Object},
        parser::Parser,
    };
    use std::cell::RefCell;

    fn test_eval(input: String) -> Result<Object, EvaluationError> {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let env = RefCell::new(Environment::new());
        let result = eval(program, &mut env.borrow_mut());
        result
    }

    fn test_integer_object(object: Object, expected: i64) {
        let result = match object {
            Object::Integer(i) => i,
            e => panic!("Expected Object::Integer\nGot: {:?}", e),
        };
        assert_eq!(result.value, expected);
    }

    fn test_boolean_object(object: Object, expected: bool) {
        let result = match object {
            Object::Boolean(b) => b,
            e => panic!("Expected Object::Boolean\nGot: {:?}", e),
        };
        assert_eq!(result.value, expected);
    }

    fn test_null_object(object: Object) {
        assert_eq!(object, Object::Null);
    }

    #[test]
    fn test_eval_integer_expression() {
        struct Test {
            input: String,
            expected: i64,
        }
        let tests = vec![
            Test {
                input: String::from("5"),
                expected: 5,
            },
            Test {
                input: String::from("10"),
                expected: 10,
            },
            Test {
                input: String::from("-5"),
                expected: -5,
            },
            Test {
                input: String::from("-10"),
                expected: -10,
            },
            Test {
                input: String::from("5 + 5 + 5 + 5 - 10"),
                expected: 10,
            },
            Test {
                input: String::from("2 * 2 * 2 * 2 * 2"),
                expected: 32,
            },
            Test {
                input: String::from("-50 + 100 + -50"),
                expected: 0,
            },
            Test {
                input: String::from("5 * 2 + 10"),
                expected: 20,
            },
            Test {
                input: String::from("5 + 2 * 10"),
                expected: 25,
            },
            Test {
                input: String::from("20 + 2 * -10"),
                expected: 0,
            },
            Test {
                input: String::from("50 / 2 * 2 + 10"),
                expected: 60,
            },
            Test {
                input: String::from("2 * (5 + 10)"),
                expected: 30,
            },
            Test {
                input: String::from("3 * 3 * 3 + 10"),
                expected: 37,
            },
            Test {
                input: String::from("3 * (3 * 3) + 10"),
                expected: 37,
            },
            Test {
                input: String::from("(5 + 10 * 2 + 15 / 3) * 2 + -10"),
                expected: 50,
            },
        ];
        for test in tests {
            let evaluated = test_eval(test.input).unwrap();
            test_integer_object(evaluated, test.expected);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        struct Test {
            input: String,
            expected: bool,
        }
        let tests = vec![
            Test {
                input: String::from("true"),
                expected: true,
            },
            Test {
                input: String::from("false"),
                expected: false,
            },
            Test {
                input: String::from("1 < 2"),
                expected: true,
            },
            Test {
                input: String::from("1 > 2"),
                expected: false,
            },
            Test {
                input: String::from("1 < 1"),
                expected: false,
            },
            Test {
                input: String::from("1 > 1"),
                expected: false,
            },
            Test {
                input: String::from("1 == 1"),
                expected: true,
            },
            Test {
                input: String::from("1 != 1"),
                expected: false,
            },
            Test {
                input: String::from("1 == 2"),
                expected: false,
            },
            Test {
                input: String::from("1 != 2"),
                expected: true,
            },
            Test {
                input: String::from("true == true"),
                expected: true,
            },
            Test {
                input: String::from("false == false"),
                expected: true,
            },
            Test {
                input: String::from("true == false"),
                expected: false,
            },
            Test {
                input: String::from("true != false"),
                expected: true,
            },
            Test {
                input: String::from("false != true"),
                expected: true,
            },
            Test {
                input: String::from("(1 < 2) == true"),
                expected: true,
            },
            Test {
                input: String::from("(1 < 2) == false"),
                expected: false,
            },
            Test {
                input: String::from("(1 > 2) == true"),
                expected: false,
            },
            Test {
                input: String::from("(1 > 2) == false"),
                expected: true,
            },
        ];
        for test in tests {
            let evaluated = test_eval(test.input).unwrap();
            test_boolean_object(evaluated, test.expected);
        }
    }

    #[test]
    fn test_bang_operator() {
        struct Test {
            input: String,
            expected: bool,
        }
        let tests = vec![
            Test {
                input: String::from("!true"),
                expected: false,
            },
            Test {
                input: String::from("!false"),
                expected: true,
            },
            Test {
                input: String::from("!5"),
                expected: false,
            },
            Test {
                input: String::from("!!true"),
                expected: true,
            },
            Test {
                input: String::from("!!false"),
                expected: false,
            },
            Test {
                input: String::from("!!5"),
                expected: true,
            },
        ];
        for test in tests {
            let evaluated = test_eval(test.input).unwrap();
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
            input: String,
            expected: Res,
        }
        let tests = vec![
            Test {
                input: String::from("if (true) { 10 }"),
                expected: Res::Good(10),
            },
            Test {
                input: String::from("if (false) { 10 }"),
                expected: Res::NoGood,
            },
            Test {
                input: String::from("if (1) { 10 }"),
                expected: Res::Good(10),
            },
            Test {
                input: String::from("if (1 < 2) { 10 }"),
                expected: Res::Good(10),
            },
            Test {
                input: String::from("if (1 > 2) { 10 }"),
                expected: Res::NoGood,
            },
            Test {
                input: String::from("if (1 > 2) { 10 } else { 20 }"),
                expected: Res::Good(20),
            },
            Test {
                input: String::from("if (1 < 2) { 10 } else { 20 }"),
                expected: Res::Good(10),
            },
        ];
        for test in tests {
            let evaluated = test_eval(test.input).unwrap();
            match test.expected {
                Res::Good(i) => test_integer_object(evaluated, i),
                Res::NoGood => test_null_object(evaluated),
            }
        }
    }

    #[test]
    fn test_return_statements() {
        struct Test {
            input: String,
            expected: i64,
        }
        let tests = vec![
            Test {
                input: String::from("return 10;"),
                expected: 10,
            },
            Test {
                input: String::from("return 10; 9;"),
                expected: 10,
            },
            Test {
                input: String::from("return 2 * 5; 9;"),
                expected: 10,
            },
            Test {
                input: String::from("9; return 2 * 5; 9;"),
                expected: 10,
            },
            Test {
                input: String::from("if (10 > 1) { if (10 > 1) { return 10; } return 1;}"),
                expected: 10,
            },
        ];
        for test in tests {
            let evaluated = test_eval(test.input).unwrap();
            test_integer_object(evaluated, test.expected);
        }
    }

    #[test]
    fn test_error_handling() {
        struct Test {
            input: String,
            expected_message: EvaluationError,
        }
        let tests = vec![
            Test {
                input: String::from("5 + true;"),
                expected_message: EvaluationError::TypeError(String::from("INTEGER + BOOLEAN")),
            },
            Test {
                input: String::from("5 + true; 5;"),
                expected_message: EvaluationError::TypeError(String::from("INTEGER + BOOLEAN")),
            },
            Test {
                input: String::from("-true"),
                expected_message: EvaluationError::OperatorError(String::from("-BOOLEAN")),
            },
            Test {
                input: String::from("true + false;"),
                expected_message: EvaluationError::OperatorError(String::from("BOOLEAN + BOOLEAN")),
            },
            Test {
                input: String::from("5; true + false; 5"),
                expected_message: EvaluationError::OperatorError(String::from("BOOLEAN + BOOLEAN")),
            },
            Test {
                input: String::from("if (10 > 1) { true + false; }"),
                expected_message: EvaluationError::OperatorError(String::from("BOOLEAN + BOOLEAN")),
            },
            Test {
                input: String::from(
                    r#"if (10 > 1) {
                    if (10 > 1) {
                    return true + false;
                    }
                    return 1;
                    }"#,
                ),
                expected_message: EvaluationError::OperatorError(String::from("BOOLEAN + BOOLEAN")),
            },
            Test {
                input: String::from("foobar"),
                expected_message: EvaluationError::IdentError(String::from("foobar")),
            },
        ];
        for test in tests {
            let evaluated = test_eval(test.input);
            let err_object = match evaluated {
                Ok(o) => panic!("Not an Error\nGot: {}", o),
                Err(e) => e,
            };
            assert_eq!(err_object, test.expected_message);
        }
    }

    #[test]
    fn test_let_statement() {
        struct Test {
            input: String,
            expected: i64,
        }
        let tests = vec![
            Test {
                input: String::from("let a = 5; a;"),
                expected: 5,
            },
            Test {
                input: String::from("let a = 5 * 5; a;"),
                expected: 25,
            },
            Test {
                input: String::from("let a = 5; let b = a; b;"),
                expected: 5,
            },
            Test {
                input: String::from("let a = 5; let b = a; let c = a + b + 5; c;"),
                expected: 15,
            },
        ];
        for test in tests {
            test_integer_object(test_eval(test.input).unwrap(), test.expected);
        }
    }
}

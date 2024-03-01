#[cfg(test)]
mod tests {
    use rost_interpreter::{evaluator::eval, lexer::Lexer, object::Object, parser::Parser};

    fn test_eval(input: String) -> Object {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        eval(program).unwrap()
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
        ];
        for test in tests {
            let evaluated = test_eval(test.input);
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
        ];
        for test in tests {
            let evaluated = test_eval(test.input);
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
            let evaluated = test_eval(test.input);
            test_boolean_object(evaluated, test.expected);
        }
    }
}

use anyhow::anyhow;
use rost_interpreter::{
    ast::{Expression, Node},
    evaluator::Evaluator,
    lexer::Lexer,
    object::{Environment, Object},
    parser::Parser,
};
use std::{cell::RefCell, collections::HashMap};

fn test_eval(input: &[u8]) -> anyhow::Result<Object> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer)?;
    let program = parser.parse_program()?;
    let env = RefCell::new(Environment::new(None));
    let result = Evaluator::eval(Node::Program(&program), &mut env.borrow_mut());
    result
}

fn test_integer_object(object: Object, expected: i64) {
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
        test_integer_object(evaluated, test.expected);
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
fn bang_operator() {
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
fn if_else_expressions() {
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
            Res::Good(i) => test_integer_object(evaluated, i),
            Res::NoGood => null_object(evaluated),
        }
    }
}

#[test]
fn return_statements() {
    struct Test {
        input: Vec<u8>,
        expected: i64,
    }
    let tests = vec![
        Test {
            input: b"return 10;".to_vec(),
            expected: 10,
        },
        Test {
            input: b"return 10; 9;".to_vec(),
            expected: 10,
        },
        Test {
            input: b"return 2 * 5; 9;".to_vec(),
            expected: 10,
        },
        Test {
            input: b"9; return 2 * 5; 9;".to_vec(),
            expected: 10,
        },
        Test {
            input: b"if (10 > 1) { if (10 > 1) { return 10; } return 1;}".to_vec(),
            expected: 10,
        },
        Test {
            input: b"let f = fn(x) { return x; x + 10; }; f(10);".to_vec(),
            expected: 10,
        },
        Test {
            input: b"let f = fn(x) { let result = x + 10; return result; return 10; }; f(10);"
                .to_vec(),
            expected: 20,
        },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input).unwrap();
        test_integer_object(evaluated, test.expected);
    }
}

#[test]
fn error_handling() {
    struct Test {
        input: Vec<u8>,
        expected_message: String,
    }
    let tests = vec![
        Test {
            input: b"5 + true;".to_vec(),
            expected_message: String::from("INTEGER + BOOLEAN"),
        },
        Test {
            input: b"5 + true; 5;".to_vec(),
            expected_message: String::from("INTEGER + BOOLEAN"),
        },
        Test {
            input: b"-true".to_vec(),
            expected_message: String::from("-BOOLEAN"),
        },
        Test {
            input: b"true + false;".to_vec(),
            expected_message: String::from("BOOLEAN + BOOLEAN"),
        },
        Test {
            input: b"5; true + false; 5".to_vec(),
            expected_message: String::from("BOOLEAN + BOOLEAN"),
        },
        Test {
            input: b"if (10 > 1) { true + false; }".to_vec(),
            expected_message: String::from("BOOLEAN + BOOLEAN"),
        },
        Test {
            input: b"if (10 > 1) {
                            if (10 > 1) {
                            return true + false;
                            }
                            return 1;
                            }"
            .to_vec(),
            expected_message: String::from("BOOLEAN + BOOLEAN"),
        },
        Test {
            input: b"foobar".to_vec(),
            expected_message: String::from("Identifier not found: foobar"),
        },
        Test {
            input: b"\"Hello\" - \"World\"".to_vec(),
            expected_message: String::from("STRING - STRING"),
        },
        Test {
            input: b"{\"name\": \"Monkey\"}[fn(x) { x }];".to_vec(),
            expected_message: String::from("Unusable as hash key: FUNCTION"),
        },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input);
        let err_object = match evaluated {
            Ok(o) => panic!("Not an Error\nGot: {}", o),
            Err(e) => e,
        };
        assert_eq!(
            format!("{}", err_object),
            format!("{}", test.expected_message)
        );
    }
}

#[test]
fn let_statement() {
    struct Test {
        input: Vec<u8>,
        expected: i64,
    }
    let tests = vec![
        Test {
            input: b"let a = 5; a;".to_vec(),
            expected: 5,
        },
        Test {
            input: b"let a = 5 * 5; a;".to_vec(),
            expected: 25,
        },
        Test {
            input: b"let a = 5; let b = a; b;".to_vec(),
            expected: 5,
        },
        Test {
            input: b"let a = 5; let b = a; let c = a + b + 5; c;".to_vec(),
            expected: 15,
        },
    ];
    for test in tests {
        test_integer_object(test_eval(&test.input).unwrap(), test.expected);
    }
}

#[test]
fn function_object() {
    let input: &[u8] = b"fn(x) { x + 2; };";
    let evaluated = test_eval(input).unwrap();
    let (parameters, body) = match evaluated {
        Object::Function {
            parameters, body, ..
        } => (parameters, body),
        e => panic!("Expected a Object::Function\nGot: {}", e),
    };
    assert_eq!(parameters.len(), 1);
    assert_eq!(format!("{}", body), "(x + 2)");
    match &parameters[0] {
        Expression::Identifier { value, .. } => {
            assert_eq!(value, "x");
        }
        e => panic!("Parameter not an Expression::Identifier\nGot: {}", e),
    }
}

#[test]
fn function_application() {
    struct Test {
        input: Vec<u8>,
        expected: i64,
    }
    let tests = vec![
        Test {
            input: b"let identity = fn(x) { x; }; identity(5);".to_vec(),
            expected: 5,
        },
        Test {
            input: b"let identity = fn(x) { return x; }; identity(5);".to_vec(),
            expected: 5,
        },
        Test {
            input: b"let double = fn(x) { x * 2; }; double(5);".to_vec(),
            expected: 10,
        },
        Test {
            input: b"let add = fn(x, y) { x + y; }; add(5, 5);".to_vec(),
            expected: 10,
        },
        Test {
            input: b"let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));".to_vec(),
            expected: 20,
        },
        Test {
            input: b"fn(x) { x; }(5)".to_vec(),
            expected: 5,
        },
    ];
    for test in tests {
        test_integer_object(test_eval(&test.input).unwrap(), test.expected)
    }
}

#[test]
fn closures() {
    let input: &[u8] =
        b"let newAdder = fn(x) { fn(y) { x + y }; }; let addTwo = newAdder(2); addTwo(2);";
    test_integer_object(test_eval(input).unwrap(), 4)
}

#[test]
fn string_literal() {
    let input = b"\"Hello World!\"";
    let evaluated = test_eval(input).unwrap();
    let str = match evaluated {
        Object::String { value } => value,
        e => panic!("Expected a Object::Function\nGot: {}", e),
    };
    assert_eq!(str, b"Hello World!");
}

#[test]
fn string_concatenation() {
    let input = b"\"Hello\" + \" \" + \"World!\"";
    let evaluated = test_eval(input).unwrap();
    let str = match evaluated {
        Object::String { value } => value,
        e => panic!("Expected a Object::Function\nGot: {}", e),
    };
    assert_eq!(str, b"Hello World!");
}

#[test]
fn builtin_functions() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        expected: anyhow::Result<i64>,
    }
    let tests = vec![
        Test {
            input: b"len(\"\")".to_vec(),
            expected: Ok(0),
        },
        Test {
            input: b"len(\"four\")".to_vec(),
            expected: Ok(4),
        },
        Test {
            input: b"len(\"hello world\")".to_vec(),
            expected: Ok(11),
        },
        Test {
            input: b"len(1)".to_vec(),
            expected: Err(anyhow!(
                "Wrong kind of argument.\nExpected: String || Array\nGot: 1",
            )),
        },
        Test {
            input: b"len(\"one\", \"two\")".to_vec(),
            expected: Err(anyhow!("Wrong number of arguments.\nExpected: 1\nGot: 2",)),
        },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input);
        dbg!(&test.expected, &evaluated);
        match test.expected {
            Ok(i) => test_integer_object(evaluated.unwrap(), i),
            Err(e) => match evaluated {
                Err(e2) => assert_eq!(format!("{}", e), format!("{}", e2)),
                _ => unreachable!(),
            },
        }
    }
}

#[test]
fn array_literals() {
    let input = b"[1, 2 * 2, 3 + 3]";
    let evaluated = test_eval(input).unwrap();
    let elements = match evaluated {
        Object::Array { elements } => elements,
        e => panic!("Expected: Object::Array\nGot: {}", e),
    };
    assert_eq!(elements.len(), 3);
    test_integer_object(elements[0].clone(), 1);
    test_integer_object(elements[1].clone(), 4);
    test_integer_object(elements[2].clone(), 6);
}

#[test]
fn array_index_expressions() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        expected: anyhow::Result<i64>,
    }
    let tests = vec![
        Test {
            input: b"[1, 2, 3][0]".to_vec(),
            expected: Ok(1),
        },
        Test {
            input: b"[1, 2, 3][1]".to_vec(),
            expected: Ok(2),
        },
        Test {
            input: b"[1, 2, 3][2]".to_vec(),
            expected: Ok(3),
        },
        Test {
            input: b"let i = 0; [1][i];".to_vec(),
            expected: Ok(1),
        },
        Test {
            input: b"[1, 2, 3][1 + 1];".to_vec(),
            expected: Ok(3),
        },
        Test {
            input: b"let myArray = [1, 2, 3]; myArray[2];".to_vec(),
            expected: Ok(3),
        },
        Test {
            input: b"let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];".to_vec(),
            expected: Ok(6),
        },
        Test {
            input: b"let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]".to_vec(),
            expected: Ok(2),
        },
        Test {
            input: b"[1, 2, 3][3]".to_vec(),
            expected: Err(anyhow!("Index out of bounds")),
        },
        Test {
            input: b"[1, 2, 3][-1]".to_vec(),
            expected: Err(anyhow!("out of range integral type conversion attempted")),
        },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input);
        dbg!(&evaluated);
        match test.expected {
            Ok(i) => test_integer_object(evaluated.unwrap(), i),
            Err(e) => match evaluated {
                Err(e2) => assert_eq!(format!("{}", e), format!("{}", e2)),
                _ => unreachable!(),
            },
        }
    }
}

#[test]
fn hash_literals() {
    let input = b"let two = \"two\";
{
\"one\": 10 - 9,
two: 1 + 1,
\"thr\" + \"ee\": 6 / 2,
4: 4,
true: 5,
false: 6
}";

    let evaluated = test_eval(input).unwrap();
    let hash_pairs = match evaluated {
        Object::Hash { pairs } => pairs,
        e => panic!("Expected: Object::Hash\nGot: {}", e),
    };
    let expected: HashMap<Object, i64> = HashMap::from([
        (
            Object::String {
                value: b"one".to_vec(),
            },
            1,
        ),
        (
            Object::String {
                value: b"two".to_vec(),
            },
            2,
        ),
        (
            Object::String {
                value: b"three".to_vec(),
            },
            3,
        ),
        (Object::Integer { value: 4 }, 4),
        (Object::Boolean { value: true }, 5),
        (Object::Boolean { value: false }, 6),
    ]);
    assert_eq!(hash_pairs.len(), expected.len());
    for (ek, ev) in expected {
        let value = hash_pairs.get(&ek).unwrap();
        test_integer_object(value.clone(), ev);
    }
}

#[test]
fn hash_index_expression() {
    #[derive(Debug)]
    struct Test {
        input: Vec<u8>,
        expected: anyhow::Result<i64>,
    }
    let tests = vec![
        Test {
            input: b"{\"foo\": 5}[\"foo\"]".to_vec(),
            expected: Ok(5),
        },
        Test {
            input: b"{\"foo\": 5}[\"bar\"]".to_vec(),
            expected: Err(anyhow!(
                "Couldn't find bar in {{String {{ value: [102, 111, 111] }}: Integer {{ value: 5 }}}}"
            )),
        },
        Test {
            input: b"let key = \"foo\"; {\"foo\": 5}[key]".to_vec(),
            expected: Ok(5),
        },
        Test {
            input: b"{}[\"foo\"]".to_vec(),
            expected: Err(anyhow!("Couldn't find foo in {{}}")),
        },
        Test {
            input: b"{5: 5}[5]".to_vec(),
            expected: Ok(5),
        },
        Test {
            input: b"{true: 5}[true]".to_vec(),
            expected: Ok(5),
        },
        Test {
            input: b"{false: 5}[false]".to_vec(),
            expected: Ok(5),
        },
    ];
    for test in tests {
        let evaluated = test_eval(&test.input);
        dbg!(&evaluated);
        match test.expected {
            Ok(i) => test_integer_object(evaluated.unwrap(), i),
            Err(e) => match evaluated {
                Err(e2) => assert_eq!(format!("{}", e), format!("{}", e2)),
                _ => unreachable!(),
            },
        }
    }
}

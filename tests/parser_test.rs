use rost_interpreter::{
    ast::{Expression, NodeTrait, Statement},
    lexer::Lexer,
    parser::Parser,
};

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

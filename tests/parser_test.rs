#[cfg(test)]
mod tests {
    use rost_interpreter::{
        ast::{NodeTrait, Program, Statement},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_let_statements() {
        let input: &str = "\
let x = 5;
let y = 10;
let foobar = 838383;";

        let expected_identifiers: Vec<String> =
            vec![String::from("x"), String::from("y"), String::from("foobar")];

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program: Program = parser.parse_program();
        if program.statements.len() != 3 {
            panic!("program is empty: {:?}", program);
        }

        if program.statements.len() != 3 {
            panic!(
                "Program.Statements doesn't contain 3 statements. Got: {:?}",
                program
            );
        }

        for (i, ident) in expected_identifiers.iter().enumerate() {
            let statement = &program.statements[i];
            if !test_let_statement(statement, ident) {
                break;
            }
        }
    }

    fn test_let_statement(statement: &Statement, name: &str) -> bool {
        if statement.token_literal() != "let" {
            panic!(
                "statement.token_literal not 'let'. Got {:?}",
                statement.token_literal()
            );
        }

        let let_statement = match statement {
            Statement::Let(let_statement) => let_statement,
        };

        if let_statement.name.value != name {
            panic!(
                "Expected name value doesn't match:\nExpected: {:?}\nGot: {:?}",
                let_statement.name.value, name
            );
        }
        if let_statement.name.token_literal() != name {
            panic!(
                "Expected name value doesn't match:\nExpected: {:?}\nGot: {:?}",
                let_statement.name.token_literal(),
                name
            );
        }
        true
    }
}

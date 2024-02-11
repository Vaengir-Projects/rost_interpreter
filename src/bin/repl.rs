use std::io::{self, Write};

use rost_interpreter::{lexer::Lexer, token::TokenType};

const PROMPT: &str = ">> ";

fn main() {
    println!("Hello! Welcome to the Monkey programming language REPL!");
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "quit" {
            println!("Quitting MonkeyLang REPL...");
            break;
        }

        let mut lexer = Lexer::new(&input);
        let mut token = lexer.next_token();
        println!("---------------------------------------");
        while token.r#type != TokenType::EOF {
            println!("Token: {:?},\nLiteral: {:?}", token.r#type, token.literal);
            println!("---------------------------------------");
            token = lexer.next_token();
        }
        println!("");
    }
}

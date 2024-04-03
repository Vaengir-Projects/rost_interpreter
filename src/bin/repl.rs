use rost_interpreter::lexer::Lexer;
use rost_interpreter::token::TokenType;
use std::io::{self, Write};

const PROMPT: &str = ">> ";

const MONKEY_FACE: &str = r#"            __,__
   .--.  .-"     "-.  .--.
  / .. \/  .-. .-.  \/ .. \
 | |  '|  /   Y   \  |'  | |
 | \   \  \ 0 | 0 /  /   / |
  \ '- ,\.-"""""""-./, -' /
   ''-' /_   ^ ^   _\ '-''
       |  \._   _./  |
       \   \ '~' /   /
        '._ '-=-' _.'
           '-----'
"#;

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

        let mut lexer = Lexer::new(input.as_bytes());
        let mut token = lexer.next_token().unwrap();

        println!("---------------------------------------");
        while token.r#type != TokenType::EOF {
            println!("Token: {:?},\nLiteral: {:?}", token.r#type, token.literal);
            println!("---------------------------------------");
            token = lexer.next_token().unwrap();
        }
        println!();
    }
}

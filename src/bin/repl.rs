use rost_interpreter::{ast::Node, evaluator::Evaluator, lexer::Lexer, parser::Parser};
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
    // let env = RefCell::new(Environment::new(None));
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

        let lexer = Lexer::new(input.as_bytes());
        let mut parser = Parser::new(lexer).expect("Creating Parser errored");
        let program = parser.parse_program();
        match program {
            Ok(prog) => {
                let evaluated = Evaluator::eval(Node::Program(&prog));
                match evaluated {
                    Ok(evaled) => {
                        println!("{}", evaled);
                    }
                    Err(e) => {
                        eprintln!("{}", MONKEY_FACE);
                        eprintln!("{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", MONKEY_FACE);
                eprintln!("{}", e);
            }
        }
        println!("---------------------------------------");
    }
}

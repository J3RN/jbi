use std::io::{self, Write};

mod lexer;
mod runtime;
mod semantic_analyzer;

fn main() {
    let stdin = io::stdin();
    let mut state = runtime::State::default();

    loop {
        let mut input = String::new();

        print!("jbi> ");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                println!("\nGoodbye!");
                break;
            }
            Ok(_) => (),
            Err(_) => break,
        }

        match lexer::lex(input, "stdin".to_string()) {
            Ok(toks) => match semantic_analyzer::analyze(toks) {
                Ok(tree) => {
                    runtime::eval(&mut state, &tree);
                }
                Err(reason) => eprintln!("{:?}", reason),
            },
            Err(reason) => eprintln!("{:?}", reason),
        }
    }
}

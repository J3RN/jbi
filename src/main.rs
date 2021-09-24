use annotate_snippets::{display_list::DisplayList, snippet::Snippet};
use std::io::{self, Write};

mod lexer;
mod runtime;
mod semantic_analyzer;

pub struct Location<'a> {
    file: &'a str,
    line: &'a str,
    lineno: usize,
    range: (usize, usize),
}

pub trait ErrorOutput {
    fn to_error(&self) -> Snippet;
}

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

        match lexer::lex(&input, "stdin") {
            Ok(toks) => match semantic_analyzer::analyze(&toks) {
                Ok(tree) => {
                    if let Err(err) = runtime::eval(&mut state, &tree) {
                        let dl = DisplayList::from(err.to_error());
                        eprintln!("{}", dl);
                    }
                }
                Err(errs) => {
                    for err in errs {
                        let dl = DisplayList::from(err.to_error());
                        eprintln!("{}", dl);
                    }
                }
            },
            Err(errs) => {
                for err in errs {
                    let dl = DisplayList::from(err.to_error());
                    eprintln!("{}", dl);
                }
            }
        }
    }
}

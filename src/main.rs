use annotate_snippets::{display_list::DisplayList, snippet::Snippet};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

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
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        _ => eprintln!(
            r#"Usage: jbi [file]

If no `file' is passed, you will be presented with a REPL. If `file' is given,
it will be executed and output printed to the console."#
        ),
    }
}

fn run_file(filename: &str) {
    let mut file = File::open(filename).expect("Could not open file");
    let mut state = runtime::State::default();
    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("Failed to read file");

    run(&mut state, &content, &filename)
}

fn run_repl() {
    let stdin = io::stdin();

    loop {
        let mut input = String::new();
        let mut state = runtime::State::default();

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
        run(&mut state, &input, "stdin")
    }
}

fn run(state: &mut runtime::State, input: &str, filename: &str) {
    match lexer::lex(input, filename) {
        Ok(toks) => match semantic_analyzer::analyze(&toks) {
            Ok(tree) => {
                if let Err(err) = runtime::eval(state, &tree) {
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

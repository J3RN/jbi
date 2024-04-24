use annotate_snippets::{display_list::DisplayList, snippet::Snippet};
use rustyline::error::ReadlineError;
use std::env;
use std::fs::File;
use std::io::Read;

mod lexer;
mod parser;
mod runtime;

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
    let rl_config: rustyline::Config = rustyline::Config::builder()
        .check_cursor_position(true)
        .build();
    let mut rl = rustyline::Editor::<()>::with_config(rl_config);
    let mut state = runtime::State::default();

    loop {
        match rl.readline("jbi> ") {
            Ok(input) => {
                run(&mut state, &input, "stdin");
                rl.add_history_entry(input);
            }
            Err(ReadlineError::Eof) => {
                println!("\nGoodbye!");
                break;
            }
            Err(_) => break,
        }
    }
}

fn run(state: &mut runtime::State, input: &str, filename: &str) {
    let toks = lexer::lex(input, filename);

    match parser::parse(&toks) {
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
    };
}

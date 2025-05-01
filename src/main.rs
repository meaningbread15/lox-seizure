use std::env::args;
use std::fs;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: lox-seizure [script]");
            std::process::exit(64);
        }
    }
    fn run_file(path: &str) {
        let source = fs::read_to_string(path).expect("Failed to read the file hehe");
        run(&source);
    }

    fn run_prompt() {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        loop {
            print!("> ");
            stdout.flush().unwrap();

            let mut line = String::new();
            if stdin.read_line(&mut line).is_err() {
                break;
            }

            run(&line);
        }
    }

    fn run(source: &str) {
        println!("Running: {}", source.trim());
    }
}

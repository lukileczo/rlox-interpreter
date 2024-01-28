#![allow(unused)]

mod error;
mod scanner;
mod token;

use error::ErrorReporter;
use scanner::Scanner;

use std::fs;
use std::io::{self, BufRead, Write};

pub struct Lox {
    error_reporter: ErrorReporter,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            error_reporter: ErrorReporter::new(),
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, loc: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, loc, message);
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::from(source);
        for token in scanner.scan_tokens(&mut self.error_reporter) {
            println!("{:?}", token);
        }
    }

    pub fn run_file(&mut self, filename: String) {
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

        self.run(contents.as_str());

        if self.error_reporter.happened() {
            std::process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        let stdin = io::stdin();
        stdin.lock();
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    self.run(input.as_str());
                    self.error_reporter.clear();
                }
                Err(_) => println!("error"),
            }
        }
        println!("Bye!");
    }
}

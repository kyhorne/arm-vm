mod lexer;
mod parser;
mod translator;

use crate::util::Form;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

use super::assembler::lexer::lexer;
pub use super::assembler::lexer::Label;

/// Read and evaluate the bufed message from standard input.
pub fn repl() -> Result<(Option<u32>, Option<Label>, Option<Form>), ()> {
    print!(">>> ");
    let _ = stdout().flush();
    let mut buf = String::new();
    buf.pop(); // Remove the trailing new line.
    match stdin().read_line(&mut buf) {
        Ok(_) => {
            // Convert buffer into meaningful lexemes.
            let mut tokens = lexer(buf);
            // Parse tokens returned from the lexer.
            match parser::run(&mut tokens) {
                Ok((form, label)) => {
                    // Translate expression into bytecode.
                    match form {
                        Some(form) => {
                            let payload = translator::get_bytecode(&mut tokens, form);
                            return Ok((Some(payload), label, Some(form)));
                        }
                        None => {
                            return Ok((None, label, None));
                        }
                    }
                }
                Err(()) => println!("Invalid syntax!"),
            }
        }
        _ => (),
    }
    return Err(());
}

pub fn read_file() -> Vec<(Option<u32>, Option<Label>, Option<Form>)> {
    let mut program = Vec::new();
    if let Ok(file) = File::open("assembly/pgrm.asm") {
        for buf in BufReader::new(file).lines() {
            match buf {
                Ok(expression) => {
                    let mut tokens = lexer(expression);
                    if let Ok((form, label)) = parser::run(&mut tokens) {
                        match form {
                            Some(form) => {
                                let payload = translator::get_bytecode(&mut tokens, form);
                                program.push((Some(payload), label, Some(form)));
                            }
                            None => {
                                program.push((None, label, None));
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }
    program
}

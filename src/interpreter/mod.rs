mod assembler;
mod lexer;
mod parser;

use crate::util::Form;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

use super::interpreter::lexer::lexer;
pub use super::interpreter::lexer::Label;

/// Read and evaluate the buffered message from standard input.
pub fn repl() -> Result<(Option<u32>, Vec<Label>, Option<Form>), ()> {
    print!(">>> ");
    let _ = stdout().flush();
    let mut buffer = String::new();
    buffer.pop(); // Remove the trailing new line.
    match stdin().read_line(&mut buffer) {
        Ok(_) => {
            // Convert buffer into meaningful lexemes.
            let mut tokens = lexer(buffer);
            // Parse tokens returned from the lexer.
            match parser::run(&mut tokens) {
                Ok((form, labels)) => {
                    // Encode expression into bytecode.
                    match form {
                        Some(form) => {
                            let payload = assembler::get_bytecode(&mut tokens, form);
                            return Ok((Some(payload), labels, Some(form)));
                        }
                        None => {
                            return Ok((None, labels, None));
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

pub fn read_file() -> Vec<(Option<u32>, Vec<Label>, Option<Form>)> {
    let mut program = Vec::new();
    if let Ok(file) = File::open("assembly/pgrm1.asm") {
        for buffer in BufReader::new(file).lines() {
            match buffer {
                Ok(expression) => {
                    let mut tokens = lexer(expression);
                    if let Ok((form, labels)) = parser::run(&mut tokens) {
                        match form {
                            Some(form) => {
                                let payload = assembler::get_bytecode(&mut tokens, form);
                                program.push((Some(payload), labels, Some(form)));
                            }
                            None => {
                                program.push((None, labels, None));
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

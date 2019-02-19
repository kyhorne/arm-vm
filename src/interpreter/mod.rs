mod lexer;
mod parser;
mod assembler;

use super::interpreter::lexer::lexer;
use std::io::{BufReader, BufRead, stdin , stdout, Write};

use std::fs::File;

/// Read and evaluate the buffered message from standard input.
pub fn repl() -> Option<u32> {
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
				Ok(())  => {
					// Encode expression into bytecode.
					let payload = assembler::get_bytecode(&mut tokens);
					return Some(payload);
				},
				Err(()) => println!("Invalid expression!")
			}
		}
		_ => ()
	}
	return None
}

pub fn read_file() -> Vec<u32> {
	let mut program = Vec::new();
	if let Ok(file) = File::open("assembly/pgrm1.asm") {
		for buffer in BufReader::new(file).lines() {
			match buffer {
				Ok(expression) => {
					let mut tokens = lexer(expression);
					if let Ok(_) = parser::run(&mut tokens) {
						program.push(assembler::get_bytecode(&mut tokens))
					}
				}
				_ => ()
			}
		}
	}
	program
}

mod lexer;
mod parser;
mod assembler;

use super::interpreter::lexer::lexer;
use std::io::{stdin , stdout, Write};

/// Read and evaluate the buffered message from standard input.
pub fn repl() -> Option<u32> {
    print!(">>> ");
	let _ = stdout().flush();
	let mut buffer = String::new();
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

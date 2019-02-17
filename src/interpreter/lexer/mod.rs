use std::io::{stdin , stdout, Write};
mod token;

pub use token::{Token, Seperator, Immediate, Comment, Label};

use super::super::util::{Opcode, Register};
use std::str::FromStr;

use super::parser;
use super::assembler;



pub fn repl() {
	loop {
	    print!("> ");
		let _ = stdout().flush();
		let mut buffer = String::new();
		match stdin().read_line(&mut buffer) {
			Ok(_) => {
				let mut tokens = lexer(buffer);
				match parser::run(&mut tokens) {
					Ok(())  => {
						println!("Valid expression!");
						let payload = assembler::run(&mut tokens);
						println!("Assembler payload: {:#010X}", payload);
					},
					Err(()) => {
						println!("Invalid expression!");
						continue
					}
				}
			}
			_ => ()
		}
	}
}

/// Convert the soruce code into meaningful lexemes.
fn lexer(mut buffer: String) -> Vec<Token> {
	buffer.pop(); // Remove trailing new line.
	// Pad seperators with whitespace.
	buffer = buffer.replace(",", " , ");
	buffer = buffer.replace("[", " [ ");
	buffer = buffer.replace("]", " ] ");
	buffer = buffer.replace("{", " { ");
	buffer = buffer.replace("}", " } ");
	// Pad comments with whitespace.
	buffer = buffer.replace(";", " ; ");
	let mut tokens = Vec::new();
	// Iterate over string split by whitespace.
	for token in buffer.split_whitespace() {
		if let Ok(opcode) = Opcode::from_str(&token) {
			tokens.push(Token::Opcode(opcode));
			continue;
		}
		if let Ok(register) = Register::from_str(&token) {
			tokens.push(Token::Register(register));
			continue;
		}
		if let Ok(seperator) = Seperator::from_str(&token) {
			tokens.push(Token::Seperator(seperator));
			continue;
		}
		if let Ok(mut immediate) = Immediate::from_str(&token) {
			if immediate.is_valid() {
				tokens.push(Token::Immediate(immediate));
				continue;
			}
		}
		if let Ok(_) = Comment::from_str(&token) {
			break;
		}
		if let Ok(label) = Label::from_str(&token) {
			tokens.push(Token::Label(label));
			continue;
		}
	}
	return tokens;
}

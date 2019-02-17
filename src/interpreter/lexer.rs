use std::io::{stdin , stdout, Write};

use super::super::util::{Opcode, Register};
use std::str::FromStr;

use super::parser;
use super::assembler;

#[derive(
	Clone,
	EnumString,
	Eq,
	Debug,
	PartialEq
)]
pub enum Immediate {
	#[strum(default="true")]
	Value(String)
}
impl Immediate {
	fn is_valid(&mut self) -> bool {
        match self {
            Immediate::Value(value) => {
				let is_valid = value.starts_with("#");
				value.remove(0); // Remove prefix
				is_valid
			}
        }
    }
	pub fn get_value(self) -> u32 {
		match self {
			Immediate::Value(value) => {
				if value.contains("0x") {
					let value = value.trim_start_matches("0x");
					return u32::from_str_radix(&value, 16).unwrap()
				}
				value.parse::<u32>().unwrap()
			}
		}
	}
}

#[derive(
	Clone,
	EnumString,
	Eq,
	Debug,
	PartialEq
)]
pub enum Seperator {
	#[strum(serialize=",")]
	Comma,
	#[strum(serialize="[")]
	OpenBrace,
	#[strum(serialize="]")]
	CloseBrace,
	#[strum(serialize="{")]
	OpenBracket,
	#[strum(serialize="}")]
	CloseBracket
}

#[derive(
	Clone,
	EnumString,
	Eq,
	Debug,
	PartialEq
)]
pub enum Comment {
	#[strum(serialize=";")]
	Comment
}

#[derive(
	EnumString,
	Eq,
	Debug,
	PartialEq,
	Clone
)]
pub enum Label {
	#[strum(default="true")]
	Identifier(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Opcode(Opcode),
	Register(Register),
	Immediate(Immediate),
	Seperator(Seperator),
	Label(Label)
}

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

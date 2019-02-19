mod token;

pub use token::{Token, Seperator, Literal, Comment};
use super::super::util::{Opcode, Register};
use std::str::FromStr;

/// Convert the soruce code into meaningful lexemes.
pub fn lexer(mut buffer: String) -> Vec<Token> {
	// Pad separators with whitespace
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
		if let Ok(mut immed) = Literal::from_str(&token) {
			// Ensure immediate value has valid prefix.
			if immed.is_valid() {
				tokens.push(Token::Literal(immed));
				continue;
			}
		}
		if let Ok(_) = Comment::from_str(&token) {
			// Comments indicate the end of an expression.
			break;
		}
	}
	return tokens;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_form1_instruction() {
		let tokens = vec![
			Token::Opcode(Opcode::ADD),
			Token::Register(Register::R5),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R10)
		];
		assert_eq!(tokens, lexer("add r5, r1, r10".to_string()));
	}

	#[test]
    fn test_form2_instruction() {
		let tokens = vec![
			Token::Opcode(Opcode::MOV),
			Token::Register(Register::R3),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R5),
		];
		assert_eq!(tokens, lexer("MOV R3, R5 ; this is a comment".to_string()));
	}

	#[test]
    fn test_form4_instruction() {
		let tokens = vec![
			Token::Opcode(Opcode::ADD),
			Token::Register(Register::R5),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Literal(Literal::Immediate("10".to_string()))
		];
		assert_eq!(tokens, lexer("ADD R5, R1, #10".to_string()));
	}

	#[test]
    fn test_form5_instruction() {
		let tokens = vec![
			Token::Opcode(Opcode::MVN),
			Token::Register(Register::R6),
			Token::Seperator(Seperator::Comma),
			Token::Literal(Literal::Immediate("0x55555".to_string()))
		];
		assert_eq!(tokens, lexer("mvn r6, #0x55555".to_string()));
	}

}

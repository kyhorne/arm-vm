mod ready_state;
mod opcode_state;
mod register_state;
mod comma_state;
mod immediate_state;
mod open_brace_state;
mod close_brace_state;

use super::lexer::Token;
use super::form::Form;

struct ReadyState;

struct OpcodeState;

struct RegisterState;

struct CommaState;

struct ImmediateState;

struct OpenBrace;

struct CloseBrace;

pub struct StateMachine<S>{
	pub state:  S,
	pub tokens: Vec<Token>,
	// Possible forms this expression may satisfy.
	pub forms:  Vec<Form>
}

pub fn run(tokens: &mut Vec<Token>) -> Result<(), ()> {
	tokens.reverse();
	return ready_state::StateMachine::new(tokens.to_vec()).handler();
}

#[cfg(test)]
mod tests {

	use super::*;

	use super::super::super::util::Opcode;

	use super::super::super::util::Register;
	#[allow(unused_imports)]
	use super::super::lexer::{Seperator, Immediate, Label};

    #[test]
    fn test_form1_instruction() {
		// LDR STR
		// Test valid form 1 instruction.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::ADD),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1)
		]).is_ok());
		assert!(run(&mut vec![
			Token::Opcode(Opcode::STR),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Seperator(Seperator::OpenBrace),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::CloseBrace)
		]).is_ok());
    }

	#[test]
    fn test_form2_instruction() {
		// Test valid form 1 instruction.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::MOV),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1)
		]).is_ok());
		assert!(run(&mut vec![
			Token::Opcode(Opcode::STR),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Seperator(Seperator::OpenBrace),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::CloseBrace)
		]).is_ok());
    }

	#[test]
    fn test_form4_instruction() {
		// Test valid form 4 instruction.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::SUB),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Immediate(Immediate::Value("2".to_string()))
		]).is_ok());
		assert!(run(&mut vec![
			Token::Opcode(Opcode::LDR),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Seperator(Seperator::OpenBrace),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Immediate(Immediate::Value("31".to_string())),
			Token::Seperator(Seperator::CloseBrace)
		]).is_ok());
    }

	#[test]
    fn test_form5_instruction() {
		// Test valid form 5 instruction.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::MVN),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Immediate(Immediate::Value("2".to_string()))
		]).is_ok());
    }

}

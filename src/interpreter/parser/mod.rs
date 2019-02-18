mod ready_state;
mod opcode_state;
mod register_state;
mod comma_state;
mod immediate_state;
mod open_brace_state;
mod close_brace_state;

use super::lexer::Token;
use super::super::util::Form;

/// The syntax is analyzed using a finite state machine
struct CloseBrace;
struct CommaState;
struct ImmediateState;
struct OpcodeState;
struct OpenBrace;
struct ReadyState;
struct RegisterState;


pub struct StateMachine<S>{
	// The current state of the state machine.
	pub state:  S,
	// The parse tree.
	pub tokens: Vec<Token>,
	// Possible forms this expression may satisfy.
	pub forms:  Vec<Form>
}

/// Run the state machine.
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
	use super::super::lexer::{Literal, Seperator};

    #[test]
    fn test_form1_instruction() {
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
			Token::Literal(Literal::Immediate("2".to_string()))
		]).is_ok());
		assert!(run(&mut vec![
			Token::Opcode(Opcode::LDR),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Seperator(Seperator::OpenBrace),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Literal(Literal::Immediate("31".to_string())),
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
			Token::Literal(Literal::Immediate("2".to_string()))
		]).is_ok());
    }

	#[test]
    fn test_invalid_expressions() {
		// Test expression with only the opcode.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::ADD),
		]).is_err());
		// Test expression with no opcode.
		assert!(run(&mut vec![
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1)
		]).is_err());
		// Test expression with no commas.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::ADD),
			Token::Register(Register::R1),
			Token::Register(Register::R1),
			Token::Register(Register::R1)
		]).is_err());
		// Test expression with one too many commas.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::MUL),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R9),
			Token::Seperator(Seperator::Comma),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R6),
			Token::Register(Register::R6)
		]).is_err());
		// Test expression with missing register.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::EOR),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R7),
			Token::Seperator(Seperator::Comma)
		]).is_err());
		// Test expression with no braces.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::STR),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1)
		]).is_err());
		// Test form 4 expression with an immediate value greater than allowed.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::SUB),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Literal(Literal::Immediate("0x1FFFF".to_string()))
		]).is_err());
		// Test form 5 expression with an immediate value greater than allowed.
		assert!(run(&mut vec![
			Token::Opcode(Opcode::MVN),
			Token::Register(Register::R1),
			Token::Seperator(Seperator::Comma),
			Token::Literal(Literal::Immediate("0x1FFFFF".to_string()))
		]).is_err());
	}

}

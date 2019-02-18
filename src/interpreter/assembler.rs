use super::super::util::{Opcode, Register, EncoderDecoder, Mask};
use super::lexer::{Token, Literal};

struct Assembler{
	payload: u32
}

impl Assembler {
	fn new() -> Assembler {
		Assembler {
			payload: std::u32::MIN
		}
	}
	/// Encode the opcode.
	fn encode_opcode(&mut self, opcode: Opcode) {
		let (_, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
		self.payload |= (opcode as Mask) << opcode_offset
	}
	/// Encode the destination register.
	fn encode_dr(&mut self, register: Register) {
		let (_, dr_offset) = EncoderDecoder::DR.get_encoding();
		self.payload |= (register as Mask) << dr_offset
	}
	/// Encode operand one as register address.
	fn encode_rx(&mut self, register: Register) {
		let (_, rx_offset) = EncoderDecoder::RX.get_encoding();
		self.payload |= (register as Mask) << rx_offset
	}
	/// Encode operand two as register address.
	fn encode_ry(&mut self, register: Register) {
		let (_, ry_offset) = EncoderDecoder::RY.get_encoding();
		self.payload |= (register as Mask) << ry_offset
	}
	/// Encode immediate 16-bit value.
	fn encode_immed16(&mut self, immed16: Literal) {
		let (_, immed16_offset) = EncoderDecoder::Immed16.get_encoding();
		self.payload |= immed16.get_value() << immed16_offset
	}
	/// Encode immediate 20-bit value.
	fn encode_immed20(&mut self, immed20: Literal) {
		let (_, immed20_offset) = EncoderDecoder::Immed20.get_encoding();
		self.payload |= immed20.get_value() << immed20_offset
	}
	/// Toggle the mode bit.
	fn toggle_mode_bit(&mut self) {
		let (mode_mask, _) = EncoderDecoder::Mode.get_encoding();
		self.payload |= mode_mask
	}
}

macro_rules! to_bytecode {
	($opcode:expr; $dr:expr; $op1:expr; $op2:expr) => (
		{
			// Encode the opcode.
			let mut assembler = Assembler::new();
			if let Some(Token::Opcode(opcode)) = $opcode {
				assembler.encode_opcode(opcode.clone());
			}
			// Encode the destination register.
			if let Some(Token::Register(register)) = $dr {
				assembler.encode_dr(register.clone());
			}
			// Encode operand two if it exists.
			if let Some(op2) = $op2 {
				match op2 {
					Token::Register(register) => assembler.encode_ry(register.clone()),
					Token::Literal(immed16) => {
						assembler.toggle_mode_bit();
						assembler.encode_immed16(immed16.clone())
					}
					_ => ()
				}
			} {
				// Encode operand one if it exists.
				match $op1 {
					Some(Token::Register(register)) => assembler.encode_rx(register.clone()),
					Some(Token::Literal(immed20)) => {
						assembler.toggle_mode_bit();
						assembler.encode_immed20(immed20.clone())
					}
					_ => ()
				}
			}
			assembler.payload
		}
	)
}

/// Get the bytecode encoding of tokens.
pub fn get_bytecode(tokens: &mut Vec<Token>) -> u32 {
	// Remove tokens that are irrelevant to byte code encoding.
	let mut optimized_tokens = tokens.iter().filter(|token| {
		match token {
			Token::Opcode(_) | Token::Register(_) | Token::Literal(_) => true,
			_ => false
		}
	}).collect::<Vec<_>>();
	let opcode = optimized_tokens.pop();
	let dr     = optimized_tokens.pop();
	let op1    = optimized_tokens.pop();
	let op2    = optimized_tokens.pop();
	to_bytecode!(opcode; dr; op1; op2)
}

#[cfg(test)]
mod tests {

    use super::*;
	use super::super::lexer::{Seperator};

    #[test]
    fn test_form1_instruction() {
		let mut tokens = vec![
				Token::Opcode(Opcode::EOR),
				Token::Register(Register::R4),
				Token::Seperator(Seperator::Comma),
				Token::Register(Register::R4),
				Token::Seperator(Seperator::Comma),
				Token::Register(Register::R4)
			];
		tokens.reverse();
		let (_, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
		let mut bytecode = (Opcode::EOR as u32) << opcode_offset;
		let (_, dr_offset) = EncoderDecoder::DR.get_encoding();
		bytecode |= (Register::R4 as u32) << dr_offset;
		let (_, rx_offset) = EncoderDecoder::RX.get_encoding();
		bytecode |= (Register::R4 as u32) << rx_offset;
		let (_, ry_offset) = EncoderDecoder::RY.get_encoding();
		bytecode |= (Register::R4 as u32) << ry_offset;
		assert_eq!(bytecode, get_bytecode(&mut tokens));
    }

	#[test]
    fn test_form2_instruction() {
		let mut tokens = vec![
				Token::Opcode(Opcode::MOV),
				Token::Register(Register::R5),
				Token::Seperator(Seperator::Comma),
				Token::Register(Register::R1)
			];
		tokens.reverse();
		let (_, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
		let mut bytecode = (Opcode::MOV as u32) << opcode_offset;
		let (_, dr_offset) = EncoderDecoder::DR.get_encoding();
		bytecode |= (Register::R5 as u32) << dr_offset;
		let (_, rx_offset) = EncoderDecoder::RX.get_encoding();
		bytecode |= (Register::R1 as u32) << rx_offset;
		assert_eq!(bytecode, get_bytecode(&mut tokens));
    }

	#[test]
    fn test_form4_instruction() {
		let immed = Literal::Immediate("0".to_string());
		let mut tokens = vec![
				Token::Opcode(Opcode::ADD),
				Token::Register(Register::R4),
				Token::Seperator(Seperator::Comma),
				Token::Register(Register::R7),
				Token::Literal(immed.clone())
			];
		tokens.reverse();
		let (_, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
		let mut bytecode = (Opcode::ADD as u32) << opcode_offset;
		let (mode_mask, _) = EncoderDecoder::Mode.get_encoding();
		bytecode |= mode_mask;
		let (_, dr_offset) = EncoderDecoder::DR.get_encoding();
		bytecode |= (Register::R4 as u32) << dr_offset;
		let (_, rx_offset) = EncoderDecoder::RX.get_encoding();
		bytecode |= (Register::R7 as u32) << rx_offset;
		let (_, immed16_offset) = EncoderDecoder::Immed16.get_encoding();
		bytecode |= immed.get_value() << immed16_offset;
		assert_eq!(bytecode, get_bytecode(&mut tokens));
    }

	#[test]
	fn test_form5_instruction() {
		let immed = Literal::Immediate("0x12345".to_string());
		let mut tokens = vec![
				Token::Opcode(Opcode::MOV),
				Token::Register(Register::R4),
				Token::Seperator(Seperator::Comma),
				Token::Literal(immed.clone())
			];
		tokens.reverse();
		let (_, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
		let mut bytecode = (Opcode::MOV as u32) << opcode_offset;
		let (mode_mask, _) = EncoderDecoder::Mode.get_encoding();
		bytecode |= mode_mask;
		let (_, dr_offset) = EncoderDecoder::DR.get_encoding();
		bytecode |= (Register::R4 as u32) << dr_offset;
		let (_, immed20_offset) = EncoderDecoder::Immed20.get_encoding();
		bytecode |= immed.get_value() << immed20_offset;
		assert_eq!(bytecode, get_bytecode(&mut tokens));
	}

}

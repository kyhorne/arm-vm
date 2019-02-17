use super::super::util::{Opcode, Register, EncoderDecoder, Mask};
use super::lexer::{Token, Immediate};

struct Assembler{
	payload: u32
}

impl Assembler {
	pub fn new() -> Assembler {
		Assembler {
			payload: std::u32::MIN
		}
	}
	fn encode_opcode(&mut self, opcode: Opcode) {
		let (_, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
		self.payload |= (opcode as Mask) << opcode_offset;
	}
	fn encode_dr(&mut self, register: Register) {
		let (_, dr_offset) = EncoderDecoder::DR.get_encoding();
		self.payload |= (register as Mask) << dr_offset;
	}
	fn encode_rx(&mut self, register: Register) {
		let (_, rx_offset) = EncoderDecoder::RX.get_encoding();
		self.payload |= (register as Mask) << rx_offset;
	}
	fn encode_ry(&mut self, register: Register) {
		let (_, ry_offset) = EncoderDecoder::RY.get_encoding();
		self.payload |= (register as Mask) << ry_offset;
	}
	fn encode_immed16(&mut self, immed16: Immediate) {
		let (_, immed16_offset) = EncoderDecoder::Immed16.get_encoding();
		self.payload |= immed16.get_value() << immed16_offset;
	}
	fn encode_immed20(&mut self, immed20: Immediate) {
		let (_, immed20_offset) = EncoderDecoder::Immed20.get_encoding();
		self.payload |= immed20.get_value() << immed20_offset;
	}
}

macro_rules! to_bytecode {
	($opcode:expr; $dr:expr) => (
		{
			let mut assembler = Assembler::new();
			if let Some(Token::Opcode(opcode)) = $opcode {
				assembler.encode_opcode(opcode.clone());
			}
			if let Some(Token::Register(register)) = $dr {
				assembler.encode_dr(register.clone());
			}
			assembler
		}
	);
	($opcode:expr; $dr:expr; $op1:expr; $op2:expr) => (
		{
			let mut assembler = to_bytecode!($opcode; $dr);
			if let Some(Token::Register(op1)) = $op1 {
				assembler.encode_rx(op1.clone());
			}
			match $op2 {
				Some(Token::Register(register)) => assembler.encode_ry(register.clone()),
				Some(Token::Immediate(immed16)) => assembler.encode_immed16(immed16.clone()),
				_ => ()
			}
			assembler.payload
		}
	);
	($opcode:expr; $dr:expr; $op1:expr) => (
		{
			let mut assembler = to_bytecode!($opcode; $dr);
			match $op1 {
				Some(Token::Register(register)) => assembler.encode_rx(register.clone()),
				Some(Token::Immediate(immed20)) => assembler.encode_immed20(immed20.clone()),
				_ => ()
			}
			assembler.payload
		}
	)
}

pub fn run(tokens: &mut Vec<Token>) -> u32 {
	let mut optimized_tokens = tokens.iter().filter(|token| {
		match token {
			Token::Opcode(_) | Token::Register(_) | Token::Immediate(_) => true,
			_ => false
		}
	}).collect::<Vec<_>>();
	// TODO: fix bad code
	if optimized_tokens.len() == 4 {
		to_bytecode!(optimized_tokens.pop(); optimized_tokens.pop(); optimized_tokens.pop(); optimized_tokens.pop())
	} else {
		to_bytecode!(optimized_tokens.pop(); optimized_tokens.pop(); optimized_tokens.pop())
	}
}

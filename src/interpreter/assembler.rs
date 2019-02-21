use super::super::util::{EncoderDecoder, Form, Mask, Opcode, Register};
use super::lexer::{Literal, Token};

struct Assembler {
    payload: u32,
}

impl Assembler {
    fn new() -> Assembler {
        Assembler {
            payload: std::u32::MIN,
        }
    }
    /// Encode the opcode.
    fn encode_opcode(&mut self, form: Form, opcode: Opcode) {
        let (_, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
        if let Some(bytecode) = opcode.get_bytecode().get(&form) {
            self.payload |= bytecode << opcode_offset
        }
    }
    fn encode_bcc(&mut self, form: Form, opcode: Opcode) {
        let (_, bcc_offset) = EncoderDecoder::Bcc.get_encoding();
        if let Some(bytecode) = opcode.get_bytecode().get(&form) {
            self.payload |= bytecode << bcc_offset
        }
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
}

/// Get the bytecode encoding of tokens.
pub fn get_bytecode(tokens: &mut Vec<Token>, form: Form) -> u32 {
    // Remove tokens that are irrelevant to byte code encoding.
    let mut optimized_tokens = tokens
        .iter()
        .filter(|token| match token {
            Token::Opcode(_) | Token::Register(_) | Token::Literal(_) => true,
            _ => false,
        })
        .collect::<Vec<_>>();
    let opcode = optimized_tokens.pop();
    let dr = optimized_tokens.pop();
    let op1 = optimized_tokens.pop();
    let op2 = optimized_tokens.pop();
    // Convert the expression to bytecode.
    let mut assembler = Assembler::new();
    if let Some(Token::Opcode(opcode)) = opcode {
        if opcode.is_bcc() {
            assembler.encode_bcc(form, opcode.clone());
        } else {
            assembler.encode_opcode(form, opcode.clone());
        }
    }
    // Encode the destination register.
    if let Some(Token::Register(register)) = dr {
        assembler.encode_dr(register.clone());
    }
    // Encode operand two if it exists.
    if let Some(op2) = op2 {
        match op2 {
            Token::Register(register) => assembler.encode_ry(register.clone()),
            Token::Literal(immed16) => assembler.encode_immed16(immed16.clone()),
            _ => (),
        }
    }
    // Encode operand one if it exists.
    match op1 {
        Some(Token::Register(register)) => assembler.encode_rx(register.clone()),
        Some(Token::Literal(immed20)) => assembler.encode_immed20(immed20.clone()),
        _ => (),
    }
    assembler.payload
}

#[cfg(test)]
mod tests {}

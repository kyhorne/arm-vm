use super::super::util::{EncoderDecoder, Form, Mask, Opcode, Register};
use super::lexer::{Literal, Token};

struct Translator {
    payload: u32,
}

impl Translator {
    fn new() -> Translator {
        Translator {
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
    let mut assembler = Translator::new();
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
mod tests_translator {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::Literal::*;
    use super::*;

    #[test]
    fn test_encode_opcode() {
        let mut translator = Translator::new();
        translator.encode_opcode(One, ADD);
        assert_eq!(translator.payload, 0x01000000);
    }

    #[test]
    fn test_encode_bcc() {
        let mut translator = Translator::new();
        translator.encode_bcc(Six, BEQ);
        assert_eq!(translator.payload, 0x80100000);
    }

    #[test]
    fn test_encode_dr() {
        let mut translator = Translator::new();
        translator.encode_dr(PC);
        assert_eq!(translator.payload, 0x00F00000);
    }

    #[test]
    fn test_encode_rx() {
        let mut translator = Translator::new();
        translator.encode_rx(PC);
        assert_eq!(translator.payload, 0x000F0000);
    }

    #[test]
    fn test_encode_ry() {
        let mut translator = Translator::new();
        translator.encode_ry(PC);
        assert_eq!(translator.payload, 0x0000F000);
    }

    #[test]
    fn test_encode_immed16() {
        let mut translator = Translator::new();
        translator.encode_immed16(Immediate(String::from("0x1234")));
        assert_eq!(translator.payload, 0x00001234);
    }

    #[test]
    fn test_encode_immed20() {
        let mut translator = Translator::new();
        translator.encode_immed20(Immediate(String::from("0x12345")));
        assert_eq!(translator.payload, 0x00012345);
    }

}

#[cfg(test)]
mod tests_get_bytecode {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Literal::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_basic_form_one() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R1),
            Separator(Comma),
            Register(R2),
            Separator(Comma),
            Register(R3),
        ];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, One), 0x01123000);
    }

    #[test]
    fn test_ldr_str_form_one() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R1),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R2),
            Separator(Comma),
            Register(R3),
            Separator(CloseBrace),
        ];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, One), 0x36123000);
    }

    #[test]
    fn test_basic_form_two() {
        let mut tokens = vec![Opcode(MOV), Register(R1), Separator(Comma), Register(R2)];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, Two), 0x03120000);
    }

    #[test]
    fn test_ldr_str_form_two() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R1),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R2),
            Separator(CloseBrace),
        ];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, Two), 0x34120000);
    }

    #[test]
    fn test_basic_form_four() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R1),
            Separator(Comma),
            Register(R2),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, Four), 0x21121234);
    }

    #[test]
    fn test_ldr_str_form_four() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R1),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R2),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, Four), 0x35121234);
    }

    #[test]
    fn test_basic_form_five() {
        let mut tokens = vec![
            Opcode(MOV),
            Register(R1),
            Separator(Comma),
            Literal(Immediate(String::from("0x12345"))),
        ];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, Five), 0x23112345);
    }

    #[test]
    fn test_ldr_str_form_five() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R1),
            Separator(Comma),
            Separator(OpenBrace),
            Literal(Immediate(String::from("0x12345"))),
            Separator(CloseBrace),
        ];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, Five), 0x37112345);
    }

    #[test]
    fn test_form_six() {
        let mut tokens = vec![Opcode(BEQ)];
        tokens.reverse();
        assert_eq!(get_bytecode(&mut tokens, Six), 0x80100000);
    }

}

use super::super::util::{EncoderDecoder, Form};
use super::lexer::Token;

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
    let mut encoder = EncoderDecoder::new(None);
    if let Some(Token::Opcode(opcode)) = opcode {
        if opcode.is_bcc() {
            encoder.set_bcc(form, opcode.clone());
        } else {
            encoder.set_opcode(form, opcode.clone());
        }
    }
    // Encode the destination register.
    if let Some(Token::Register(register)) = dr {
        encoder.set_dr(register.clone());
    }
    // Encode operand two if it exists.
    if let Some(op2) = op2 {
        match op2 {
            Token::Register(register) => encoder.set_ry(register.clone()),
            Token::Literal(immed16) => encoder.set_immed16(immed16.clone()),
            _ => (),
        }
    }
    // Encode operand one if it exists.
    match op1 {
        Some(Token::Register(register)) => encoder.set_rx(register.clone()),
        Some(Token::Literal(immed20)) => encoder.set_immed20(immed20.clone()),
        _ => (),
    }
    encoder.get_instr()
}

#[cfg(test)]
mod tests_get_bytecode {

    use super::super::super::util::{Form::*, Literal::*, Opcode::*, Register::*};
    use super::super::lexer::{Separator::*, Token::*};
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

mod token;

use std::str::FromStr;

use super::super::util::{Opcode, Register};
pub use token::*;

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
        if let Ok(label) = Label::from_str(&token) {
            tokens.push(Token::Label(label));
            continue;
        }
    }
    return tokens;
}

#[cfg(test)]
mod tests {}

mod token;

use std::str::FromStr;

use super::super::util::{Opcode, Register};
pub use token::*;

/// Convert the soruce code into meaningful lexemes.
pub fn lexer(mut buf: String) -> Vec<Token> {
    // Pad separators with whitespace.
    buf = buf.replace(",", " , ");
    buf = buf.replace("[", " [ ");
    buf = buf.replace("]", " ] ");
    // Pad comments with whitespace.
    buf = buf.replace(";", " ; ");
    let mut tokens = Vec::new();
    // Iterate over string split by whitespace.
    for token in buf.split_whitespace() {
        if let Ok(opcode) = Opcode::from_str(&token) {
            tokens.push(Token::Opcode(opcode));
            continue;
        }
        if let Ok(register) = Register::from_str(&token) {
            tokens.push(Token::Register(register));
            continue;
        }
        if let Ok(separator) = Separator::from_str(&token) {
            tokens.push(Token::Separator(separator));
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
mod tests {

    use super::super::super::util::{Opcode::*, Register::*};
    use super::*;

    #[test]
    fn test_opcode_token() {
        let mut tokens = lexer(String::from("ADD"));
        if let Some(token) = tokens.pop() {
            assert_eq!(token, Token::Opcode(ADD))
        }
    }

    #[test]
    fn test_register_token() {
        let mut tokens = lexer(String::from("PC"));
        if let Some(token) = tokens.pop() {
            assert_eq!(token, Token::Register(PC))
        }
    }

    #[test]
    fn test_separator_token() {
        let mut tokens = lexer(String::from(","));
        if let Some(token) = tokens.pop() {
            assert_eq!(token, Token::Separator(Separator::Comma))
        }
    }

    #[test]
    fn test_literal_token() {
        let mut tokens = lexer(String::from("#0"));
        if let Some(token) = tokens.pop() {
            // Lexer automatically strips pound sign.
            assert_eq!(token, Token::Literal(Literal::Immediate(String::from("0"))))
        }
    }

    #[test]
    fn test_comment() {
        let mut tokens = lexer(String::from(";"));
        // Lexer drops comments.
        if let Some(_) = tokens.pop() {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn test_label_token() {
        let mut tokens = lexer(String::from("foo"));
        if let Some(token) = tokens.pop() {
            assert_eq!(token, Token::Label(Label::Name(String::from("foo"))))
        }
    }

    #[test]
    fn test_expr_after_comment() {
        let mut tokens = lexer(String::from("; ADD R0, R0, R0"));
        if let Some(_) = tokens.pop() {
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[test]
    fn test_expr_with_no_whitespace() {
        let tokens = vec![
            Token::Separator(Separator::Comma),
            Token::Separator(Separator::OpenBrace),
            Token::Separator(Separator::CloseBrace),
        ];
        assert_eq!(lexer(String::from(",[]")), tokens);
    }

}

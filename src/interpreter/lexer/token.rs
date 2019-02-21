use super::super::super::util::{Opcode, Register};

impl Literal {
    /// Check whether immediate has a valid prefix.
    pub fn is_valid(&mut self) -> bool {
        match self {
            Literal::Immediate(immed) => {
                let mut is_valid = immed.starts_with("#");
                immed.remove(0); // Remove prefix.
                                 // Ensure value is parable to u32.
                if let Err(_) = immed.parse::<u32>() {
                    is_valid = false
                }
                is_valid
            }
        }
    }
    /// Get immediate value.
    pub fn get_value(self) -> u32 {
        match self {
            Literal::Immediate(immed) => {
                if immed.contains("0x") {
                    // Get the value encoded as in base 16.
                    let immed = immed.trim_start_matches("0x");
                    return u32::from_str_radix(&immed, 16).unwrap();
                }
                immed.parse::<u32>().unwrap()
            }
        }
    }
}

#[derive(Clone, EnumString, Eq, Debug, PartialEq, ToString)]
pub enum Seperator {
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "[")]
    OpenBrace,
    #[strum(serialize = "]")]
    CloseBrace,
}

#[derive(Clone, EnumString, Eq, Debug, PartialEq, ToString)]
pub enum Comment {
    #[strum(serialize = ";")]
    Comment,
}

#[derive(Clone, EnumString, Eq, Debug, PartialEq, ToString)]
pub enum Literal {
    #[strum(default = "true")]
    Immediate(String),
}

#[derive(Clone, EnumString, Eq, Debug, Hash, PartialEq, ToString)]
pub enum Label {
    #[strum(default = "true")]
    Name(String),
}

#[derive(Clone, Debug, PartialEq, ToString)]
/// Parsable tokens.
pub enum Token {
    Opcode(Opcode),
    Register(Register),
    Literal(Literal),
    Seperator(Seperator),
    Label(Label),
}

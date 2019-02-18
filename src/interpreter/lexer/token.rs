use super::super::super::util::{Opcode, Register};

#[derive(
	Clone,
	EnumString,
	Eq,
	Debug,
	PartialEq
)]
pub enum Literal {
	#[strum(default="true")]
	Immediate(String)
}
impl Literal {
	/// Check whether immediate has a valid prefix.
	pub fn is_valid(&mut self) -> bool {
        match self {
            Literal::Immediate(immed) => {
				let is_valid = immed.starts_with("#");
				immed.remove(0); // Remove prefix
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
					return u32::from_str_radix(&immed, 16).unwrap()
				}
				immed.parse::<u32>().unwrap()
			}
		}
	}
}

#[cfg(test)]
mod tests {

	use super::*;

    #[test]
    fn test_is_valid() {
		// Test valid immediate value.
		let mut immed = Literal::Immediate("#123".to_string());
		assert!(immed.is_valid());
		// Test invalid immediate value.
		immed = Literal::Immediate("456".to_string());
		assert!(!immed.is_valid());
	}

	#[test]
    fn test_get_value() {
		// Test immediate value encoded in base 10.
		let mut immed = Literal::Immediate("123".to_string());
		assert_eq!(immed.get_value(), 123);
		// Test immediate value encoded in base 16.
		immed = Literal::Immediate("0x999".to_string());
		assert_eq!(immed.get_value(), 0x999);
	}


}

#[derive(
	Clone,
	EnumString,
	Eq,
	Debug,
	PartialEq
)]
pub enum Seperator {
	#[strum(serialize=",")]
	Comma,
	#[strum(serialize="[")]
	OpenBrace,
	#[strum(serialize="]")]
	CloseBrace
}

#[derive(
	Clone,
	EnumString,
	Eq,
	Debug,
	PartialEq
)]
pub enum Comment {
	#[strum(serialize=";")]
	Comment
}

#[derive(
	Clone,
	Debug,
	PartialEq
)]
/// Parsable tokens.
pub enum Token {
	Opcode(Opcode),
	Register(Register),
	Literal(Literal),
	Seperator(Seperator),
}

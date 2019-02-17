use super::super::super::util::{Opcode, Register};

#[derive(
	Clone,
	EnumString,
	Eq,
	Debug,
	PartialEq
)]
pub enum Immediate {
	#[strum(default="true")]
	Value(String)
}
impl Immediate {
	pub fn is_valid(&mut self) -> bool {
        match self {
            Immediate::Value(value) => {
				let is_valid = value.starts_with("#");
				value.remove(0); // Remove prefix
				is_valid
			}
        }
    }
	pub fn get_value(self) -> u32 {
		match self {
			Immediate::Value(value) => {
				if value.contains("0x") {
					let value = value.trim_start_matches("0x");
					return u32::from_str_radix(&value, 16).unwrap()
				}
				value.parse::<u32>().unwrap()
			}
		}
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
	CloseBrace,
	#[strum(serialize="{")]
	OpenBracket,
	#[strum(serialize="}")]
	CloseBracket
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
	EnumString,
	Eq,
	Debug,
	PartialEq,
	Clone
)]
pub enum Label {
	#[strum(default="true")]
	Identifier(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
	Opcode(Opcode),
	Register(Register),
	Immediate(Immediate),
	Seperator(Seperator),
	Label(Label)
}

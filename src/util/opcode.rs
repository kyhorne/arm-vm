use crate::util::form::Form;
use num_derive::FromPrimitive;

#[derive(
	Clone,
	EnumString,
	Eq,
	Debug,
	PartialEq,
	FromPrimitive
)]
pub enum Opcode {
	#[strum(
		serialize="ADD",
		serialize="add"
	)]
	ADD = 0x1,
	#[strum(
		serialize="AND",
		serialize="and"
	)]
	AND,
	#[strum(
		serialize="EOR",
		serialize="eor"
	)]
	EOR,
	#[strum(
		serialize="MUL",
		serialize="mul"
	)]
	MUL,
	#[strum(
		serialize="ORR",
		serialize="orr"
	)]
	ORR,
	#[strum(
		serialize="SUB",
		serialize="sub"
	)]
	SUB,
	#[strum(
		serialize="MOV",
		serialize="mov"
	)]
	MOV,
	#[strum(
		serialize="MVN",
		serialize="mvn"
	)]
	MVN,
	#[strum(
		serialize="STR",
		serialize="str"
	)]
	STR,
	#[strum(
		serialize="LDR",
		serialize="ldr"
	)]
	LDR
}

impl Opcode {
	/// Get the forms associated with a given opcode.
	pub fn get_forms(&self) -> Vec<Form> {
		match *self {
			Opcode::ADD | Opcode::AND | Opcode::EOR | Opcode::MUL | Opcode::ORR | Opcode::SUB
				=> vec![Form::One, Form::Four],
			Opcode::MOV | Opcode::MVN => vec![Form::Two, Form::Five],
			Opcode::STR | Opcode::LDR => vec![Form::One, Form::Two, Form::Four]
		}
	}
}

use crate::util::opcode::Opcode;

#[derive(
	Clone,
	Debug,
	PartialEq
)]
pub enum Form {
	/// A form one instruction has the following encoding scheme:
	/// OP, DR, RX, RY ; DR <- [RX] OP [RY]
	///
	/// # Examples:
	/// ```
	///	ADD R5, R1, R10
	/// XOR R4, R4, R4
	/// ```
	One,
	/// A form one instruction has the following encoding scheme:
	/// OP, DR, RX ; DR <- OP([RX])
	///
	/// # Examples:
	/// ```
	/// MOV R5, R1
	/// MVN R10, R11
	/// ```
	Two,
	/// A form four instruction has the following encoding scheme:
	/// OP, DR, RX, #immed16 ; DR <- [RX] OP #immed16
	///
	/// # Examples:
	/// ```
	///	ADD R5, R1, #10
	///	AND R4, R4, #0x1
	/// ```
	Four,
	/// A form five instruction has the following encoding scheme:
	/// OP, DR, #immed20
	///
	/// # Examples:
	/// ```
	///	MOV R5, #0xF1234
	/// MVN R9, #0x0
	/// ```
	Five
}

impl Form {
	/// Get the expression length associated with a given form and opcode.
	pub fn get_expr_length(&self, opcode: &Opcode) -> usize {
		let mut delta = 0;
		match *opcode {
			Opcode::STR | Opcode::LDR => delta = 2,
			_ => ()

		}
		match *self {
			Form::One | Form::Four => 6 + delta,
			Form::Two | Form::Five => 4 + delta
		}
	}
}

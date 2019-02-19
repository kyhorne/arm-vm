use crate::util::register::get_name;
use crate::vm::Address;
use crate::vm::Payload;
use super::form::Form;
use super::opcode::Opcode;
use num_traits::FromPrimitive;

/// Mask defines the bits used to access specific data from an instruction.
pub type Mask = u32;

// Offset defines the bits needed to right shift data to the least significant bit.
pub type Offset = u8;

/// EncoderDecoder list the types of information encoded into an instruction.
pub enum EncoderDecoder {
	Opcode,   /* The opcode bits define the type of operation to execute. */
	DR,       /* The address of the destination register. */
	Mode,     /* The mode bit specifies whether the instruction is in immediate mode or register
	           * mode. */
	RX,       /* The address of the register for the first operand. */
	RY,       /* The address of the register for the second operand. */
	Immed16,  /* The immediate 16-bit value of the second operand. */
	Immed20   /* The immediate 20-bit value of the second operand. */
}

impl EncoderDecoder {
	/// Get the mask and offset needed to insert or extract relevant data to or from an instruction.
	pub fn get_encoding(self) -> (Mask, Offset) {
		match self {
			/* The opcode is encoded in the two most significant bytes excluding the mode bit. */
			EncoderDecoder::Opcode  => (0xDF000000, 0x18),
			/* The address of the destination register is encoded in the third most significant
			 * byte. */
			EncoderDecoder::DR      => (0x00F00000, 0x14),
			/* The mode bit is encoded in the 29th bit. It is encoded as part of the opcode. */
			EncoderDecoder::Mode    => (0x20000000, 0x1C),
			/* The address of the register for the first operand is encoded in the fourth most
			 * significant byte. */
			EncoderDecoder::RX      => (0x000F0000, 0x10),
			/* The address of the register for the second operand is encoded in the fourth least
			 * significant byte. */
			EncoderDecoder::RY      => (0x0000F000, 0x0C),
			/* The immediate 16-bit value is encoded in the four least significant bytes. */
			EncoderDecoder::Immed16 => (0x0000FFFF, 0x00),
			/* The immediate 16-bit value is encoded in the four least significant bytes. */
			EncoderDecoder::Immed20 => (0x000FFFFF, 0x00)
		}
	}
}

/// Check whether the mode bit is toggled for a given payload.
pub fn is_mode_bit_toggled(payload: Payload) -> bool {
	let (mode_mask, _) = EncoderDecoder::Mode.get_encoding();
	payload & mode_mask != 0
}

// Parse the address of the destination register from a payload.
pub fn get_dr_addr(payload: Payload) -> Address {
	let (dr_mask, dr_offset) = EncoderDecoder::DR.get_encoding();
	let dr_addr = ((payload & dr_mask) >> dr_offset) as Address;
	println!("{:19}{}", "Dr: ", get_name(dr_addr));
	dr_addr
}

// Parse the address of register x from a payload.
pub fn get_rx_addr(payload: Payload) -> Address {
	let (rx_mask, rx_offset) = EncoderDecoder::RX.get_encoding();
	((payload & rx_mask) >> rx_offset) as Address
}

// Parse the address of register y from a payload.
pub fn get_ry_addr(payload: Payload) -> Address {
	let (ry_mask, ry_offset) = EncoderDecoder::RY.get_encoding();
	((payload & ry_mask) >> ry_offset) as Address
}

// Parse the immediate 20-bit value from a payload.
pub fn get_immed16(payload: Payload) -> Payload {
	let (immed16_mask, immed16_offset) = EncoderDecoder::Immed16.get_encoding();
	let immed16 = (payload & immed16_mask) >> immed16_offset;
	println!("{:24}{:#010X}", "Immed16: ", immed16);
	immed16
}

// Parse the immediate 20-bit value from a payload.
pub fn get_immed20(payload: Payload) -> Payload {
	let (immed20_mask, immed20_offset) = EncoderDecoder::Immed20.get_encoding();
	let immed20 = (payload & immed20_mask) >> immed20_offset;
	println!("{:24}{:#010X}", "Immed20: ", immed20);
	immed20
}

// Get the encoded opcode and its form from a given payload.
pub fn get_form_and_opcode(payload: Payload) -> Result<(Opcode, Form), ()> {
	// Extract the opcode from the payload.
	let (opcode_mask, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
	let opcode = (payload & opcode_mask) >> opcode_offset;
	if let Some(opcode) = FromPrimitive::from_u32(opcode) {
		match opcode {
			Opcode::ADD | Opcode::AND | Opcode::EOR | Opcode::MUL | Opcode::ORR | Opcode::SUB => {
				if is_mode_bit_toggled(payload) {
					return Ok((opcode, Form::Four))
				} else {
					return Ok((opcode, Form::One))
				};
			}
			Opcode::MOV | Opcode::MVN => {
				if is_mode_bit_toggled(payload) {
					return Ok((opcode, Form::Five))
				} else {
					return Ok((opcode, Form::Two))
				};
			}
			_ => ()
		}
	}
	return Err(())
}

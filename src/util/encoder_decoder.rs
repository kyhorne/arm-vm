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

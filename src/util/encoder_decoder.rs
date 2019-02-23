use crate::util::opcode::Form;
use crate::util::opcode::Opcode;
use crate::vm::Address;
use crate::vm::Payload;

/// Mask defines the bits used to access specific data from an instruction.
pub type Mask = u32;

// Offset defines the bits needed to right shift data to the least significant bit.
pub type Offset = u8;

/// EncoderDecoder list the types of information encoded into an instruction.
pub enum EncoderDecoder {
    Opcode, /* The opcode bits define the type of operation to execute. */
    Bcc,
    DR,      /* The address of the destination register. */
    RX,      /* The address of the register for the first operand. */
    RY,      /* The address of the register for the second operand. */
    Immed16, /* The immediate 16-bit value of the second operand. */
    Immed20, /* The immediate 20-bit value of the second operand. */
}

impl EncoderDecoder {
    /// Get the mask and offset needed to insert or extract relevant data to or from an instruction.
    pub fn get_encoding(self) -> (Mask, Offset) {
        match self {
            /* The opcode is encoded in the two most significant bytes. */
            EncoderDecoder::Opcode => (0xFF000000, 0x18),
            EncoderDecoder::Bcc => (0xFFF00000, 0x14),
            /* The address of the destination register is encoded in the third most significant
             * byte. */
            EncoderDecoder::DR => (0x00F00000, 0x14),
            /* The address of the register for the first operand is encoded in the fourth most
             * significant byte. */
            EncoderDecoder::RX => (0x000F0000, 0x10),
            /* The address of the register for the second operand is encoded in the fourth least
             * significant byte. */
            EncoderDecoder::RY => (0x0000F000, 0x0C),
            /* The immediate 16-bit value is encoded in the four least significant bytes. */
            EncoderDecoder::Immed16 => (0x0000FFFF, 0x00),
            /* The immediate 16-bit value is encoded in the four least significant bytes. */
            EncoderDecoder::Immed20 => (0x000FFFFF, 0x00),
        }
    }
}

pub fn get_form_and_opcode(payload: Payload) -> Result<((Form, Opcode)), ()> {
    let (opcode_mask, opcode_offset) = EncoderDecoder::Opcode.get_encoding();
    let bytecode = ((payload & opcode_mask) >> opcode_offset) as u32;
    Opcode::get_opcode(bytecode)
}

pub fn get_form_and_bcc(payload: Payload) -> Result<((Form, Opcode)), ()> {
    let (opcode_mask, opcode_offset) = EncoderDecoder::Bcc.get_encoding();
    let bytecode = ((payload & opcode_mask) >> opcode_offset) as u32;
    Opcode::get_opcode(bytecode)
}

// Parse the address of the destination register from a payload.
pub fn get_dr_addr(payload: Payload) -> Address {
    let (dr_mask, dr_offset) = EncoderDecoder::DR.get_encoding();
    let dr_addr = ((payload & dr_mask) >> dr_offset) as Address;
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
    println!("{:30}{:#010X}", "Immed16: ", immed16);
    immed16
}

// Parse the immediate 20-bit value from a payload.
pub fn get_immed20(payload: Payload) -> Payload {
    let (immed20_mask, immed20_offset) = EncoderDecoder::Immed20.get_encoding();
    let immed20 = (payload & immed20_mask) >> immed20_offset;
    println!("{:30}{:#010X}", "Immed20: ", immed20);
    immed20
}

#[cfg(test)]
mod tests_get_form_and_opcode {

    use super::*;

    #[test]
    fn test_is_ok() {
        if let Ok(_) = get_form_and_opcode(0x01000000) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_is_err() {
        if let Err(_) = get_form_and_opcode(0x99000000) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_opcode() {
        if let Ok((_, opcode)) = get_form_and_opcode(0x01000000) {
            assert_eq!(opcode, Opcode::ADD);
        }
    }

    #[test]
    fn test_form() {
        if let Ok((form, _)) = get_form_and_opcode(0x01000000) {
            assert_eq!(form, Form::One);
        }
    }

}

#[cfg(test)]
mod tests_get_form_and_bcc {

    use super::*;

    #[test]
    fn test_is_ok() {
        if let Ok(_) = get_form_and_bcc(0x80100000) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_is_err() {
        if let Err(_) = get_form_and_bcc(0x99000000) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_opcode() {
        if let Ok((_, opcode)) = get_form_and_bcc(0x80100000) {
            assert_eq!(opcode, Opcode::BEQ);
        }
    }

    #[test]
    fn test_form() {
        if let Ok((form, _)) = get_form_and_bcc(0x80100000) {
            assert_eq!(form, Form::Six);
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_dr_addr() {
        assert_eq!(get_dr_addr(0x00100000), 0x1);
    }

    #[test]
    fn test_get_rx_addr() {
        assert_eq!(get_rx_addr(0x00010000), 0x1);
    }

    #[test]
    fn test_get_ry_addr() {
        assert_eq!(get_ry_addr(0x00001000), 0x1);
    }

    #[test]
    fn test_get_immed16() {
        assert_eq!(get_immed16(0x00001234), 0x1234);
    }

    #[test]
    fn test_get_immed20() {
        assert_eq!(get_immed20(0x00012345), 0x12345);
    }

}

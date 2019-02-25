use super::{ConditionCode, Literal, Register};
use crate::util::opcode::*;
use crate::vm::Address;

/// Mask defines the bits used to access specific data from an instruction.
pub type Mask = u32;

// Offset defines the bits needed to right shift data to the least significant bit.
pub type Offset = u8;

/// Payload list the types of information encoded into an instruction.
pub enum Payload {
    Opcode,  /* The opcode bits define the type of operation to execute. */
    CC,      /* The condition code. */
    DR,      /* The address of the destination register. */
    RX,      /* The address of the register for the first operand. */
    RY,      /* The address of the register for the second operand. */
    Immed16, /* The immediate 16-bit value of the second operand. */
    Immed20, /* The immediate 20-bit value of the second operand. */
}

impl Payload {
    /// Get the mask and offset needed to encoded or decode relevant data to or from an instruction.
    pub fn get_mask_and_offset(self) -> (Mask, Offset) {
        match self {
            /* The opcode is encoded in the two most significant bytes. */
            Payload::Opcode => (0xFF000000, 0x18),
            Payload::CC => (0x00F00000, 0x14),
            /* The address of the destination register is encoded in the third most significant
             * byte. */
            Payload::DR => (0x00F00000, 0x14),
            /* The address of the register for the first operand is encoded in the fourth most
             * significant byte. */
            Payload::RX => (0x000F0000, 0x10),
            /* The address of the register for the second operand is encoded in the fourth least
             * significant byte. */
            Payload::RY => (0x0000F000, 0x0C),
            /* The immediate 16-bit value is encoded in the four least significant bytes. */
            Payload::Immed16 => (0x0000FFFF, 0x00),
            /* The immediate 16-bit value is encoded in the four least significant bytes. */
            Payload::Immed20 => (0x000FFFFF, 0x00),
        }
    }
}

pub type Instruction = u32;

pub struct EncoderDecoder {
    instr: Instruction,
}

impl EncoderDecoder {
    pub fn new(init: Option<u32>) -> EncoderDecoder {
        if let Some(init) = init {
            println!("{:17}{:>8} = {:#010X} ", "Instruction:", "MMem[[PC]]", init);
            EncoderDecoder { instr: init }
        } else {
            EncoderDecoder {
                instr: std::u32::MIN,
            }
        }
    }
    pub fn get_instr(&self) -> Instruction {
        self.instr
    }
    /// Encode the opcode.
    pub fn set_opcode(&mut self, form: Form, opcode: Opcode) {
        let (_, opcode_offset) = Payload::Opcode.get_mask_and_offset();
        if let Some(bytecode) = opcode.get_bytecode().get(&form) {
            self.instr |= bytecode << opcode_offset
        }
    }
    /// Encode the condition code.
    pub fn set_cc(&mut self, cond_code: ConditionCode) {
        let (_, cc_offset) = Payload::CC.get_mask_and_offset();
        self.instr |= (cond_code as Mask) << cc_offset
    }
    /// Encode the destination register.
    pub fn set_dr(&mut self, register: Register) {
        let (_, dr_offset) = Payload::DR.get_mask_and_offset();
        self.instr |= (register as Mask) << dr_offset
    }
    /// Encode operand one as a register address.
    pub fn set_rx(&mut self, register: Register) {
        let (_, rx_offset) = Payload::RX.get_mask_and_offset();
        self.instr |= (register as Mask) << rx_offset
    }
    /// Encode operand two as a register address.
    pub fn set_ry(&mut self, register: Register) {
        let (_, ry_offset) = Payload::RY.get_mask_and_offset();
        self.instr |= (register as Mask) << ry_offset
    }
    /// Encode operand two as a immediate 16-bit value.
    pub fn set_immed16(&mut self, immed16: Literal) {
        let (_, immed16_offset) = Payload::Immed16.get_mask_and_offset();
        self.instr |= immed16.get_value() << immed16_offset
    }
    /// Encode operand one as a immediate 20-bit value.
    pub fn set_immed20(&mut self, immed20: Literal) {
        let (_, immed20_offset) = Payload::Immed20.get_mask_and_offset();
        self.instr |= immed20.get_value() << immed20_offset
    }
    // TODO: Explanation.
    pub fn get_form_and_opcode(&mut self) -> Result<((Form, Opcode)), ()> {
        let (opcode_mask, opcode_offset) = Payload::Opcode.get_mask_and_offset();
        let bytecode = ((self.instr & opcode_mask) >> opcode_offset) as u32;
        Opcode::get_opcode(bytecode)
    }
    // TODO: Explanation.
    pub fn get_cc(&mut self) -> ConditionCode {
        let (opcode_mask, opcode_offset) = Payload::CC.get_mask_and_offset();
        let cc = ((self.instr & opcode_mask) >> opcode_offset) as usize;
        ConditionCode::get_cc(cc)
    }
    // Parse the address of the destination register from an instruction.
    pub fn get_dr(&mut self) -> Address {
        let (dr_mask, dr_offset) = Payload::DR.get_mask_and_offset();
        ((self.instr & dr_mask) >> dr_offset) as Address
    }
    // Parse the address of register x from an instruction.
    pub fn get_rx(&mut self) -> Address {
        let (rx_mask, rx_offset) = Payload::RX.get_mask_and_offset();
        ((self.instr & rx_mask) >> rx_offset) as Address
    }
    // Parse the address of register y from an instruction.
    pub fn get_ry(&mut self) -> Address {
        let (ry_mask, ry_offset) = Payload::RY.get_mask_and_offset();
        ((self.instr & ry_mask) >> ry_offset) as Address
    }
    // Parse the immediate 20-bit value from an instruction.
    pub fn get_immed16(&mut self) -> Instruction {
        let (immed16_mask, immed16_offset) = Payload::Immed16.get_mask_and_offset();
        let immed16 = (self.instr & immed16_mask) >> immed16_offset;
        println!("{:30}{:#010X}", "Immed16: ", immed16);
        immed16
    }
    // Parse the immediate 20-bit value from an instruction.
    pub fn get_immed20(&mut self) -> Instruction {
        let (immed20_mask, immed20_offset) = Payload::Immed20.get_mask_and_offset();
        let immed20 = (self.instr & immed20_mask) >> immed20_offset;
        println!("{:30}{:#010X}", "Immed20: ", immed20);
        immed20
    }
}

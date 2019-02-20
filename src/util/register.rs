use core::mem::transmute;
use strum::AsStaticRef;

use num_derive::FromPrimitive;

#[derive(Clone, EnumString, Eq, Debug, PartialEq, FromPrimitive, AsStaticStr)]
pub enum Register {
    #[strum(serialize = "R0", serialize = "r0")]
    R0 = 0x0,
    #[strum(serialize = "R1", serialize = "r1")]
    R1,
    #[strum(serialize = "R2", serialize = "r2")]
    R2,
    #[strum(serialize = "R3", serialize = "r3")]
    R3,
    #[strum(serialize = "R4", serialize = "r4")]
    R4,
    #[strum(serialize = "R5", serialize = "r5")]
    R5,
    #[strum(serialize = "R6", serialize = "r6")]
    R6,
    #[strum(serialize = "R7", serialize = "r7")]
    R7,
    #[strum(serialize = "R8", serialize = "r8")]
    R8,
    #[strum(serialize = "R9", serialize = "r9")]
    R9,
    #[strum(serialize = "R10", serialize = "r10")]
    R10,
    #[strum(serialize = "R11", serialize = "r11")]
    R11,
    #[strum(serialize = "R12", serialize = "r12")]
    R12,
    #[strum(serialize = "SP", serialize = "sp")]
    SP, // Stack pointer.
    #[strum(serialize = "LR", serialize = "lr")]
    LR, // Link register.
    #[strum(serialize = "PC", serialize = "pc")]
    PC, // Program counter.
}

/// Get the register name from a given address.
pub fn get_name(addr: usize) -> String {
    let register: Register = unsafe { transmute(addr as u8) };
    register.as_static().to_uppercase()
}

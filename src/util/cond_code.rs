use core::mem::transmute;
use num_derive::FromPrimitive;

#[derive(Clone, EnumString, Eq, Debug, PartialEq, FromPrimitive)]
pub enum ConditionCode {
    #[strum(serialize = "BAL", serialize = "bal", serialize = "B", serialize = "b")]
    AL = 0x0,
    #[strum(serialize = "BEQ", serialize = "beq")]
    EQ,
    #[strum(serialize = "BNE", serialize = "bne")]
    NE,
    #[strum(serialize = "BHS", serialize = "bhs")]
    HS,
    #[strum(serialize = "BLO", serialize = "blo")]
    LO,
    #[strum(serialize = "BMI", serialize = "bmi")]
    MI,
    #[strum(serialize = "BPL", serialize = "bpl")]
    PL,
    #[strum(serialize = "BVS", serialize = "bvs")]
    VS,
    #[strum(serialize = "BVC", serialize = "bvc")]
    VC,
    #[strum(serialize = "BHI", serialize = "bhi")]
    HI,
    #[strum(serialize = "BLS", serialize = "bls")]
    LS,
    #[strum(serialize = "BGE", serialize = "bge")]
    GE,
    #[strum(serialize = "BLT", serialize = "blt")]
    LT,
    #[strum(serialize = "BGT", serialize = "bgt")]
    GT,
    #[strum(serialize = "BLE", serialize = "ble")]
    LE,
}

impl ConditionCode {
    pub fn get_cc(addr: usize) -> ConditionCode {
        let cc: ConditionCode = unsafe { transmute(addr as u8) };
        cc
    }
}

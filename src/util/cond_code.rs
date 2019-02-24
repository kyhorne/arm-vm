use core::mem::transmute;
use num_derive::FromPrimitive;

#[derive(Clone, EnumString, Eq, Debug, PartialEq, FromPrimitive)]
pub enum ConditionCode {
    #[strum(serialize = "AL", serialize = "al")]
    AL = 0x0,
    #[strum(serialize = "EQ", serialize = "eq")]
    EQ,
    #[strum(serialize = "NE", serialize = "ne")]
    NE,
    #[strum(serialize = "HS", serialize = "hs")]
    HS,
    #[strum(serialize = "LO", serialize = "lo")]
    LO,
    #[strum(serialize = "MI", serialize = "mi")]
    MI,
    #[strum(serialize = "PL", serialize = "pl")]
    PL,
    #[strum(serialize = "VS", serialize = "vs")]
    VS,
    #[strum(serialize = "VC", serialize = "vc")]
    VC,
    #[strum(serialize = "HI", serialize = "hi")]
    HI,
    #[strum(serialize = "LS", serialize = "ls")]
    LS,
    #[strum(serialize = "GE", serialize = "ge")]
    GE,
    #[strum(serialize = "LT", serialize = "lt")]
    LT,
    #[strum(serialize = "GT", serialize = "gt")]
    GT,
    #[strum(serialize = "LE", serialize = "le")]
    LE,
}

impl ConditionCode {
    pub fn get_cc(addr: usize) -> ConditionCode {
        let cc: ConditionCode = unsafe { transmute(addr as u8) };
        cc
    }
}

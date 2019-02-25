use super::super::super::util::{ConditionCode, Literal, Opcode, Register};

#[derive(Clone, EnumString, Eq, Debug, PartialEq, ToString)]
pub enum Separator {
    #[strum(serialize = ",")]
    Comma,
    #[strum(serialize = "[")]
    OpenBrace,
    #[strum(serialize = "]")]
    CloseBrace,
}

#[derive(Clone, EnumString, Eq, Debug, PartialEq, ToString)]
pub enum Comment {
    #[strum(serialize = ";")]
    Comment,
}

#[derive(Clone, EnumString, Eq, Debug, Hash, PartialEq, ToString)]
pub enum Label {
    #[strum(default = "true")]
    Name(String),
}

#[derive(Clone, Debug, PartialEq, ToString)]
pub enum Token {
    Opcode(Opcode),
    ConditionCode(ConditionCode),
    Register(Register),
    Literal(Literal),
    Separator(Separator),
    Label(Label),
}

use super::super::super::util::Form;
use super::super::lexer::Token;
use super::super::parser::{ConditionCodeState, OpcodeState, RegisterState, StateMachine};

impl From<StateMachine<OpcodeState>> for StateMachine<RegisterState> {
    fn from(machine: StateMachine<OpcodeState>) -> StateMachine<RegisterState> {
        StateMachine {
            state: RegisterState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl From<StateMachine<OpcodeState>> for StateMachine<ConditionCodeState> {
    fn from(machine: StateMachine<OpcodeState>) -> StateMachine<ConditionCodeState> {
        StateMachine {
            state: ConditionCodeState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl StateMachine<OpcodeState> {
    pub fn handler(mut self) -> Result<Option<Form>, ()> {
        match self.tokens.pop() {
            Some(Token::Register(_)) => return StateMachine::<RegisterState>::from(self).handler(),
            Some(Token::ConditionCode(_)) => {
                return StateMachine::<ConditionCodeState>::from(self).handler();
            }
            _ => (),
        }
        return Err(());
    }
}

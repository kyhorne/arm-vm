use super::super::super::util::Form;
use super::super::lexer::{Label, Token};
use super::super::parser::{OpcodeState, RegisterState, StateMachine};

impl From<StateMachine<OpcodeState>> for StateMachine<RegisterState> {
    fn from(machine: StateMachine<OpcodeState>) -> StateMachine<RegisterState> {
        StateMachine {
            state: RegisterState,
            tokens: machine.tokens,
            forms: machine.forms,
            labels: machine.labels,
        }
    }
}

impl StateMachine<OpcodeState> {
    pub fn handler(mut self) -> Result<(Form, Vec<Label>), ()> {
        if let Some(Token::Register(_)) = self.tokens.pop() {
            return StateMachine::<RegisterState>::from(self).handler();
        }
        return Err(());
    }
}

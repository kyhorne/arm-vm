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
    pub fn handler(mut self) -> Result<(Option<Form>, Vec<Label>), ()> {
        match self.tokens.pop() {
            Some(Token::Register(_)) => return StateMachine::<RegisterState>::from(self).handler(),
            Some(Token::Label(label)) => {
                self.labels.push(label);
                if self.forms.contains(&Form::Six) {
                    return Ok((Some(Form::Six), self.labels));
                }
                return Err(());
            }
            _ => return Err(()),
        }
    }
}

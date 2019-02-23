use super::super::super::util::Form;
use super::super::lexer::{Label, Token};
use super::super::parser::{ImmediateState, OpenBraceState, RegisterState, StateMachine};

impl From<StateMachine<OpenBraceState>> for StateMachine<RegisterState> {
    fn from(machine: StateMachine<OpenBraceState>) -> StateMachine<RegisterState> {
        StateMachine {
            state: RegisterState,
            tokens: machine.tokens,
            forms: machine.forms,
            label: machine.label,
        }
    }
}

impl From<StateMachine<OpenBraceState>> for StateMachine<ImmediateState> {
    fn from(machine: StateMachine<OpenBraceState>) -> StateMachine<ImmediateState> {
        StateMachine {
            state: ImmediateState,
            tokens: machine.tokens,
            forms: machine.forms,
            label: machine.label,
        }
    }
}

impl StateMachine<OpenBraceState> {
    pub fn handler(mut self) -> Result<(Option<Form>, Option<Label>), ()> {
        match self.tokens.pop() {
            Some(Token::Literal(_)) => StateMachine::<ImmediateState>::from(self).handler(),
            Some(Token::Register(_)) => StateMachine::<RegisterState>::from(self).handler(),
            _ => Err(()),
        }
    }
}

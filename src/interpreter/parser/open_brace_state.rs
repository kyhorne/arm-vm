use super::super::super::util::Form;
use super::super::lexer::{Label, Token};
use super::super::parser::{ImmediateState, OpenBrace, RegisterState, StateMachine};

impl From<StateMachine<OpenBrace>> for StateMachine<RegisterState> {
    fn from(machine: StateMachine<OpenBrace>) -> StateMachine<RegisterState> {
        StateMachine {
            state: RegisterState,
            tokens: machine.tokens,
            forms: machine.forms,
            labels: machine.labels,
        }
    }
}

impl From<StateMachine<OpenBrace>> for StateMachine<ImmediateState> {
    fn from(machine: StateMachine<OpenBrace>) -> StateMachine<ImmediateState> {
        StateMachine {
            state: ImmediateState,
            tokens: machine.tokens,
            forms: machine.forms,
            labels: machine.labels,
        }
    }
}

impl StateMachine<OpenBrace> {
    pub fn handler(mut self) -> Result<(Form, Vec<Label>), ()> {
        match self.tokens.pop() {
            Some(Token::Literal(_)) => StateMachine::<ImmediateState>::from(self).handler(),
            Some(Token::Register(_)) => StateMachine::<RegisterState>::from(self).handler(),
            _ => Err(()),
        }
    }
}

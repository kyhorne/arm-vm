use super::super::super::util::{reducer, Form};
use super::super::lexer::Token;
pub use super::super::parser::StateMachine;
use super::super::parser::{LabelState, OpcodeState};

impl From<StateMachine<LabelState>> for StateMachine<OpcodeState> {
    fn from(machine: StateMachine<LabelState>) -> StateMachine<OpcodeState> {
        StateMachine {
            state: OpcodeState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl StateMachine<LabelState> {
    pub fn handler(mut self) -> Result<Option<Form>, ()> {
        match self.tokens.pop() {
            Some(Token::Opcode(opcode)) => {
                self.forms = reducer(opcode.get_forms(), &opcode, self.tokens.len() + 1);
                if self.forms.is_empty() {
                    return Err(());
                }
                return StateMachine::<OpcodeState>::from(self).handler();
            }
            None => return Ok(None),
            _ => return Err(()),
        }
    }
}

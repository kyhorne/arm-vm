use super::super::super::util::{reducer, Form};
use super::super::lexer::{Label, Token};
pub use super::super::parser::StateMachine;
use super::super::parser::{LabelState, OpcodeState, ReadyState};

impl From<StateMachine<ReadyState>> for StateMachine<OpcodeState> {
    fn from(machine: StateMachine<ReadyState>) -> StateMachine<OpcodeState> {
        StateMachine {
            state: OpcodeState,
            tokens: machine.tokens,
            forms: machine.forms,
            labels: machine.labels,
        }
    }
}

impl From<StateMachine<ReadyState>> for StateMachine<LabelState> {
    fn from(machine: StateMachine<ReadyState>) -> StateMachine<LabelState> {
        StateMachine {
            state: LabelState,
            tokens: machine.tokens,
            forms: machine.forms,
            labels: machine.labels,
        }
    }
}

impl StateMachine<ReadyState> {
    pub fn new(tokens: Vec<Token>) -> Self {
        StateMachine {
            state: ReadyState,
            tokens: tokens,
            forms: Vec::new(),
            labels: Vec::new(),
        }
    }
    pub fn handler(mut self) -> Result<(Option<Form>, Vec<Label>), ()> {
        match self.tokens.pop() {
            Some(Token::Opcode(opcode)) => {
                self.forms = reducer(opcode.get_forms(), opcode, self.tokens.len() + 1);
                if self.forms.is_empty() {
                    return Err(());
                }
                return StateMachine::<OpcodeState>::from(self).handler();
            }
            Some(Token::Label(label)) => {
                self.labels.push(label);
                return StateMachine::<LabelState>::from(self).handler();
            }
            _ => return Err(()),
        }
    }
}

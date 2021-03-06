use super::super::super::util::{reducer, Form, Opcode};
use super::super::lexer::{Separator, Token};
pub use super::super::parser::StateMachine;
use super::super::parser::{LabelState, OpcodeState, ReadyState};

impl From<StateMachine<ReadyState>> for StateMachine<OpcodeState> {
    fn from(machine: StateMachine<ReadyState>) -> StateMachine<OpcodeState> {
        StateMachine {
            state: OpcodeState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl From<StateMachine<ReadyState>> for StateMachine<LabelState> {
    fn from(machine: StateMachine<ReadyState>) -> StateMachine<LabelState> {
        StateMachine {
            state: LabelState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl StateMachine<ReadyState> {
    pub fn new(tokens: Vec<Token>) -> Self {
        StateMachine {
            state: ReadyState,
            tokens: tokens,
            forms: Vec::new(),
        }
    }
    pub fn handler(mut self) -> Result<Option<Form>, ()> {
        match self.tokens.pop() {
            Some(Token::Opcode(opcode)) => {
                self.forms = reducer(opcode.get_forms(), &opcode, self.tokens.len() + 1);
                // Ensure there exist a valid form.
                if self.forms.is_empty() {
                    return Err(());
                }
                // If LDR or STR instruction, ensure that expression ends with a close bracket.
                match opcode {
                    Opcode::STR | Opcode::LDR => match self.tokens.get(0) {
                        Some(Token::Separator(Separator::CloseBrace)) => (),
                        _ => return Err(()),
                    },
                    _ => (),
                }
                return StateMachine::<OpcodeState>::from(self).handler();
            }
            Some(Token::Label(_)) => {
                return StateMachine::<LabelState>::from(self).handler();
            }
            _ => return Err(()),
        }
    }
}

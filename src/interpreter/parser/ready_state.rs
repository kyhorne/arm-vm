use super::super::lexer::Token;
pub use super::super::parser::StateMachine;
use super::super::parser::{OpcodeState, ReadyState};
use super::super::super::util::{Form, reducer};

impl From<StateMachine<ReadyState>> for StateMachine<OpcodeState> {
    fn from(machine: StateMachine<ReadyState>) -> StateMachine<OpcodeState> {
        StateMachine {
            state:  OpcodeState,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl StateMachine<ReadyState> {
	pub fn new(tokens: Vec<Token>) -> Self {
		StateMachine {
			state:  ReadyState,
			tokens: tokens,
			forms:  Vec::new()
		}
	}
	pub fn handler(mut self) -> Result<Form, ()>  {
		if let Some(Token::Opcode(opcode)) = self.tokens.pop() {
			self.forms = reducer(opcode.get_forms(), opcode, self.tokens.len() + 1);
			if self.forms.is_empty() {
				return Err(())
			}
			return StateMachine::<OpcodeState>::from(self).handler();
		}
		return Err(())
	}
}

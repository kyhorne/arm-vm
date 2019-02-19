use super::super::lexer::Token;
pub use super::super::parser::StateMachine;
use super::super::parser::{OpcodeState, ReadyState};

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
	pub fn handler(mut self) -> Result<(), ()>  {
		if let Some(Token::Opcode(opcode)) = self.tokens.pop() {
			let forms = opcode.get_forms();
			let mut expression_lengths = Vec::new();
			for form in &forms {
				expression_lengths.push(form.get_expr_length(&opcode));
			}
			if expression_lengths.contains(&(self.tokens.len() + 1)) {
				self.forms = forms;
				return StateMachine::<OpcodeState>::from(self).handler();
			}
		}
		return Err(())
	}
}

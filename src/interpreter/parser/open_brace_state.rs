use super::super::lexer::Token;
use super::super::parser::{StateMachine, OpenBrace, RegisterState};

impl From<StateMachine<OpenBrace>> for StateMachine<RegisterState> {
    fn from(machine: StateMachine<OpenBrace>) -> StateMachine<RegisterState> {
        StateMachine {
            state:  RegisterState,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl StateMachine<OpenBrace> {
	pub fn handler(mut self) -> Result<(), ()> {
		if let Some(Token::Register(_)) = self.tokens.pop() {
			return StateMachine::<RegisterState>::from(self).handler();
		}
		return Err(())
	}
}

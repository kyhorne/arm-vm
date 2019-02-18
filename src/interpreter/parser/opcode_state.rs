use super::super::lexer::Token;
use super::super::parser::{OpcodeState, RegisterState, StateMachine};

impl From<StateMachine<OpcodeState>> for StateMachine<RegisterState> {
    fn from(machine: StateMachine<OpcodeState>) -> StateMachine<RegisterState> {
        StateMachine {
            state:  RegisterState,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl StateMachine<OpcodeState> {
	pub fn handler(mut self) -> Result<(), ()> {
		if let Some(Token::Register(_)) = self.tokens.pop() {
			return StateMachine::<RegisterState>::from(self).handler();
		}
		return Err(())
	}
}

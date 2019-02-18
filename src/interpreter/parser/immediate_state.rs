use super::super::lexer::{Seperator, Token};
use super::super::parser::{CloseBrace, ImmediateState, StateMachine};
use super::super::super::util::Form;

impl From<StateMachine<ImmediateState>> for StateMachine<CloseBrace> {
    fn from(machine: StateMachine<ImmediateState>) -> StateMachine<CloseBrace> {
        StateMachine {
            state:  CloseBrace,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl StateMachine<ImmediateState> {
	pub fn handler(mut self) -> Result<(), ()> {
		match self.tokens.pop() {
			Some(Token::Seperator(seperator)) => {
				match seperator {
					Seperator::CloseBrace =>
						return StateMachine::<CloseBrace>::from(self).handler(),
					_ => ()
				}
			}
			None => {
				if self.forms.contains(&Form::Four) || self.forms.contains(&Form::Five) {
					return Ok(())
				}
				return Err(())
			}
			_ => ()
		}
		return Err(())
	}
}

use super::super::lexer::{Token, Seperator};
use super::super::parser::{StateMachine, ImmediateState, CloseBrace};
use super::super::form::Form;

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
					Seperator::CloseBrace => {
						return StateMachine::<CloseBrace>::from(self).handler();
					}
					_ => ()
				}
			}
			None => {
				if self.forms.contains(&Form::Four) || self.forms.contains(&Form::Five) {
					return Ok(())
				}
			}
			_ => ()
		}
		return Err(())
	}
}

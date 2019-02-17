use super::super::parser::{StateMachine, CloseBrace};
use super::super::form::Form;

impl StateMachine<CloseBrace> {
	pub fn handler(mut self) -> Result<(), ()> {
		if let None = self.tokens.pop() {
			if self.forms.contains(&Form::One) || self.forms.contains(&Form::Two) || self.forms.contains(&Form::Four) {
				return Ok(())
			}
		}
		return Err(())
	}
}

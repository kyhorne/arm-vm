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
	pub fn handler(mut self) -> Result<Form, ()> {
		match self.tokens.pop() {
			Some(Token::Seperator(seperator)) => {
				match seperator {
					Seperator::CloseBrace => {
						// Cannot be form one.
						if self.forms.contains(&Form::One) {
							self.forms = vec![Form::Four];
						}
						// Cannot be form two.
						if self.forms.contains(&Form::Two) {
							self.forms = vec![Form::Five];
						}
						return StateMachine::<CloseBrace>::from(self).handler()
					}
					_ => ()
				}
			}
			None => {
				if self.forms.contains(&Form::Four) {
					return Ok(Form::Four)
				}
				if self.forms.contains(&Form::Five) {
					return Ok(Form::Five)
				}
				return Err(())
			}
			_ => ()
		}
		return Err(())
	}
}

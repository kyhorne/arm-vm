use super::super::lexer::{Token, Seperator};
use super::super::parser::{StateMachine, RegisterState, CommaState, CloseBrace};
use super::super::form::Form;

impl From<StateMachine<RegisterState>> for StateMachine<CommaState> {
    fn from(machine: StateMachine<RegisterState>) -> StateMachine<CommaState> {
        StateMachine {
            state:  CommaState,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl From<StateMachine<RegisterState>> for StateMachine<CloseBrace> {
    fn from(machine: StateMachine<RegisterState>) -> StateMachine<CloseBrace> {
        StateMachine {
            state:  CloseBrace,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl StateMachine<RegisterState> {
	pub fn handler(mut self) -> Result<(), ()>  {
		let token = self.tokens.pop();
		match token {
			Some(Token::Seperator(seperator)) => {
				match seperator {
					Seperator::Comma => {
						return StateMachine::<CommaState>::from(self).handler()
					}
					Seperator::CloseBrace => {
						return StateMachine::<CloseBrace>::from(self).handler()
					}
					_ => ()
				}
			},
			None => {
				if self.forms.contains(&Form::One) || self.forms.contains(&Form::Two) {
					return Ok(())
				}
			}
			_ => ()
		}
		return Err(())
	}
}

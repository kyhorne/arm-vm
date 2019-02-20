use super::super::lexer::{Token, Seperator};
use super::super::super::util::Form;

use super::super::parser::{
	CommaState,
	ImmediateState,
	OpenBrace,
	RegisterState,
	StateMachine
};

impl From<StateMachine<CommaState>> for StateMachine<ImmediateState> {
    fn from(machine: StateMachine<CommaState>) -> StateMachine<ImmediateState> {
        StateMachine {
            state:  ImmediateState,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl From<StateMachine<CommaState>> for StateMachine<RegisterState> {
    fn from(machine: StateMachine<CommaState>) -> StateMachine<RegisterState> {
        StateMachine {
            state:  RegisterState,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl From<StateMachine<CommaState>> for StateMachine<OpenBrace> {
    fn from(machine: StateMachine<CommaState>) -> StateMachine<OpenBrace> {
        StateMachine {
            state:  OpenBrace,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl StateMachine<CommaState> {
	pub fn handler(mut self) -> Result<Form, ()> {
		match self.tokens.pop() {
			Some(Token::Register(_)) => {
				return StateMachine::<RegisterState>::from(self).handler()
			}
			Some(Token::Literal(immed)) => {
				let immed = immed.get_value();
				if (self.forms.contains(&Form::Four) && 0xFFFF < immed)
					|| (self.forms.contains(&Form::Five) && 0xFFFFF < immed) {
						println!("dsada {:?}", 0xFFFFF < immed);
						return Err(())
				}
				return StateMachine::<ImmediateState>::from(self).handler()
			}
			Some(Token::Seperator(seperator)) => {
				match seperator {
					Seperator::OpenBrace => {
						return StateMachine::<OpenBrace>::from(self).handler()
					}
					_ => ()
				}
			}
			_ => ()
		}
		return Err(())
	}
}

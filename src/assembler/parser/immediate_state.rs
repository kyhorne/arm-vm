use super::super::super::util::Form;
use super::super::lexer::{Separator, Token};
use super::super::parser::{CloseBraceState, ImmediateState, StateMachine};

impl From<StateMachine<ImmediateState>> for StateMachine<CloseBraceState> {
    fn from(machine: StateMachine<ImmediateState>) -> StateMachine<CloseBraceState> {
        StateMachine {
            state: CloseBraceState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl StateMachine<ImmediateState> {
    pub fn handler(mut self) -> Result<Option<Form>, ()> {
        match self.tokens.pop() {
            Some(Token::Separator(separator)) => {
                match separator {
                    Separator::CloseBrace => {
                        // Cannot be form one.
                        if self.forms.contains(&Form::One) {
                            self.forms = vec![Form::Four];
                        }
                        // Cannot be form two.
                        if self.forms.contains(&Form::Two) {
                            self.forms = vec![Form::Five];
                        }
                        return StateMachine::<CloseBraceState>::from(self).handler();
                    }
                    _ => (),
                }
            }
            None => {
                if self.forms.contains(&Form::Four) {
                    return Ok(Some(Form::Four));
                }
                if self.forms.contains(&Form::Five) {
                    return Ok(Some(Form::Five));
                }
                return Err(());
            }
            _ => (),
        }
        return Err(());
    }
}

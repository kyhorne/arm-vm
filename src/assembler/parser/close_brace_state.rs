use super::super::super::util::Form;
use super::super::parser::{CloseBraceState, StateMachine};

impl StateMachine<CloseBraceState> {
    pub fn handler(mut self) -> Result<Option<Form>, ()> {
        if let None = self.tokens.pop() {
            if self.forms.contains(&Form::One) {
                return Ok(Some(Form::One));
            }
            if self.forms.contains(&Form::Two) {
                return Ok(Some(Form::Two));
            }
            if self.forms.contains(&Form::Four) {
                return Ok(Some(Form::Four));
            }
            if self.forms.contains(&Form::Five) {
                return Ok(Some(Form::Five));
            }
        }
        return Err(());
    }
}

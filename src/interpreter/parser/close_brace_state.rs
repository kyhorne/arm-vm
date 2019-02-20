use super::super::super::util::Form;
use super::super::parser::{CloseBrace, StateMachine};

impl StateMachine<CloseBrace> {
    pub fn handler(mut self) -> Result<Form, ()> {
        if let None = self.tokens.pop() {
            if self.forms.contains(&Form::One) {
                return Ok(Form::One);
            }
            if self.forms.contains(&Form::Two) {
                return Ok(Form::Two);
            }
            if self.forms.contains(&Form::Four) {
                return Ok(Form::Four);
            }
            if self.forms.contains(&Form::Five) {
                return Ok(Form::Five);
            }
        }
        return Err(());
    }
}

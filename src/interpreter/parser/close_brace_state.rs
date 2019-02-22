use super::super::super::util::Form;
use super::super::lexer::Label;
use super::super::parser::{CloseBrace, StateMachine};

impl StateMachine<CloseBrace> {
    pub fn handler(mut self) -> Result<(Option<Form>, Option<Label>), ()> {
        if let None = self.tokens.pop() {
            if self.forms.contains(&Form::One) {
                return Ok((Some(Form::One), self.label));
            }
            if self.forms.contains(&Form::Two) {
                return Ok((Some(Form::Two), self.label));
            }
            if self.forms.contains(&Form::Four) {
                return Ok((Some(Form::Four), self.label));
            }
            if self.forms.contains(&Form::Five) {
                return Ok((Some(Form::Five), self.label));
            }
        }
        return Err(());
    }
}

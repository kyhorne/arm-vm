use super::super::super::util::Form;
use super::super::lexer::Label;
use super::super::parser::{CloseBrace, StateMachine};

impl StateMachine<CloseBrace> {
    pub fn handler(mut self) -> Result<(Form, Vec<Label>), ()> {
        if let None = self.tokens.pop() {
            if self.forms.contains(&Form::One) {
                return Ok((Form::One, self.labels));
            }
            if self.forms.contains(&Form::Two) {
                return Ok((Form::Two, self.labels));
            }
            if self.forms.contains(&Form::Four) {
                return Ok((Form::Four, self.labels));
            }
            if self.forms.contains(&Form::Five) {
                return Ok((Form::Five, self.labels));
            }
        }
        return Err(());
    }
}

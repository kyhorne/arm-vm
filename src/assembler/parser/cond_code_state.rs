use super::super::super::util::Form;
use super::super::lexer::Token;
use super::super::parser::{ConditionCodeState, StateMachine};

impl StateMachine<ConditionCodeState> {
    pub fn handler(mut self) -> Result<Option<Form>, ()> {
        match self.tokens.pop() {
            Some(Token::Label(_)) => {
                if self.forms.contains(&Form::Six) {
                    return Ok(Some(Form::Six));
                }
            }
            _ => (),
        }
        return Err(());
    }
}

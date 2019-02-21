use super::super::super::util::Form;
use super::super::lexer::{Label, Seperator, Token};
use super::super::parser::{CloseBrace, ImmediateState, StateMachine};

impl From<StateMachine<ImmediateState>> for StateMachine<CloseBrace> {
    fn from(machine: StateMachine<ImmediateState>) -> StateMachine<CloseBrace> {
        StateMachine {
            state: CloseBrace,
            tokens: machine.tokens,
            forms: machine.forms,
            labels: machine.labels,
        }
    }
}

impl StateMachine<ImmediateState> {
    pub fn handler(mut self) -> Result<(Option<Form>, Vec<Label>), ()> {
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
                        return StateMachine::<CloseBrace>::from(self).handler();
                    }
                    _ => (),
                }
            }
            None => {
                if self.forms.contains(&Form::Four) {
                    return Ok((Some(Form::Four), self.labels));
                }
                if self.forms.contains(&Form::Five) {
                    return Ok((Some(Form::Five), self.labels));
                }
                return Err(());
            }
            _ => (),
        }
        return Err(());
    }
}

use super::super::super::util::Form;
use super::super::lexer::{Separator, Token};
use super::super::parser::{CloseBraceState, CommaState, RegisterState, StateMachine};

impl From<StateMachine<RegisterState>> for StateMachine<CloseBraceState> {
    fn from(machine: StateMachine<RegisterState>) -> StateMachine<CloseBraceState> {
        StateMachine {
            state: CloseBraceState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl From<StateMachine<RegisterState>> for StateMachine<CommaState> {
    fn from(machine: StateMachine<RegisterState>) -> StateMachine<CommaState> {
        StateMachine {
            state: CommaState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl StateMachine<RegisterState> {
    pub fn handler(mut self) -> Result<Option<Form>, ()> {
        let token = self.tokens.pop();
        match token {
            Some(Token::Separator(separator)) => match separator {
                Separator::Comma => return StateMachine::<CommaState>::from(self).handler(),
                Separator::CloseBrace => {
                    return StateMachine::<CloseBraceState>::from(self).handler();
                }
                _ => (),
            },
            None => {
                if self.forms.contains(&Form::One) {
                    return Ok(Some(Form::One));
                }
                if self.forms.contains(&Form::Two) {
                    return Ok(Some(Form::Two));
                }
            }
            _ => (),
        }
        return Err(());
    }
}

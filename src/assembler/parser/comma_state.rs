use super::super::super::util::Form;
use super::super::lexer::{Separator, Token};

use super::super::parser::{
    CommaState, ImmediateState, OpenBraceState, RegisterState, StateMachine,
};

impl From<StateMachine<CommaState>> for StateMachine<ImmediateState> {
    fn from(machine: StateMachine<CommaState>) -> StateMachine<ImmediateState> {
        StateMachine {
            state: ImmediateState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl From<StateMachine<CommaState>> for StateMachine<RegisterState> {
    fn from(machine: StateMachine<CommaState>) -> StateMachine<RegisterState> {
        StateMachine {
            state: RegisterState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl From<StateMachine<CommaState>> for StateMachine<OpenBraceState> {
    fn from(machine: StateMachine<CommaState>) -> StateMachine<OpenBraceState> {
        StateMachine {
            state: OpenBraceState,
            tokens: machine.tokens,
            forms: machine.forms,
        }
    }
}

impl StateMachine<CommaState> {
    pub fn handler(mut self) -> Result<Option<Form>, ()> {
        match self.tokens.pop() {
            Some(Token::Register(_)) => return StateMachine::<RegisterState>::from(self).handler(),
            Some(Token::Literal(immed)) => {
                let immed = immed.get_value();
                if (self.forms.contains(&Form::Four) && 0xFFFF < immed)
                    || (self.forms.contains(&Form::Five) && 0xFFFFF < immed)
                {
                    return Err(());
                }
                return StateMachine::<ImmediateState>::from(self).handler();
            }
            Some(Token::Separator(separator)) => match separator {
                Separator::OpenBrace => return StateMachine::<OpenBraceState>::from(self).handler(),
                _ => (),
            },
            _ => (),
        }
        return Err(());
    }
}

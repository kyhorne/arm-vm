use super::super::lexer::Token;
use super::super::super::util::{Form, Opcode};
pub use super::super::parser::StateMachine;
use super::super::parser::{OpcodeState, ReadyState};

impl From<StateMachine<ReadyState>> for StateMachine<OpcodeState> {
    fn from(machine: StateMachine<ReadyState>) -> StateMachine<OpcodeState> {
        StateMachine {
            state:  OpcodeState,
			tokens: machine.tokens,
			forms:  machine.forms
        }
    }
}

impl Opcode {
	fn get_forms(&self) -> (Vec<Form>, Vec<usize>) {
		match *self {
			Opcode::ADD | Opcode::AND | Opcode::EOR | Opcode::MUL | Opcode::ORR | Opcode::SUB => (vec![Form::One, Form::Four], vec![6]),
			Opcode::MOV | Opcode::MVN => (vec![Form::Two, Form::Five], vec![4]),
			Opcode::STR | Opcode::LDR => (vec![Form::One, Form::Two, Form::Four], vec![6, 8])
		}
	}
}

impl StateMachine<ReadyState> {
	pub fn new(tokens: Vec<Token>) -> Self {
		StateMachine {
			state:  ReadyState,
			tokens: tokens,
			forms:  Vec::new()
		}
	}
	pub fn handler(mut self) -> Result<(), ()>  {
		if let Some(Token::Opcode(opcode)) = self.tokens.pop() {
			let (forms, expression_lengths) = opcode.get_forms();
			if expression_lengths.contains(&(self.tokens.len() + 1)) {
				self.forms = forms;
				return StateMachine::<OpcodeState>::from(self).handler();
			}
		}
		return Err(())
	}
}

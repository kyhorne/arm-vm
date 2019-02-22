mod close_brace_state;
mod comma_state;
mod immediate_state;
mod label_state;
mod opcode_state;
mod open_brace_state;
mod ready_state;
mod register_state;

use super::super::util::Form;
use super::lexer::{Label, Token};

/// The syntax is analyzed using a finite state machine.
struct CloseBrace;
struct CommaState;
struct ImmediateState;
struct OpcodeState;
struct OpenBrace;
struct ReadyState;
struct RegisterState;
struct LabelState;

pub struct StateMachine<S> {
    // The current state of the state machine.
    pub state: S,
    // The parse tree.
    pub tokens: Vec<Token>,
    // Possible forms this expression may satisfy.
    pub forms: Vec<Form>,
    pub label: Option<Label>,
}

/// Run the state machine.
pub fn run(tokens: &mut Vec<Token>) -> Result<(Option<Form>, Option<Label>), ()> {
    tokens.reverse();
    return ready_state::StateMachine::new(tokens.to_vec()).handler();
}

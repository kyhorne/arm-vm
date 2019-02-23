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
struct CloseBraceState;
struct CommaState;
struct ImmediateState;
struct OpcodeState;
struct OpenBraceState;
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

#[cfg(test)]
mod tests_basic_form_one {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn testis_ok() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        if let Ok((Some(form), _)) = run(&mut tokens) {
            assert_eq!(form, One);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_without_label() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        if let Ok((_, None)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_with_label() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_ldr_str_form_one {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_is_ok() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(CloseBrace),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(CloseBrace),
        ];
        if let Ok((Some(form), _)) = run(&mut tokens) {
            assert_eq!(form, One);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_without_label() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(CloseBrace),
        ];
        if let Ok((_, None)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_with_label() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(CloseBrace),
        ];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_basic_form_two {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_is_ok() {
        let mut tokens = vec![Opcode(MOV), Register(R0), Separator(Comma), Register(R0)];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![Opcode(MOV), Register(R0), Separator(Comma), Register(R0)];
        if let Ok((Some(form), _)) = run(&mut tokens) {
            assert_eq!(form, Two);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_without_label() {
        let mut tokens = vec![Opcode(MOV), Register(R0), Separator(Comma), Register(R0)];
        if let Ok((_, None)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_with_label() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Opcode(MOV),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_ldr_str_form_two {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_is_ok() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(CloseBrace),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(CloseBrace),
        ];
        if let Ok((Some(form), _)) = run(&mut tokens) {
            assert_eq!(form, Two);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_without_label() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(CloseBrace),
        ];
        if let Ok((_, None)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_with_label() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(CloseBrace),
        ];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_basic_form_four {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Literal::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_ok_with_base_16() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_out_of_bounds_with_base_16() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1FFFF"))),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_is_ok_with_base_10() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("1234"))),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_out_of_bounds_with_base_10() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Literal(Immediate((1 + u32::pow(2, 16)).to_string())),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        if let Ok((Some(form), _)) = run(&mut tokens) {
            assert_eq!(form, Four);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_without_label() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        if let Ok((_, None)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_with_label() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_ldr_str_form_four {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Literal::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_is_ok() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        if let Ok((Some(form), _)) = run(&mut tokens) {
            assert_eq!(form, Four);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_without_label() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        if let Ok((_, None)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_with_label() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_basic_form_five {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Literal::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_is_ok_with_base_16() {
        let mut tokens = vec![
            Opcode(MOV),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_out_of_bounds_with_base_16() {
        let mut tokens = vec![
            Opcode(MOV),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1FFFFF"))),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_is_ok_with_base_10() {
        let mut tokens = vec![
            Opcode(MOV),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("1234"))),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_out_of_bounds_with_base_10() {
        let mut tokens = vec![
            Opcode(MOV),
            Register(R0),
            Separator(Comma),
            Literal(Immediate((1 + u32::pow(2, 20)).to_string())),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![
            Opcode(MOV),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        if let Ok((Some(form), _)) = run(&mut tokens) {
            assert_eq!(form, Five);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_without_label() {
        let mut tokens = vec![
            Opcode(MOV),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        if let Ok((_, None)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_with_label() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Opcode(MOV),
            Register(R0),
            Separator(Comma),
            Literal(Immediate(String::from("0x1234"))),
        ];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_ldr_str_form_five {

    use super::super::super::util::{Form::*, Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Literal::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_is_ok() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        if let Ok((Some(form), _)) = run(&mut tokens) {
            assert_eq!(form, Five);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_without_label() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        if let Ok((_, None)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_with_label() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Literal(Immediate(String::from("0x1234"))),
            Separator(CloseBrace),
        ];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_form_six {

    use super::*;

    #[test]
    fn test_is_ok() {
        let mut tokens = vec![Token::Label(Label::Name(String::from("foo")))];
        assert!(run(&mut tokens).is_ok());
    }

    #[test]
    fn test_form() {
        let mut tokens = vec![Token::Label(Label::Name(String::from("foo")))];
        if let Ok((None, _)) = run(&mut tokens) {
            assert!(true);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_label() {
        let mut tokens = vec![Token::Label(Label::Name(String::from("foo")))];
        if let Ok((_, Some(label))) = run(&mut tokens) {
            assert_eq!(label, Label::Name(String::from("foo")));
        } else {
            assert!(false);
        }
    }

}

#[cfg(test)]
mod tests_incorrect_behaviour {

    use super::super::super::util::{Opcode::*, Register::*};
    use super::super::lexer::{Label::*, Separator::*, Token::*};
    use super::*;

    #[test]
    fn test_mult_labels() {
        let mut tokens = vec![
            Label(Name(String::from("foo"))),
            Label(Name(String::from("bar"))),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_mult_opcodes() {
        let mut tokens = vec![
            Opcode(ADD),
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_mult_commas() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_basic_expr_with_braces() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Separator(OpenBrace),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(CloseBrace),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_ldr_str_with_no_open_brace() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(CloseBrace),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_ldr_str_with_no_close_brace() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(OpenBrace),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_ldr_str_with_no_braces() {
        let mut tokens = vec![
            Opcode(STR),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
        ];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_no_commas() {
        let mut tokens = vec![Opcode(ADD), Register(R0), Register(R0), Register(R0)];
        assert!(run(&mut tokens).is_err());
    }

    #[test]
    fn test_label_at_end_of_expr() {
        let mut tokens = vec![
            Opcode(ADD),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Separator(Comma),
            Register(R0),
            Label(Name(String::from("foo"))),
        ];
        assert!(run(&mut tokens).is_err());
    }
}

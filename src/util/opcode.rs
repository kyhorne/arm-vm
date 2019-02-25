use num_derive::FromPrimitive;
use std::collections::HashMap;
use std::slice::Iter;

use self::Form::*;
use self::Opcode::*;

#[derive(Clone, EnumString, Eq, Debug, PartialEq, FromPrimitive)]
pub enum Opcode {
    #[strum(serialize = "ADD", serialize = "add")]
    ADD,
    #[strum(serialize = "AND", serialize = "and")]
    AND,
    #[strum(serialize = "EOR", serialize = "eor")]
    EOR,
    #[strum(serialize = "MUL", serialize = "mul")]
    MUL,
    #[strum(serialize = "ORR", serialize = "orr")]
    ORR,
    #[strum(serialize = "SUB", serialize = "sub")]
    SUB,
    #[strum(serialize = "MOV", serialize = "mov")]
    MOV,
    #[strum(serialize = "MVN", serialize = "mvn")]
    MVN,
    #[strum(serialize = "STR", serialize = "str")]
    STR,
    #[strum(serialize = "LDR", serialize = "ldr")]
    LDR,
    #[strum(serialize = "CMP", serialize = "cmp")]
    CMP,
    #[strum(serialize = "B", serialize = "b")]
    B,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Form {
    /// A form one instruction has the following encoding scheme:
    /// OP DR, RX, RY ; DR <- [RX] OP [RY]
    ///
    /// # Examples:
    /// ```
    ///	ADD R5, R1, R10
    /// XOR R4, R4, R4
    /// ```
    One,
    /// A form one instruction has the following encoding scheme:
    /// OP DR, RX ; DR <- OP([RX])
    ///
    /// # Examples:
    /// ```
    /// MOV R5, R1
    /// MVN R10, R11
    /// ```
    Two,
    /// A form four instruction has the following encoding scheme:
    /// OP DR, RX, #immed16 ; DR <- [RX] OP #immed16
    ///
    /// # Examples:
    /// ```
    ///	ADD R5, R1, #10
    ///	AND R4, R4, #0x1
    /// ```
    Four,
    /// A form five instruction has the following encoding scheme:
    /// OP DR, #immed20 ; DR <- OP(#immed20)
    ///
    /// # Examples:
    /// ```
    ///	MOV R5, #0xF1234
    /// MVN R9, #0x0
    /// ```
    Five,
    /// TODO: Explanation.
    Six,
}

impl Opcode {
    fn iter() -> Iter<'static, Opcode> {
        static OPCODE: [Opcode; 12] = [ADD, SUB, MOV, AND, ORR, EOR, MVN, MUL, LDR, STR, CMP, B];
        OPCODE.into_iter()
    }
    /// Get the bytecode and form associated from a given opcode.
    pub fn get_bytecode(&self) -> HashMap<Form, u32> {
        match *self {
            ADD => [(One, 0x1), (Four, 0x21)].iter().cloned().collect(),
            SUB => [(One, 0x2), (Four, 0x22)].iter().cloned().collect(),
            MOV => [(Two, 0x3), (Five, 0x23)].iter().cloned().collect(),
            AND => [(One, 0x4), (Four, 0x24)].iter().cloned().collect(),
            ORR => [(One, 0x5), (Four, 0x25)].iter().cloned().collect(),
            EOR => [(One, 0x6), (Four, 0x26)].iter().cloned().collect(),
            MVN => [(Two, 0x7), (Five, 0x27)].iter().cloned().collect(),
            MUL => [(One, 0x8), (Four, 0x28)].iter().cloned().collect(),
            LDR => [(Two, 0x30), (Four, 0x31), (One, 0x32), (Five, 0x33)]
                .iter()
                .cloned()
                .collect(),
            STR => [(Two, 0x34), (Four, 0x35), (One, 0x36), (Five, 0x37)]
                .iter()
                .cloned()
                .collect(),
            CMP => [(Two, 0x47), (Five, 0x57)].iter().cloned().collect(),
            B => [(Six, 0x80)].iter().cloned().collect(),
        }
    }
    /// Get the forms associated with a given opcode.
    pub fn get_forms(&self) -> Vec<Form> {
        self.get_bytecode().keys().map(|key| key.clone()).collect()
    }
    /// Get the opcode from a given bytecode.
    pub fn get_opcode(bytecode: u32) -> Result<(Form, Opcode), ()> {
        for opcode in Opcode::iter() {
            for (key, value) in opcode.get_bytecode().iter() {
                if value.clone() == bytecode {
                    return Ok((*key, opcode.clone()));
                }
            }
        }
        return Err(());
    }
}

impl Form {
    /// Get the expression length associated with a given form and opcode.
    fn get_expr_length(&self, opcode: &Opcode) -> usize {
        let mut delta = 0;
        match *opcode {
            Opcode::STR | Opcode::LDR => delta = 2,
            _ => (),
        }
        match *self {
            One | Four => 6 + delta,
            Two | Five => 4 + delta,
            Six => 3,
        }
    }
}

/// Reduce the list of forms given an opcode and expression length.
pub fn reducer(forms: Vec<Form>, opcode: &Opcode, len: usize) -> Vec<Form> {
    let mut reducer = Vec::new();
    for form in forms {
        if len == form.get_expr_length(opcode) {
            reducer.push(form);
        }
    }
    reducer
}

#[cfg(test)]
mod tests_opcode {

    use super::*;

    #[test]
    fn test_get_bytecode() {
        assert_eq!(
            ADD.get_bytecode(),
            [(One, 0x1), (Four, 0x21)].iter().cloned().collect()
        );
    }

    #[test]
    fn test_get_form() {
        assert!(ADD.get_forms().contains(&One) && ADD.get_forms().contains(&Four));
    }

    #[test]
    fn test_get_opcode_is_ok() {
        assert!(Opcode::get_opcode(0x01).is_ok());
    }

    #[test]
    fn test_get_opcode_form() {
        if let Ok((form, _)) = Opcode::get_opcode(0x01) {
            assert_eq!(form, One);
        }
    }

    #[test]
    fn test_get_opcode_opcode() {
        if let Ok((_, opcode)) = Opcode::get_opcode(0x01) {
            assert_eq!(opcode, ADD);
        }
    }

}

#[cfg(test)]
mod tests_form {

    use super::*;

    #[test]
    fn test_basic_form_one() {
        assert_eq!(One.get_expr_length(&ADD), 6);
    }

    #[test]
    fn test_ldr_str_form_one() {
        assert_eq!(One.get_expr_length(&STR), 8);
    }

    #[test]
    fn test_basic_form_two() {
        assert_eq!(Two.get_expr_length(&MOV), 4);
    }

    #[test]
    fn test_ldr_str_form_two() {
        assert_eq!(Two.get_expr_length(&STR), 6);
    }

    #[test]
    fn test_basic_form_four() {
        assert_eq!(Four.get_expr_length(&ADD), 6);
    }

    #[test]
    fn test_ldr_str_form_four() {
        assert_eq!(Four.get_expr_length(&STR), 8);
    }

    #[test]
    fn test_basic_form_five() {
        assert_eq!(Five.get_expr_length(&MOV), 4);
    }

    #[test]
    fn test_ldr_str_form_five() {
        assert_eq!(Five.get_expr_length(&STR), 6);
    }

    #[test]
    fn test_form_six() {
        assert_eq!(Six.get_expr_length(&B), 3);
    }

}

#[cfg(test)]
mod tests_reducer {

    use super::*;

    #[test]
    fn test_basic_form_one_and_four() {
        let forms = reducer(vec![One, Two, Four, Five], &ADD, 6);
        assert!(forms.contains(&One) && forms.contains(&Four));
    }

    #[test]
    fn test_ldr_str_form_one_and_four() {
        let forms = reducer(vec![One, Four], &STR, 8);
        assert!(forms.contains(&One) && forms.contains(&Four));
    }

    #[test]
    fn test_basic_form_two_and_five() {
        let forms = reducer(vec![Two, Five], &MOV, 4);
        assert!(forms.contains(&Two) && forms.contains(&Five));
    }

    #[test]
    fn test_ldr_str_form_two_and_five() {
        let forms = reducer(vec![One, Two, Four, Five], &STR, 6);
        assert!(forms.contains(&Two) && forms.contains(&Five));
    }

}

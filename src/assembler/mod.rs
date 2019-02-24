mod lexer;
mod parser;

pub use super::assembler::lexer::Label;
use super::assembler::lexer::{lexer, Token, Token::*};
use super::util::{EncoderDecoder, Form, Instruction, Literal::*};

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct LabelRegistry {
    // The index of the current instruction being assembled which is used as a pointer for variable
    // references.
    instr_ptr: Instruction,
    /// Defines the label as a location of an instruction in a program.
    declaration: HashMap<Label, Instruction>,
    /// Label is interpreted by the assembler as a refernce to a target address.
    reference: HashMap<Instruction, Label>,
}

impl LabelRegistry {
    pub fn new() -> LabelRegistry {
        LabelRegistry {
            instr_ptr: 0,
            declaration: HashMap::new(),
            reference: HashMap::new(),
        }
    }
    pub fn register_variable_reference(&mut self, label: Label) {
        self.reference.insert(self.instr_ptr, label);
    }
    pub fn register_variable_declaration(&mut self, label: Label) {
        self.declaration.insert(label, self.instr_ptr);
    }
    pub fn get_reference(&mut self) -> &Instruction {
        self.declaration
            .get(self.reference.get(&self.instr_ptr).unwrap())
            .unwrap()
    }
    pub fn incr_instr_ptr(&mut self) {
        self.instr_ptr += 1;
    }
    pub fn reset_instr_ptr(&mut self) {
        self.instr_ptr = 0;
    }
}

#[derive(Clone)]
struct Expression {
    tokens: Vec<Token>,
    form: Form,
}

pub struct Assembler {
    registry: LabelRegistry,
    program: Vec<Expression>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            registry: LabelRegistry::new(),
            program: Vec::new(),
        }
    }
    pub fn read_file(&mut self) -> Vec<Instruction> {
        let mut program = Vec::new();
        let file = File::open("assembly/pgrm.asm").unwrap();
        for buf in BufReader::new(file).lines() {
            let expression = buf.unwrap();
            // Convert the expression into meaningful lexemes.
            let mut tokens = lexer(expression);
            // Ensure expression is syntactically correct.
            if let Ok(form) = parser::run(&mut tokens) {
                self.optimized_tokens(&mut tokens, form);
            }
        }
        self.registry.reset_instr_ptr();
        // Convert the program to bytecode.
        for expr in self.program.clone() {
            program.push(self.get_bytecode(expr));
            self.registry.incr_instr_ptr();
        }
        program
    }
    /// Remove tokens that are irrelevant to byte code encoding and register all labels in label
    /// registry if it is not a Form Six expression.
    fn optimized_tokens(&mut self, tokens: &mut Vec<Token>, form: Option<Form>) {
        let mut optimized_tokens = Vec::new();
        for token in tokens {
            match token {
                Label(label) => {
                    match form {
                        Some(Form::Six) => self.registry.register_variable_reference(label.clone()),
                        _ => self.registry.register_variable_declaration(label.clone()),
                    }
                    optimized_tokens.push(token.clone());
                }
                Opcode(_) | Register(_) | Literal(_) => optimized_tokens.push(token.clone()),
                _ => (),
            }
        }
        optimized_tokens.reverse();
        if let Some(form) = form {
            self.program.push(Expression {
                tokens: optimized_tokens,
                form: form,
            });
        }
        self.registry.incr_instr_ptr();
    }
    /// Get the bytecode encoding of the expression.
    fn get_bytecode(&mut self, expr: Expression) -> u32 {
        #[derive(Clone)]
        enum RegisterType {
            Dr = 0,
            Rx,
            Ry,
        }
        let mut next_encoded_register = RegisterType::Dr;
        let mut encoder = EncoderDecoder::new(None);
        for token in &expr.tokens {
            match token {
                Label(_) => match expr.form {
                    Form::Six => {
                        let immed = self.registry.get_reference();
                        encoder.set_immed20(Immediate(immed.to_string()));
                    }
                    _ => (),
                },
                Opcode(opcode) => {
                    if opcode.is_bcc() {
                        encoder.set_bcc(expr.form, opcode.clone());
                    } else {
                        encoder.set_opcode(expr.form, opcode.clone());
                    }
                }
                Register(register) => match next_encoded_register.clone() {
                    RegisterType::Dr => {
                        encoder.set_dr(register.clone());
                        next_encoded_register = RegisterType::Rx;
                    }
                    RegisterType::Rx => {
                        encoder.set_rx(register.clone());
                        next_encoded_register = RegisterType::Ry;
                    }
                    RegisterType::Ry => encoder.set_ry(register.clone()),
                },
                Literal(immed) => match expr.form {
                    Form::Four => encoder.set_immed16(immed.clone()),
                    Form::Five => encoder.set_immed20(immed.clone()),
                    _ => (),
                },
                _ => (),
            }
        }
        encoder.get_instr()
    }
}

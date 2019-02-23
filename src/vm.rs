use super::assembler::{repl, Label};
use super::util::{
    get_dr_addr, get_form_and_bcc, get_form_and_opcode, get_immed16, get_immed20, get_rx_addr,
    get_ry_addr, Form, Opcode,
};
use super::util::{get_name, Register};
use std::collections::HashMap;

/// The initial value of all registers in the processor.
pub const INIT_REGISTER_VALUE: Payload = 0;

/// The number of addressable registers in this processor. A 32-bit ARM processor has 16
/// registers.
const N_REGISTERS_IN_PROCESSOR: Address = 16;

/// The number of addressable registers in main memory. A 32-bit processor has 2^32 addressable
/// memory locations.
const N_REGISTERS_IN_MAIN_MEMORY: Address = std::u32::MAX as usize;

struct Flag {
    v: bool, // oVerflow.
    z: bool, // Zero.
    n: bool, // Negative.
    c: bool, // Carry.
}

struct Variable {
    /// Defines the label as a location (address) in a program;
    declaration: HashMap<Label, u32>,
    /// Label is interpreted by the assembler as identifying the target address.
    reference: HashMap<Address, Label>,
}

/// A virtual processor has virtual registers and memory.
pub struct Processor {
    registers: Vec<u32>,
    main_memory: Vec<u32>,
    variable: Variable,
    flag: Flag,
}

pub type Payload = u32;
pub type Address = usize;

impl Processor {
    /// Instantiate a new processor.
    pub fn new() -> Processor {
        Processor {
            registers: vec![INIT_REGISTER_VALUE; N_REGISTERS_IN_PROCESSOR],
            main_memory: vec![INIT_REGISTER_VALUE; N_REGISTERS_IN_MAIN_MEMORY],
            variable: Variable {
                declaration: HashMap::new(),
                reference: HashMap::new(),
            },
            flag: Flag {
                v: false,
                z: false,
                n: false,
                c: false,
            },
        }
    }
    /// Get the contents stored in the program counter.
    fn get_pc(&self) -> Address {
        self.registers[Register::PC as Address] as Address
    }
    fn set_pc(&mut self, payload: Payload) {
        self.registers[Register::PC as Address] = payload
    }
    /// Increment the program counter by 1.
    fn incr_pc(&mut self) {
        self.registers[Register::PC as Address] += 1
    }
    /// Write data to main memory pointed to by the given address.
    pub fn write_to_mm(&mut self, address: Address, instruction: Payload) {
        self.main_memory[address] = instruction
    }
    /// Read data from the main memory pointed to by the program counter.
    fn read_from_mm(&self) -> &Payload {
        &self.main_memory[self.get_pc()]
    }
    /// Fetch and decode instruction pointed to by the program counter.
    fn fetch_and_decode(&mut self) {
        println!("----------------------------------------");
        println!("{:30}{:#010X} ", "Pc:", self.get_pc());
        // Read data from the main memory pointed to by the program counter.
        let payload = self.read_from_mm();
        println!("{:17}{:>8} = {:#010X} ", "Payload:", "MMem[[PC]]", payload);
        // Extract the opcode and form from the payload.
        if let Ok((form, opcode)) = get_form_and_opcode(*payload) {
            println!("{:24}{:?}", "Opcode:", opcode);
            // Execute the handler based on instruction form.
            match form {
                Form::One => self.form_one_handler(opcode, *payload),
                Form::Two => self.form_two_handler(opcode, *payload),
                Form::Four => self.form_four_handler(opcode, *payload),
                Form::Five => self.form_five_handler(opcode, *payload),
                _ => (),
            }
        } else if let Ok((form, opcode)) = get_form_and_bcc(*payload) {
            println!("{:24}{:?}", "Opcode:", opcode);
            // Execute the handler based on instruction form.
            match form {
                Form::Six => self.form_six_handler(opcode),
                _ => (),
            }
        }
    }
    fn form_one_handler(&mut self, opcode: Opcode, payload: Payload) {
        // Parse the destination address.
        let dr_addr = get_dr_addr(payload);
        let dr_cont = self.registers[dr_addr];
        println!("{:23}[{}] = {:#010X}", "Dr: ", get_name(dr_addr), dr_cont);
        // Define operand 1 by retrieving the content pointed to by register x.
        let rx_addr = get_rx_addr(payload);
        let op1 = self.registers[rx_addr];
        println!("{:23}[{}] = {:#010X}", "Rx:", get_name(rx_addr), op1);
        // Define operand 1 by retrieving the content pointed to by register y.
        let ry_addr = get_ry_addr(payload);
        let op2 = self.registers[ry_addr];
        println!("{:23}[{}] = {:#010X}", "Ry:", get_name(ry_addr), op2);
        // Execute instruction based on the opcode.
        match opcode {
            Opcode::ADD => self.execute(dr_addr, Box::new(move || op1 + op2)),
            Opcode::AND => self.execute(dr_addr, Box::new(move || op1 & op2)),
            Opcode::EOR => self.execute(dr_addr, Box::new(move || op1 ^ op2)),
            Opcode::MUL => self.execute(dr_addr, Box::new(move || op1 * op2)),
            Opcode::ORR => self.execute(dr_addr, Box::new(move || op1 | op2)),
            Opcode::SUB => self.execute(dr_addr, Box::new(move || op1 - op2)),
            Opcode::LDR => {
                let ptr = self.main_memory[(op1 + op2) as usize].clone();
                println!(
                    "{:10}MMem[[{}] + [{}]] = {:#010X}",
                    "Ptr:",
                    get_name(rx_addr),
                    get_name(ry_addr),
                    ptr
                );
                self.registers[dr_addr] = ptr;
                println!("{:23}[{}] = {:#010X}", "Result:", get_name(dr_addr), ptr);
            }
            Opcode::STR => {
                let payload = self.registers[dr_addr];
                self.main_memory[(op1 + op2) as usize] = payload;
                println!(
                    "{:10}MMem[[{}] + [{}]] = {:#010X}",
                    "Result:",
                    get_name(rx_addr),
                    get_name(ry_addr),
                    payload
                );
            }
            _ => (),
        }
    }

    fn form_two_handler(&mut self, opcode: Opcode, payload: Payload) {
        // Parse the destination address.
        let dr_addr = get_dr_addr(payload);
        let dr_cont = self.registers[dr_addr]; // The destination register contents.
        println!("{:23}[{}] = {:#010X}", "Dr: ", get_name(dr_addr), dr_cont);
        // Define operand 1 by retrieving the content pointed to by register x.
        let rx_addr = get_rx_addr(payload);
        let op1 = self.registers[rx_addr];
        println!("{:23}[{}] = {:#010X}", "Rx:", get_name(rx_addr), op1);
        // Execute instruction based on the opcode.
        match opcode {
            Opcode::MOV => self.execute(dr_addr, Box::new(move || op1)),
            Opcode::MVN => self.execute(dr_addr, Box::new(move || !op1)),
            Opcode::LDR => {
                let ptr = self.main_memory[op1 as usize].clone();
                println!("{:17}MMem[[{}]] = {:#010X}", "Ptr:", get_name(rx_addr), ptr);
                self.registers[dr_addr] = ptr;
                println!("{:23}[{}] = {:#010X}", "Result:", get_name(dr_addr), ptr);
            }
            Opcode::STR => {
                let payload = self.registers[dr_addr];
                self.main_memory[op1 as usize] = payload;
                println!(
                    "{:17}MMem[[{}]] = {:#010X}",
                    "Result:",
                    get_name(rx_addr),
                    payload
                );
            }
            Opcode::CMP => self.update_flags(dr_cont, op1),
            _ => (),
        }
    }
    fn update_flags(&mut self, op1: u32, op2: u32) {
        // Update carry flag.
        if let Some(result) = op1.checked_sub(op2) {
            self.flag.c = false;
            // Update zero flag.
            if result == 0 {
                self.flag.z = true;
            } else {
                self.flag.z = false;
            }
            // Always positive.
            self.flag.n = false;
        } else {
            self.flag.c = true;
        }
        let op1 = op1 as i32;
        let op2 = op2 as i32;
        // Check oVerflow flag.
        if let Some(result) = op1.checked_sub(op2) {
            self.flag.v = false;
            // Update zero flag.
            if result == 0 {
                self.flag.z = true;
            } else {
                self.flag.z = false;
            }
            // Update negative flag.
            if result < 0 {
                self.flag.n = true;
            } else {
                self.flag.n = false;
            }
        } else {
            self.flag.v = true;
        }
    }
    fn form_four_handler(&mut self, opcode: Opcode, payload: Payload) {
        // Parse the destination address.
        let dr_addr = get_dr_addr(payload);
        let dr_cont = self.registers[dr_addr];
        println!("{:23}[{}] = {:#010X}", "Dr: ", get_name(dr_addr), dr_cont);
        // Define operand 1 by retrieving the content pointed to by register x.
        let rx_addr = get_rx_addr(payload);
        let op1 = self.registers[rx_addr];
        println!("{:23}[{}] = {:#010X}", "Rx:", get_name(rx_addr), op1);
        // Define operand 2 by extracting the immediate 16-bit value.
        let op2 = get_immed16(payload);
        // Execute instruction based on the opcode.
        match opcode {
            Opcode::ADD => self.execute(dr_addr, Box::new(move || op1 + op2)),
            Opcode::AND => self.execute(dr_addr, Box::new(move || op1 & op2)),
            Opcode::EOR => self.execute(dr_addr, Box::new(move || op1 ^ op2)),
            Opcode::MUL => self.execute(dr_addr, Box::new(move || op1 * op2)),
            Opcode::ORR => self.execute(dr_addr, Box::new(move || op1 | op2)),
            Opcode::SUB => self.execute(dr_addr, Box::new(move || op1 - op2)),
            Opcode::LDR => {
                let ptr = self.main_memory[(op1 + op2) as usize].clone();
                println!(
                    "{:11}MMem[[{}] + {:#0X}] = {:#010X}",
                    "Ptr:",
                    get_name(rx_addr),
                    op2,
                    ptr
                );
                self.registers[dr_addr] = ptr;
                println!("{:23}[{}] = {:#010X}", "Result:", get_name(dr_addr), ptr);
            }
            Opcode::STR => {
                let payload = self.registers[dr_addr];
                self.main_memory[(op1 + op2) as usize] = payload;
                println!(
                    "{:9}MMem[[{}] + [{:#0X}]] = {:#010X}",
                    "Result:",
                    get_name(rx_addr),
                    op2,
                    payload
                );
            }
            Opcode::CMP => self.update_flags(dr_cont, op1),
            _ => (),
        }
    }
    fn form_five_handler(&mut self, opcode: Opcode, payload: Payload) {
        // Parse the destination address.
        let dr_addr = get_dr_addr(payload);
        let dr_cont = self.registers[dr_addr];
        println!("{:23}[{}] = {:#010X}", "Dr: ", get_name(dr_addr), dr_cont);
        // Define operand1  by extracting the immediate 20-bit value.
        let op1 = get_immed20(payload);
        // Execute instruction based on the opcode.
        match opcode {
            Opcode::MOV => self.execute(dr_addr, Box::new(move || op1)),
            Opcode::MVN => self.execute(dr_addr, Box::new(move || !op1)),
            Opcode::LDR => {
                // PC reletive mode.
                let ptr = self.main_memory[(self.registers[Register::PC as usize] + op1) as usize]
                    .clone();
                println!("{:18}MMem[{:#0X}] = {:#010X}", "Ptr:", op1, ptr);
                self.registers[dr_addr] = ptr;
                println!("{:23}[{}] = {:#010X}", "Result:", get_name(dr_addr), ptr);
            }
            Opcode::STR => {
                let payload = self.registers[dr_addr];
                // PC reletive mode.
                self.main_memory[(self.registers[Register::PC as usize] + op1) as usize] = payload;
                println!("{:18}MMem[{:#0X}] = {:#010X}", "Result:", op1, payload);
            }
            Opcode::CMP => self.update_flags(dr_cont, op1),
            _ => (),
        }
    }
    fn exe_bcc(&mut self, cond: bool) {
        if cond {
            if let Some(label) = self.variable.reference.get(&self.get_pc()) {
                if let Some(payload) = self.variable.declaration.get(label) {
                    // Subtract one because the main loop will increment program counter.
                    self.set_pc(*payload - 1)
                }
            }
        }
    }

    fn form_six_handler(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::BEQ => self.exe_bcc(self.flag.z),
            Opcode::BNE => self.exe_bcc(!self.flag.z),
            Opcode::BHS => self.exe_bcc(self.flag.c),
            Opcode::BLO => self.exe_bcc(!self.flag.c),
            Opcode::BMI => self.exe_bcc(self.flag.n),
            Opcode::BPL => self.exe_bcc(!self.flag.n),
            Opcode::BVS => self.exe_bcc(self.flag.v),
            Opcode::BVC => self.exe_bcc(!self.flag.v),
            Opcode::BHI => self.exe_bcc(self.flag.c && !self.flag.z),
            Opcode::BLS => self.exe_bcc(!self.flag.c || self.flag.z),
            Opcode::BGE => self.exe_bcc(self.flag.n == self.flag.v),
            Opcode::BLT => self.exe_bcc(self.flag.n != self.flag.v),
            Opcode::BGT => self.exe_bcc(!self.flag.z && (self.flag.n == self.flag.v)),
            Opcode::BLE => self.exe_bcc(self.flag.z || (self.flag.n != self.flag.v)),
            Opcode::B => self.exe_bcc(true),
            _ => (),
        }
    }
    /// Execute instruction and save the result to the destination register.
    fn execute(&mut self, dr_addr: Address, lambda: Box<Fn() -> Payload>) {
        let result = (*lambda)();
        println!("{:23}[{}] = {:#010X}", "Result:", get_name(dr_addr), result);
        self.registers[dr_addr] = result;
    }
    fn register_variable_reference(&mut self, label: Label, pc_ptr: Option<Address>) {
        if let Some(pc_ptr) = pc_ptr {
            self.variable.reference.insert(pc_ptr, label);
        } else {
            self.variable.reference.insert(self.get_pc(), label);
        }
    }
    fn register_variable_declaration(&mut self, label: Label, pc_ptr: Option<u32>) {
        if let Some(pc_ptr) = pc_ptr {
            self.variable.declaration.insert(label, pc_ptr);
        } else {
            self.variable
                .declaration
                .insert(label, self.get_pc() as u32);
        }
    }
    /// Load program into main memory.
    pub fn load_program(&mut self, program: &Vec<(Option<u32>, Option<Label>, Option<Form>)>) {
        let mut pc_ptr = 0;
        for (payload, label, form) in program {
            if let Some(label) = label {
                match form {
                    Some(Form::Six) => {
                        self.register_variable_reference(label.clone(), Some(pc_ptr))
                    }
                    _ => self.register_variable_declaration(label.clone(), Some(pc_ptr as u32)),
                }
            }
            match payload {
                Some(payload) => {
                    self.write_to_mm(pc_ptr, *payload);
                    pc_ptr += 1;
                }
                _ => (),
            }
        }
    }
    /// Run program loaded into main memory.
    pub fn run(&mut self) {
        while self.main_memory[self.get_pc()] != 0 {
            // Fetch and decode a new instruction.
            self.fetch_and_decode(); // This function will invoke the execute function.
            self.incr_pc(); // Increment the program counter.
        }
    }
    /// Execute machine in REPL mode.
    pub fn repl(&mut self) {
        loop {
            self.run(); // If there exist an instruction already in main memory then fetch and execute.
                        // Else read-eval the next instruction from standard input.
            if let Ok((instruction, label, form)) = repl() {
                if let Some(label) = label {
                    match form {
                        Some(Form::Six) => self.register_variable_reference(label, None),
                        _ => self.register_variable_declaration(label, None),
                    }
                }
                match instruction {
                    Some(instruction) => {
                        // Write instruction from standard input to the main memory pointed to by the
                        // program counter.
                        self.write_to_mm(self.get_pc(), instruction);
                        // Fetch and decode a new instruction.
                        self.fetch_and_decode(); // This function will invoke the execute function.
                        self.incr_pc(); // Increment the program counter.
                    }
                    _ => (),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_translator {

    use super::super::util::{Opcode::*, Register::*};
    use super::*;

    #[test]
    fn test_set_pc() {
        let mut vm = Processor::new();
        vm.set_pc(0x1234);
        assert_eq!(vm.registers[PC as Address], 0x1234);
    }

    #[test]
    fn test_incr_pc() {
        let mut vm = Processor::new();
        vm.incr_pc();
        assert_eq!(vm.registers[PC as Address], 0x1);
    }

    #[test]
    fn test_write_to_mm() {
        let mut vm = Processor::new();
        vm.write_to_mm(vm.get_pc(), 0x1234);
        assert_eq!(*vm.read_from_mm(), 0x1234);
    }

    #[test]
    fn test_form_one_add() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.registers[R3 as usize] = 0x3;
        vm.form_one_handler(ADD, 0x01123000);
        assert_eq!(vm.registers[R1 as usize], 0x5);
    }

    #[test]
    fn test_form_one_and() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.registers[R3 as usize] = 0x3;
        vm.form_one_handler(AND, 0x04123000);
        assert_eq!(vm.registers[R1 as usize], 0x2);
    }

    #[test]
    fn test_form_one_eor() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.registers[R3 as usize] = 0x3;
        vm.form_one_handler(EOR, 0x06123000);
        assert_eq!(vm.registers[R1 as usize], 0x1);
    }

    #[test]
    fn test_form_one_mul() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.registers[R3 as usize] = 0x3;
        vm.form_one_handler(MUL, 0x08123000);
        assert_eq!(vm.registers[R1 as usize], 0x6);
    }

    #[test]
    fn test_form_one_orr() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.registers[R3 as usize] = 0x3;
        vm.form_one_handler(ORR, 0x05123000);
        assert_eq!(vm.registers[R1 as usize], 0x3);
    }

    #[test]
    fn test_form_one_sub() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x3;
        vm.registers[R3 as usize] = 0x2;
        vm.form_one_handler(SUB, 0x02123000);
        assert_eq!(vm.registers[R1 as usize], 0x1);
    }

    #[test]
    fn test_form_one_ldr() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.registers[R3 as usize] = 0x3;
        vm.main_memory[0x2 + 0x3] = 0x1234;
        vm.form_one_handler(LDR, 0x32123000);
        assert_eq!(vm.registers[R1 as usize], 0x1234);
    }

    #[test]
    fn test_form_one_str() {
        let mut vm = Processor::new();
        vm.registers[R1 as usize] = 0x1234;
        vm.registers[R2 as usize] = 0x2;
        vm.registers[R3 as usize] = 0x3;
        vm.form_one_handler(STR, 0x36123000);
        assert_eq!(vm.main_memory[0x2 + 0x3], 0x1234);
    }

    #[test]
    fn test_form_two_mov() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.form_two_handler(MOV, 0x03120000);
        assert_eq!(vm.registers[R1 as usize], 0x2);
    }

    #[test]
    fn test_form_two_mvn() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.form_two_handler(MVN, 0x07120000);
        assert_eq!(vm.registers[R1 as usize], 0xFFFFFFFD);
    }

    #[test]
    fn test_form_two_ldr() {
        let mut vm = Processor::new();
        vm.registers[R2 as usize] = 0x2;
        vm.main_memory[0x2] = 0x1234;
        vm.form_two_handler(LDR, 0x30120000);
        assert_eq!(vm.registers[R1 as usize], 0x1234);
    }

    #[test]
    fn test_form_two_str() {
        let mut vm = Processor::new();
        vm.registers[R1 as usize] = 0x1234;
        vm.registers[R2 as usize] = 0x2;
        vm.form_two_handler(STR, 0x34120000);
        assert_eq!(vm.main_memory[0x2], 0x1234);
    }

    #[test]
    fn test_form_two_cmp() {
        assert!(false);
    }

    #[test]
    fn test_update_flags() {
        assert!(false);
    }

    #[test]
    fn test_update_flags_with_carry() {
        assert!(false);
    }

    #[test]
    fn test_update_flags_with_overflow() {
        assert!(false);
    }

    #[test]
    fn test_update_flags_with_negative() {
        assert!(false);
    }

    #[test]
    fn test_update_flags_with_zero() {
        assert!(false);
    }

    #[test]
    fn test_form_four_add() {
        assert!(false);
    }

    #[test]
    fn test_form_four_and() {
        assert!(false);
    }

    #[test]
    fn test_form_four_eor() {
        assert!(false);
    }

    #[test]
    fn test_form_four_mul() {
        assert!(false);
    }

    #[test]
    fn test_form_four_orr() {
        assert!(false);
    }

    #[test]
    fn test_form_four_sub() {
        assert!(false);
    }

    #[test]
    fn test_form_four_ldr() {
        assert!(false);
    }

    #[test]
    fn test_form_four_str() {
        assert!(false);
    }

    #[test]
    fn test_form_five_mov() {
        assert!(false);
    }

    #[test]
    fn test_form_five_mvn() {
        assert!(false);
    }

    #[test]
    fn test_form_five_ldr() {
        assert!(false);
    }

    #[test]
    fn test_form_five_str() {
        assert!(false);
    }

    #[test]
    fn test_form_five_cmp() {
        assert!(false);
    }

    #[test]
    fn test_exe_bcc() {
        assert!(false);
    }

    #[test]
    fn test_form_six_beq() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bne() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bhs() {
        assert!(false);
    }

    #[test]
    fn test_form_six_blo() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bmi() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bpl() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bvs() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bvc() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bhi() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bls() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bge() {
        assert!(false);
    }

    #[test]
    fn test_form_six_blt() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bgt() {
        assert!(false);
    }

    #[test]
    fn test_form_six_ble() {
        assert!(false);
    }

    #[test]
    fn test_form_six_bal() {
        assert!(false);
    }

    #[test]
    fn test_execute() {
        assert!(false);
    }

    #[test]
    fn test_register_variable_reference() {
        assert!(false);
    }

    #[test]
    fn test_register_variable_declaration() {
        assert!(false);
    }

    #[test]
    fn test_load_program() {
        assert!(false);
    }

    #[test]
    fn test_run() {
        assert!(false);
    }

}

use super::interpreter::{repl, Label};
use super::util::{
    get_dr_addr, get_form_and_opcode, get_immed16, get_immed20, get_rx_addr, get_ry_addr, Form,
    Opcode,
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

/// A virtual processor has virtual registers and memory.
pub struct Processor {
    registers: Vec<u32>,
    main_memory: Vec<u32>,
    labels: HashMap<Label, Address>,
}

pub type Payload = u32;
pub type Address = usize;

impl Processor {
    /// Instantiate a new processor.
    pub fn new() -> Processor {
        Processor {
            registers: vec![INIT_REGISTER_VALUE; N_REGISTERS_IN_PROCESSOR],
            main_memory: vec![INIT_REGISTER_VALUE; N_REGISTERS_IN_MAIN_MEMORY],
            labels: HashMap::new(),
        }
    }
    /// Get the contents stored in the program counter.
    fn get_pc(&self) -> Address {
        self.registers[Register::PC as Address] as Address
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
        let dr_cont = self.registers[dr_addr];
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
            _ => (),
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
            _ => (),
        }
    }
    /// Execute instruction and save the result to the destination register.
    fn execute(&mut self, dr_addr: Address, lambda: Box<Fn() -> Payload>) {
        let result = (*lambda)();
        println!("{:23}[{}] = {:#010X}", "Result:", get_name(dr_addr), result);
        self.registers[dr_addr] = result;
    }
    fn register_labels(&mut self, labels: Vec<Label>) {
        for label in labels {
            self.labels.insert(label, self.get_pc());
        }
    }

    /// Load program into main memory.
    pub fn load_program(&mut self, program: &Vec<(u32, Vec<Label>)>) {
        let mut pc_ptr = 0;
        for (payload, labels) in program {
            self.register_labels(labels.to_vec());
            self.write_to_mm(pc_ptr, *payload);
            pc_ptr += 1;
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
            if let Ok((instruction, labels)) = repl() {
                self.register_labels(labels.to_vec());
                // Write instruction from standard input to the main memory pointed to by the
                // program counter.
                self.write_to_mm(self.get_pc(), instruction);
                // Fetch and decode a new instruction.
                self.fetch_and_decode(); // This function will invoke the execute function.
                self.incr_pc(); // Increment the program counter.
            }
        }
    }
}

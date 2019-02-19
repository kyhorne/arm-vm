use super::interpreter::repl;
use super::util::{Register, get_name};
use super::util::{
	Form,
	Opcode,
	is_mode_bit_toggled,
	get_form_and_opcode,
	get_dr_addr,
	get_rx_addr,
	get_ry_addr,
	get_immed16,
	get_immed20
};

/// A virtual processor has virtual registers and memory.
pub struct Processor {
	registers:   Vec<u32>,
	main_memory: Vec<u32>
}

pub type Payload = u32;
pub type Address = usize;

/// The initial value of all registers in the processor.
pub const INIT_REGISTER_VALUE: Payload = 0;

/// The number of addressable registers in this processor. A 32-bit ARM processor has 16 registers.
const N_REGISTERS_IN_PROCESSOR: Address = 0x10;

/// The number of addressable registers in main memory. A 32-bit processor has 2^32 addressable memory locations.
const N_REGISTERS_IN_MAIN_MEMORY: Address = std::u32::MAX as Address;

impl Processor {
	/// Instantiate a new processor.
	pub fn new() -> Processor {
		Processor {
			registers:   vec![INIT_REGISTER_VALUE; N_REGISTERS_IN_PROCESSOR],
			main_memory: vec![INIT_REGISTER_VALUE; N_REGISTERS_IN_MAIN_MEMORY]
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
	pub fn write_to_mm(&mut self, address: Address, payload: &Payload) {
		self.main_memory[address] = *payload
	}
	/// Read data from the main memory pointed to by the program counter.
	fn read_from_mm(&self) -> Payload {
		self.main_memory[self.get_pc()]
	}
	/// Fetch and decode instruction pointed to by the program counter.
	fn fetch_and_decode(&mut self) {
		// Read data from the main memory pointed to by the program counter.
		let payload = self.read_from_mm();
		println!("{:11}{:>8} = {:#010X} ", "Payload:", "MMem[[PC]]", payload);
		// Extract the opcode and form from the payload.
		if let Ok((opcode, form)) = get_form_and_opcode(payload) {
			println!("{:18}{:?}", "Opcode:", opcode);
			// Execute the instruction handler based on payload form.
			match form {
				Form::One | Form::Four => self.form_one_and_four_handler(payload, opcode),
				Form::Two              => self.form_two_handler(payload, opcode),
				Form::Five             => self.form_five_handler(payload, opcode)
			}
		}
	}
	/// Parse a form one or form four instruction and then execute.
	fn form_one_and_four_handler(&mut self, payload: Payload, opcode: Opcode) {
		// Parse the destination address.
		let dr_addr = get_dr_addr(payload);
		// Define operand 1 by retrieving the content pointed to by register x.
		let rx_addr = get_rx_addr(payload);
		let op1 = self.registers[rx_addr];
		println!("{:17}[{}] = {:#010X}", "Rx:", get_name(rx_addr), op1);
		// Define operand 2 depending on the mode bit.
		let op2 = if is_mode_bit_toggled(payload) {
			get_immed16(payload)
		} else {
			let ry_addr = get_ry_addr(payload);
			let op2 = self.registers[ry_addr];
			println!("{:17}[{}] = {:#010X}", "Ry:", get_name(ry_addr), op2);
			op2
		};
		// Capture state of the parsed instruction by encapsulating state inside of closure and then call the execute function.
		match opcode {
			Opcode::ADD => self.execute(dr_addr, Box::new(move || op1 + op2)),
			Opcode::AND => self.execute(dr_addr, Box::new(move || op1 & op2)),
			Opcode::EOR => self.execute(dr_addr, Box::new(move || op1 ^ op2)),
			Opcode::MUL => self.execute(dr_addr, Box::new(move || op1 * op2)),
			Opcode::ORR => self.execute(dr_addr, Box::new(move || op1 | op2)),
			Opcode::SUB => self.execute(dr_addr, Box::new(move || op1 - op2)),
			_ => ()
		}
	}
	/// Parse a form five instruction and then execute.
	pub fn form_five_handler(&mut self, payload: Payload, opcode: Opcode) {
		// Parse the destination address.
		let dr_addr = get_dr_addr(payload);
		// Parse the immediate 20-bit value.
		let op1 = get_immed20(payload);
		// Capture state of the parsed instruction by encapsulating state inside of closure and then call the execute function.
		match opcode {
			Opcode::MOV => self.execute(dr_addr, Box::new(move || op1)),
			Opcode::MVN => self.execute(dr_addr, Box::new(move || !op1)),
			_ => ()
		}
	}
	/// Parse a form two instruction and then execute.
	pub fn form_two_handler(&mut self, payload: Payload, opcode: Opcode) {
		// Parse the destination address.
		let dr_addr = get_dr_addr(payload);
		// Define operand 1 by retrieving the content pointed to by register x.
		let rx_addr = get_rx_addr(payload);
		let op1 = self.registers[rx_addr];
		println!("{:17}[{}] = {:#010X}", "Rx:", get_name(rx_addr), op1);
		match opcode {
			Opcode::MOV => self.execute(dr_addr, Box::new(move || op1)),
			Opcode::MVN => self.execute(dr_addr, Box::new(move || !op1)),
			_ => ()
		}
	}
	/// Execute instruction and save the result to the destination register.
	fn execute(&mut self, dr_addr: Address, lambda: Box<Fn() -> Payload>) {
		let result = (*lambda)();
		println!("{:17}[{}] = {:#010X}", "Result", get_name(dr_addr), result);
		self.registers[dr_addr] = result;
	}
	/// Load program into main memory.
	pub fn load_program(&mut self, program: &Vec<u32>) {
		let mut pc_ptr = 0;
		for expression in program {
			self.write_to_mm(pc_ptr, expression);
			pc_ptr += 1;
		}
	}
	/// Run program loaded into main memory.
	pub fn run(&mut self) {
		while self.main_memory[self.get_pc()] != 0 {
			// Fetch and decode a new instruction.
			self.fetch_and_decode(); // This function will invoke the execute function.
			// Increment the program counter.
			self.incr_pc();
		}
	}
	/// Execute machine in REPL mode.
	pub fn repl(&mut self) {
		loop {
			if let Some(instruction) = repl() {
				// Write instruction from standard input to the main memory pointed to by program
				// counter + 1.
				self.write_to_mm(self.get_pc(), &instruction);
				// Fetch and decode a new instruction.
				self.fetch_and_decode(); // This function will invoke the execute function.
				// Increment the program counter.
				self.incr_pc();
			}
		}
	}
}

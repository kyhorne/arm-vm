# arm-vm

This project is a virtualization of a 32-bit processor inspired by the ARM microarchitecture. It is integrated with an assembler that can parse native ARM assembly opcodes into its mapped bytecode. Additionally, it is capable of logging the complete execution cycle; thereby, allowing students to visually understand the inner workings of an ARM processor. 

Can support 2^8 different operations and can address 2^32 memory locations. Each memory location can store a 32-bit word.

## Usage

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, run:

```
git clone https://github.com/kyhorne/arm-vm.git
cd arm-vm
cargo run -- -h
```


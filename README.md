# arm-vm

This project is a virtualization of a 32-bit processor inspired by the ARM microarchitecture. This virtual processor supports 2^8 different operations and can address 2^32 memory locations. Each memory location can store a 32-bit word.

The supported operation codes are located in: `~/src/util/opcode.rs`. Each operation is one of the following forms:

- Form One:
  - OP DR, RX, RY ; DR <- [RX] OP [RY]
  - Example: EOR R5, R1, R10
- Form Two:
  - OP DR, RX ; DR <- OP([RX])
  - Example: MOV R5, R1
- Form Four:
  - OP DR, RX, #immed16 ; DR <- [RX] OP #immed16
  - Example: ADD R4, R4, #1
- Form Five:

  - OP DR, #immed20 ; DR <- OP(#immed20)
  - Example: MVN R5, #0xF1234

## Usage

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, run:

```
git clone https://github.com/kyhorne/arm-vm.git
cd arm-vm
cargo run -- -h
```

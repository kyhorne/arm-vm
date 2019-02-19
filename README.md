# arm-vm

This project is a virtualization of a 32-bit processor inspired by the ARM microarchitecture. This virtual processor supports 2^8 different operations and can address 2^32 memory locations. Each memory location can store a 32-bit word.

The supported operation codes are located in: `~/src/util/opcode.rs`.

## Usage

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, run:

```
git clone https://github.com/kyhorne/arm-vm.git
cd arm-vm
cargo run -- -h
```

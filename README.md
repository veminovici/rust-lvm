# RUST-LVM
A language virtual machine in Rust.

## Goal
Practice different Rust aspects, parsing strings and bytes using nom.

## Build, Test

```
cargo build --workspace
cargo test --workspace
```

## Projects

- **lvm-core** contains the core structures (e.g. RIndex, Operand8, Instruction, Program)
- **lvm-parser** contains the traits and the parsing of strings or bytes.
- **lvm-machine** contains core structures related to virtual machine.
- **lvm-repl** implements a REPL application.

## REPL
The REPL application supports several commands:
- *:q* - terminates the application
- *:h* - prints the help
- *:i* - prints the internal information
- *:ix* - prints the internal information in hex format
- *LOAD $1 #10* - executes a load instruction
- *ADD $1 $2 $3* - executed an add instruction
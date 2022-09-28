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
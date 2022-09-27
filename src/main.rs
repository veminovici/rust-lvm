use lvm_core::Program;
use lvm_parser::ParseBytes;
fn main() {
    println!("Testing program!");

    let input = [1u8, 10u8, 1u8, 0xF4u8, 2u8, 10u8, 20u8, 30u8].as_slice();
    let (_, program) = Program::parse_bytes(input).unwrap();
    let bytes: Vec<u8> = program.into();

    assert_eq!(bytes, input);
}

use crate::{ParseBytes, ParseString, Result};

use lvm_core::{Instruction, Program};
use nom::{
    character::complete::multispace1,
    combinator::map,
    error::context,
    multi::{many0, separated_list0},
};

const CONTEXT: &str = "program";

impl ParseString for Program {
    type Output = Self;

    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        let f = separated_list0(multispace1, Instruction::parse_str);
        let ff = map(f, Program::make);
        context(CONTEXT, ff)(input)
    }

    fn parse_hex_str(input: &str) -> Result<&str, Self::Output> {
        let f = separated_list0(multispace1, Instruction::parse_hex_str);
        let ff = map(f, Program::make);
        context(CONTEXT, ff)(input)
    }
}

impl ParseBytes for Program {
    type Output = Self;

    fn parse_bytes(input: &[u8]) -> Result<&[u8], Self::Output> {
        let f = map(many0(Instruction::parse_bytes), Program::make);
        context(CONTEXT, f)(input)
    }
}

#[cfg(test)]
mod tests {
    use lvm_core::{Add, Load, Operand16, RIndex};

    use super::*;

    fn create_load() -> Instruction {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        Instruction::LoadI(load)
    }

    fn create_add() -> Instruction {
        let rindx1 = RIndex::make(10u8);
        let rindx2 = RIndex::make(20u8);
        let rindx3 = RIndex::make(30u8);
        let add = Add::make(rindx1, rindx2, rindx3);
        Instruction::AddI(add)
    }

    #[test]
    fn parse_str() {
        let input = "LOAD $10 #500\nADD $10 $20 $30";
        let res = Program::parse_str(input);
        assert!(res.is_ok());

        let (_, program) = res.unwrap();
        let mut instructions = program.into_iter();
        assert_eq!(instructions.next(), Some(create_load()));
        assert_eq!(instructions.next(), Some(create_add()));
        assert!(instructions.next().is_none())
    }

    #[test]
    fn parse_hex_str() {
        let input = "LOAD $0A #01F4\nADD $0A $14 $1E";
        let res = Program::parse_hex_str(input);
        assert!(res.is_ok());

        let (_, program) = res.unwrap();
        let mut instructions = program.into_iter();
        assert_eq!(instructions.next(), Some(create_load()));
        assert_eq!(instructions.next(), Some(create_add()));
        assert!(instructions.next().is_none())
    }

    #[test]
    fn parse_bytes() {
        let input = [1u8, 10u8, 1u8, 0xF4u8, 2u8, 10u8, 20u8, 30u8, 0u8].as_slice();
        let res = Program::parse_bytes(input);
        assert!(res.is_ok());

        let (_, program) = res.unwrap();
        let mut instructions = program.into_iter();
        assert_eq!(instructions.next(), Some(create_load()));
        assert_eq!(instructions.next(), Some(create_add()));
        assert!(instructions.next().is_none())
    }
}

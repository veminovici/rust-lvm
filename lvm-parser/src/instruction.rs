use crate::{ParseBytes, ParseString, Result};

use lvm_core::{Add, Instruction, Load};
use nom::{branch::alt, combinator::map, error::context};

const CONTEXT: &str = "instruction";

impl ParseString for Instruction {
    type Output = Self;

    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        let load = map(Load::parse_str, Instruction::LoadI);
        let add = map(Add::parse_str, Instruction::AddI);

        let f = alt((load, add));
        context(CONTEXT, f)(input)
    }

    fn parse_hex_str(input: &str) -> Result<&str, Self::Output> {
        let load = map(Load::parse_hex_str, Instruction::LoadI);
        let add = map(Add::parse_hex_str, Instruction::AddI);

        let f = alt((load, add));
        context(CONTEXT, f)(input)
    }
}

impl ParseBytes for Instruction {
    type Output = Self;

    fn parse_bytes(input: &[u8]) -> Result<&[u8], Self::Output> {
        let load = map(Load::parse_bytes, Instruction::LoadI);
        let add = map(Add::parse_bytes, Instruction::AddI);

        let f = alt((load, add));
        context(CONTEXT, f)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str() {
        let input = "LOAD $10 #500";

        let res = Instruction::parse_str(input);
        assert!(res.is_ok());

        let (_, instruction) = res.unwrap();

        match instruction {
            Instruction::LoadI(load) => {
                assert_eq!(10u8, load.index().into());
                assert_eq!(500u16, load.operand().into());
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn parse_hex_str() {
        let input = "LOAD $0A #01F4";

        let res = Instruction::parse_hex_str(input);
        assert!(res.is_ok());

        let (_, instruction) = res.unwrap();

        match instruction {
            Instruction::LoadI(load) => {
                assert_eq!(10u8, load.index().into());
                assert_eq!(500u16, load.operand().into());
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn parse_bytes() {
        let input = [1u8, 10u8, 50u8, 1u8, 0u8].as_slice();

        let res = Instruction::parse_bytes(input);
        assert!(res.is_ok());

        let (_, instruction) = res.unwrap();

        match instruction {
            Instruction::LoadI(load) => {
                assert_eq!(10u8, load.index().into());
                assert_eq!(((50u16 << 8) + 1u16), load.operand().into());
            }
            _ => assert!(false),
        }
    }
}

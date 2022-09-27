use crate::{ParseBytes, ParseString, Result};

use lvm_core::Operand16;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, hex_digit1},
    combinator::{map, map_res},
    error::context,
    sequence::preceded,
};

const CONTEXT: &str = "operand16";
const LEN: usize = 2;

impl ParseString for Operand16 {
    type Output = Self;

    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        let fst = tag(Operand16::PREFIX);
        let snd = map_res(digit1, Operand16::try_from);

        context(CONTEXT, preceded(fst, snd))(input)
    }

    fn parse_hex_str(input: &str) -> Result<&str, Self::Output> {
        let fst = tag(Operand16::PREFIX);
        let snd = map_res(hex_digit1, Operand16::try_from_hex);
        context(CONTEXT, preceded(fst, snd))(input)
    }
}

impl ParseBytes for Operand16 {
    type Output = Self;

    fn parse_bytes(input: &[u8]) -> Result<&[u8], Self::Output> {
        let f = map(take(LEN), Operand16::from);
        context(CONTEXT, f)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str() {
        let input = "#10 ABC";
        let (rst, index) = Operand16::parse_str(input).unwrap();

        assert_eq!(" ABC", rst);
        assert_eq!(10u16, index.into());
    }

    #[test]
    fn parse_hex_str() {
        let input = "#0A ABC";
        let (rst, index) = Operand16::parse_hex_str(input).unwrap();

        assert_eq!(" ABC", rst);
        assert_eq!(10u16, index.into());
    }
}

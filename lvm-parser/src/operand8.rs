use crate::{ParseBytes, ParseString, Result};

use lvm_core::Operand8;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, hex_digit1},
    combinator::{map, map_res},
    error::context,
    sequence::preceded,
};

const PREFIX: &str = "#";
const CONTEXT: &str = "operand8";
const LEN: usize = 1;

impl ParseString for Operand8 {
    type Output = Self;

    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        let fst = tag(PREFIX);
        let snd = map_res(digit1, Operand8::try_from);

        context(CONTEXT, preceded(fst, snd))(input)
    }

    fn parse_hex_str(input: &str) -> Result<&str, Self::Output> {
        let fst = tag(PREFIX);
        let snd = map_res(hex_digit1, Operand8::try_from_hex);
        context(CONTEXT, preceded(fst, snd))(input)
    }
}

impl ParseBytes for Operand8 {
    type Output = Self;

    fn parse_bytes(input: &[u8]) -> Result<&[u8], Self::Output> {
        let f = map(take(LEN), Operand8::from);
        context(CONTEXT, f)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str() {
        let input = "#10 ABC";
        let (rst, index) = Operand8::parse_str(input).unwrap();

        assert_eq!(" ABC", rst);
        assert_eq!(10u8, index.into());
    }

    #[test]
    fn parse_hex_str() {
        let input = "#0A ABC";
        let (rst, index) = Operand8::parse_hex_str(input).unwrap();

        assert_eq!(" ABC", rst);
        assert_eq!(10u8, index.into());
    }
}

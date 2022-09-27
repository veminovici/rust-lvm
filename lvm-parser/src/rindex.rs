use crate::{ParseBytes, ParseString, Result};

use lvm_core::RIndex;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, hex_digit1},
    combinator::{map, map_res},
    error::context,
    sequence::preceded,
};

const CONTEXT: &str = "rindex";
const LEN: usize = 1;

impl ParseString for RIndex {
    type Output = Self;

    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        let fst = tag(RIndex::PREFIX);
        let snd = map_res(digit1, RIndex::try_from);

        context(CONTEXT, preceded(fst, snd))(input)
    }

    fn parse_hex_str(input: &str) -> Result<&str, Self::Output> {
        let fst = tag(RIndex::PREFIX);
        let snd = map_res(hex_digit1, RIndex::try_from_hex);
        context(CONTEXT, preceded(fst, snd))(input)
    }
}

impl ParseBytes for RIndex {
    type Output = Self;

    fn parse_bytes(input: &[u8]) -> Result<&[u8], Self::Output> {
        let f = map(take(LEN), RIndex::from);

        context(CONTEXT, f)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str() {
        let input = "$10 ABC";
        let (rst, index) = RIndex::parse_str(input).unwrap();

        assert_eq!(" ABC", rst);
        assert_eq!(10u8, index.into());
    }

    #[test]
    fn parse_hex_str() {
        let input = "$0A ABC";
        let (rst, index) = RIndex::parse_hex_str(input).unwrap();

        assert_eq!(" ABC", rst);
        assert_eq!(10u8, index.into());
    }
}

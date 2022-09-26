use crate::{ParseString, ParseBytes, Result};

use lvm_core::RIndex;
use nom::{
    bytes::complete::{tag, take},
    combinator::{map, map_res},
    error::context,
    sequence::preceded,
};

const PREFIX: &str = "$";
const LEN: usize = 1;

impl ParseString for RIndex {
    type Output = Self;

    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        let fst = tag(PREFIX);
        let snd = map_res(super::take_while_digit(), RIndex::try_from);
    
        let ctx = std::any::type_name::<RIndex>();
        context(ctx, preceded(fst, snd))(input)
    }

    fn parse_hex_str(input: &str) -> Result<&str, Self::Output> {
        let fst = tag(PREFIX);
        let snd = map_res(super::take_while_hex_digit(), RIndex::try_from);
    
        let ctx = std::any::type_name::<RIndex>();
        context(ctx, preceded(fst, snd))(input)
    }
}

impl ParseBytes for RIndex {
    type Output = Self;

    fn parse_u8s(input: &[u8]) -> Result<&[u8], Self::Output> {
        let f = map(take(LEN), RIndex::from);

        let ctx = std::any::type_name::<RIndex>();
        context(ctx, f)(input)
    }
}

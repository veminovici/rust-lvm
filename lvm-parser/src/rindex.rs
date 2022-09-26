use crate::{Parseable, Result};

use lvm_core::RIndex;
use nom::{
    bytes::complete::{tag, take},
    combinator::{map, map_res},
    error::context,
    sequence::preceded,
};

const PREFIX: &str = "$";
const LEN: usize = 1;

fn from_str(input: &str) -> Result<&str, RIndex> {
    let fst = tag(PREFIX);
    let snd = map_res(super::take_while_digit(), RIndex::try_from);

    let ctx = std::any::type_name::<RIndex>();
    context(ctx, preceded(fst, snd))(input)
}

fn from_u8s(input: &[u8]) -> Result<&[u8], RIndex> {
    let f = map(take(LEN), RIndex::from);

    let ctx = std::any::type_name::<RIndex>();
    context(ctx, f)(input)
}

impl Parseable for RIndex {
    type Output = Self;

    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        from_str(input)
    }

    fn parse_u8s(input: &[u8]) -> Result<&[u8], Self::Output> {
        from_u8s(input)
    }
}

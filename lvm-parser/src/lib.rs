mod rindex;

pub use rindex::*;

use nom::{bytes::complete::take_while, error::VerboseError, IResult};

pub type Result<I, O> = IResult<I, O, VerboseError<I>>;

fn is_digit(c: char) -> bool {
    nom::character::is_digit(c as u8)
}

pub(crate) fn take_while_digit<'a>() -> impl Fn(&'a str) -> Result<&'a str, &'a str> {
    take_while(is_digit)
}

pub trait Parseable<Output = Self> {
    type Output;

    fn parse_str(input: &str) -> Result<&str, Self::Output>;
    fn parse_u8s(input: &[u8]) -> Result<&[u8], Self::Output>;
}

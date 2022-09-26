mod rindex;

pub use rindex::*;

use nom::{bytes::complete::take_while, error::VerboseError, IResult};

pub type Result<I, O> = IResult<I, O, VerboseError<I>>;

#[inline]
fn is_digit(chr: char) -> bool {
    nom::character::is_digit(chr as u8)
}

#[inline]
fn is_hex_digit(chr: char) -> bool {
    nom::character::is_hex_digit(chr as u8)
}

pub(crate) fn take_while_digit<'a>() -> impl Fn(&'a str) -> Result<&'a str, &'a str> {
    take_while(is_digit)
}

pub(crate) fn take_while_hex_digit<'a>() -> impl Fn(&'a str) -> Result<&'a str, &'a str> {
    take_while(is_hex_digit)
}

pub trait ParseString<Output = Self> {
    type Output;

    fn parse_str(input: &str) -> Result<&str, Self::Output>;
    fn parse_hex_str(input: &str) -> Result<&str, Self::Output>;
}

pub trait ParseBytes {
    type Output;

    fn parse_u8s(input: &[u8]) -> Result<&[u8], Self::Output>;
}

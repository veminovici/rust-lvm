mod operand16;
mod operand8;
mod rindex;

pub use operand16::*;
pub use operand8::*;
pub use rindex::*;

use nom::{error::VerboseError, IResult};

pub type Result<I, O> = IResult<I, O, VerboseError<I>>;

pub trait ParseString<Output = Self> {
    type Output;

    fn parse_str(input: &str) -> Result<&str, Self::Output>;
    fn parse_hex_str(input: &str) -> Result<&str, Self::Output>;
}

pub trait ParseBytes {
    type Output;

    fn parse_u8s(input: &[u8]) -> Result<&[u8], Self::Output>;
}

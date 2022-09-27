mod add;
mod load;
mod operand16;
mod operand8;
mod rindex;

pub use add::*;
pub use load::*;
pub use operand16::*;
pub use operand8::*;
pub use rindex::*;

use nom::{error::VerboseError, IResult};

pub type Result<I, O> = IResult<I, O, VerboseError<I>>;

/// A trait that encapsulates the functions
/// that parse a string or a hex-string.
pub trait ParseString<Output = Self> {
    type Output;

    /// Parses a string
    fn parse_str(input: &str) -> Result<&str, Self::Output>;

    /// Parses a hex-string
    fn parse_hex_str(input: &str) -> Result<&str, Self::Output>;
}

/// A trait that encapsulates the functions
/// that parse a slice of bytes.
pub trait ParseBytes {
    type Output;

    /// Parses a slice of bytes.
    fn parse_bytes(input: &[u8]) -> Result<&[u8], Self::Output>;
}

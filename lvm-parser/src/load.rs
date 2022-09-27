use crate::{ParseBytes, ParseString, Result};

use lvm_core::{Load, Operand16, RIndex};
use nom::{bytes::complete::tag, character::complete::multispace1, error::context};

const CONTEXT: &str = "load";

fn load_from_str(input: &str) -> Result<&str, Load> {
    let (input, _) = tag(Load::PREFIX)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, indx) = RIndex::parse_str(input)?;
    let (input, _) = multispace1(input)?;
    let (input, oprnd) = Operand16::parse_str(input)?;

    let load = Load::make(indx, oprnd);

    Ok((input, load))
}

fn load_from_hex_str(input: &str) -> Result<&str, Load> {
    let (input, _) = tag(Load::PREFIX)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, indx) = RIndex::parse_hex_str(input)?;
    let (input, _) = multispace1(input)?;
    let (input, oprnd) = Operand16::parse_hex_str(input)?;

    let load = Load::make(indx, oprnd);

    Ok((input, load))
}

fn load_from_bytes(input: &[u8]) -> Result<&[u8], Load> {
    let (input, _) = tag([Load::ID])(input)?;
    let (input, indx) = RIndex::parse_bytes(input)?;
    let (input, oprnd) = Operand16::parse_bytes(input)?;

    let load = Load::make(indx, oprnd);

    Ok((input, load))
}

impl ParseString for Load {
    type Output = Self;

    /// Tries to create an [`Load`] instance by parsing a string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lvm_core::{Load, RIndex, Operand16};
    /// use lvm_parser::*;
    /// 
    /// let input = "LOAD $10 #500";
    /// let (_, load) = Load::parse_str(input).unwrap();
    /// 
    /// assert_eq!(10u8, load.index().into());
    /// assert_eq!(500u16, load.operand().into());
    /// ```
    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        context(CONTEXT, load_from_str)(input)
    }

    /// Tries to create an [`Load`] instance by parsing a hex string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lvm_core::{Load, RIndex, Operand16};
    /// use lvm_parser::*;
    /// 
    /// let input = "LOAD $0A #01F4";
    /// let (_, load) = Load::parse_hex_str(input).unwrap();
    /// 
    /// assert_eq!(10u8, load.index().into());
    /// assert_eq!(500u16, load.operand().into());
    /// ```
    fn parse_hex_str(input: &str) -> Result<&str, Self::Output> {
        context(CONTEXT, load_from_hex_str)(input)
    }
}

impl ParseBytes for Load {
    type Output = Self;

    /// Tries to create an [`Load`] instance by parsing a slide of bytes
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lvm_core::{Load, RIndex, Operand16};
    /// use lvm_parser::*;
    /// 
    /// let input = [1u8, 10u8, 50u8, 1u8, 0u8].as_slice();
    /// let (_, load) = Load::parse_bytes(input).unwrap();
    /// 
    /// assert_eq!(10u8, load.index().into());
    /// assert_eq!(((50u16 << 8) + 1u16), load.operand().into());
    /// ```
    fn parse_bytes(input: &[u8]) -> Result<&[u8], Self::Output> {
        context(CONTEXT, load_from_bytes)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str() {
        let input = "LOAD $10 #500";

        let res = Load::parse_str(input);
        assert!(res.is_ok());

        let load = res.unwrap().1;

        assert_eq!(10u8, load.index().into());
        assert_eq!(500u16, load.operand().into());
    }

    #[test]
    fn parse_hex_str() {
        let input = "LOAD $0A #01F4";

        let res = Load::parse_hex_str(input);
        assert!(res.is_ok());

        let load = res.unwrap().1;

        assert_eq!(10u8, load.index().into());
        assert_eq!(500u16, load.operand().into());
    }

    #[test]
    fn parse_bytes() {
        let input = [1u8, 10u8, 50u8, 1u8, 0u8].as_slice();

        let res = Load::parse_bytes(input);
        assert!(res.is_ok());

        let (rst, load) = res.unwrap();

        assert_eq!(1, rst.len());
        assert_eq!(10u8, load.index().into());
        assert_eq!(((50u16 << 8) + 1u16), load.operand().into());
    }
}

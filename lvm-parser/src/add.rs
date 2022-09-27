use crate::{ParseBytes, ParseString, Result};

use lvm_core::{Add, RIndex};
use nom::{bytes::complete::tag, character::complete::multispace1, error::context};

const CONTEXT: &str = "add";

fn load_from_str(input: &str) -> Result<&str, Add> {
    let (input, _) = tag(Add::PREFIX)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, indx1) = RIndex::parse_str(input)?;
    let (input, _) = multispace1(input)?;
    let (input, indx2) = RIndex::parse_str(input)?;
    let (input, _) = multispace1(input)?;
    let (input, indx3) = RIndex::parse_str(input)?;

    let add = Add::make(indx1, indx2, indx3);

    Ok((input, add))
}

fn load_from_hex_str(input: &str) -> Result<&str, Add> {
    let (input, _) = tag(Add::PREFIX)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, indx1) = RIndex::parse_hex_str(input)?;
    let (input, _) = multispace1(input)?;
    let (input, indx2) = RIndex::parse_hex_str(input)?;
    let (input, _) = multispace1(input)?;
    let (input, indx3) = RIndex::parse_hex_str(input)?;

    let add = Add::make(indx1, indx2, indx3);

    Ok((input, add))
}

fn load_from_bytes(input: &[u8]) -> Result<&[u8], Add> {
    let (input, _) = tag([Add::ID])(input)?;
    let (input, indx1) = RIndex::parse_bytes(input)?;
    let (input, indx2) = RIndex::parse_bytes(input)?;
    let (input, indx3) = RIndex::parse_bytes(input)?;

    let add = Add::make(indx1, indx2, indx3);

    Ok((input, add))
}

impl ParseString for Add {
    type Output = Self;

    /// Tries to create an [`Add`] instance by parsing a string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lvm_core::{Add, RIndex};
    /// use lvm_parser::*;
    /// 
    /// let input = "ADD $10 $20 $30";
    /// let (_, add) = Add::parse_str(input).unwrap();
    /// 
    /// assert_eq!(10u8, add.index1().into());
    /// assert_eq!(20u8, add.index2().into());
    ///assert_eq!(30u8, add.index3().into());
    /// ```
    fn parse_str(input: &str) -> Result<&str, Self::Output> {
        context(CONTEXT, load_from_str)(input)
    }

    /// Tries to create an [`Add`] instance by parsing a hex string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lvm_core::{Add, RIndex};
    /// use lvm_parser::*;
    /// 
    /// let input = "ADD $0A $14 $1E";
    /// let (_, add) = Add::parse_hex_str(input).unwrap();
    /// 
    /// assert_eq!(10u8, add.index1().into());
    /// assert_eq!(20u8, add.index2().into());
    ///assert_eq!(30u8, add.index3().into());
    /// ```
    fn parse_hex_str(input: &str) -> Result<&str, Self::Output> {
        context(CONTEXT, load_from_hex_str)(input)
    }
}

impl ParseBytes for Add {
    type Output = Self;

    /// Tries to create an [`Add`] instance by parsing a hex string
    /// 
    /// # Examples
    /// 
    /// ```
    /// use lvm_core::{Add, RIndex};
    /// use lvm_parser::*;
    /// 
    /// let input = [2u8, 10u8, 20u8, 30u8, 0u8].as_slice();
    /// let (_, add) = Add::parse_bytes(input).unwrap();
    /// 
    /// assert_eq!(10u8, add.index1().into());
    /// assert_eq!(20u8, add.index2().into());
    ///assert_eq!(30u8, add.index3().into());
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
        let input = "ADD $10 $20 $30";

        let res = Add::parse_str(input);
        assert!(res.is_ok());

        let add = res.unwrap().1;

        assert_eq!(10u8, add.index1().into());
        assert_eq!(20u8, add.index2().into());
        assert_eq!(30u8, add.index3().into());
    }

    #[test]
    fn parse_hex_str() {
        let input = "ADD $0A $14 $1E";

        let res = Add::parse_hex_str(input);
        assert!(res.is_ok());

        let add = res.unwrap().1;

        assert_eq!(10u8, add.index1().into());
        assert_eq!(20u8, add.index2().into());
        assert_eq!(30u8, add.index3().into());
    }

    #[test]
    fn parse_bytes() {
        let input = [2u8, 10u8, 20u8, 30u8, 0u8].as_slice();

        let res = Add::parse_bytes(input);
        assert!(res.is_ok());

        let (rst, add) = res.unwrap();

        assert_eq!(1, rst.len());
        assert_eq!(10u8, add.index1().into());
        assert_eq!(20u8, add.index2().into());
        assert_eq!(30u8, add.index3().into());
    }
}

use std::{
    fmt::{Debug, Display, LowerHex, UpperHex},
    num::ParseIntError,
};

/// An operand with an `u16` value.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Operand16(u16);

impl Operand16 {
    pub const PREFIX: &str = "#";

    /// Creates a [`Operand16`] instance.
    pub fn make(value: u16) -> Self {
        Self(value)
    }

    /// Returns the internal value.
    pub const fn value(&self) -> u16 {
        self.0
    }

    /// Creates a new instance of [`Operand16`] from a given string representation.
    pub fn try_from_dec(src: &str) -> Result<Self, ParseIntError> {
        src.parse::<u16>().map(Operand16::make)
    }

    /// Creates a new instance of [`Operand16`] from a given string hex representation.
    pub fn try_from_hex(src: &str) -> Result<Self, ParseIntError> {
        u16::from_str_radix(src, 16).map(Operand16::make)
    }
}

/// Used for the regular string representation.
///
/// # Examples
///
/// [`Operand16`] implements `Display`.
///
/// ```
/// use lvm_core::Operand16;
///
/// let oprnd = Operand16::make(10);
/// assert_eq!("#10", oprnd.to_string())
/// ```
impl Display for Operand16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Operand16`] implements `UpperHex`.
///
/// ```
/// use lvm_core::Operand16;
///
/// let oprnd = Operand16::make(10);
/// assert_eq!("000A", format!("{:#X}", oprnd))
/// ```
impl UpperHex for Operand16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04X}", self.0)
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Operand16`] implements `LowerHex`.
///
/// ```
/// use lvm_core::Operand16;
///
/// let oprnd = Operand16::make(10);
/// assert_eq!("000a", format!("{:#x}", oprnd))
/// ```
impl LowerHex for Operand16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04x}", self.0)
    }
}

/// Obtains a [`Operand16`] instance from an u16 value.
///
/// # Examples
///
/// [`Operand16`] implements `From<u16>`.
///
/// ```
/// use lvm_core::Operand16;
///
/// let oprnd = Operand16::from(10u16);
/// assert_eq!(10u16, oprnd.value())
/// ```
impl From<u16> for Operand16 {
    fn from(idx: u16) -> Self {
        Self::make(idx)
    }
}

/// Obtains a [`Operand16`] instance from a slice of u8 values.
///
/// # Examples
///
/// ```
/// use lvm_core::Operand16;
///
/// let input = [1u8, 2u8, 3u8].as_slice();
/// let oprnd = Operand16::from(input);
/// assert_eq!((1u16 << 8) + 2u16, oprnd.value())
/// ```
impl From<&[u8]> for Operand16 {
    fn from(xs: &[u8]) -> Self {
        let value = ((xs[0] as u16) << 8) + (xs[1] as u16);
        Self::make(value)
    }
}

/// Obtains an u18 from a [`Operand16`] value.
///
/// # Examples
///
/// ```
/// use lvm_core::Operand16;
///
/// let oprnd = Operand16::make(10);
/// assert_eq!(10, oprnd.value())
/// ```
impl From<Operand16> for u16 {
    fn from(oprnd: Operand16) -> Self {
        oprnd.0
    }
}

/// Attempts parsing a string into a [`Operand16`] value.
///
/// # Examples
///
/// ```
/// use lvm_core::Operand16;
///
/// let res = Operand16::try_from("10");
/// assert_eq!(10u16, res.unwrap().into())
/// ```
impl TryFrom<&str> for Operand16 {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Operand16::try_from_dec(value)
    }
}

impl From<Operand16> for [u8; 2] {
    fn from(oprnd: Operand16) -> Self {
        [(oprnd.0 >> 8) as u8, oprnd.0 as u8]
    }
}

#[cfg(test)]
mod tests {
    use super::Operand16;

    #[test]
    fn to_string() {
        let oprnd = Operand16::make(10);
        assert_eq!("#10", oprnd.to_string())
    }

    #[test]
    fn to_upper_hex() {
        let oprnd = Operand16::make(10);
        assert_eq!("000A", format!("{:#X}", oprnd))
    }

    #[test]
    fn to_lower_hex() {
        let oprnd = Operand16::make(10);
        assert_eq!("000a", format!("{:#x}", oprnd))
    }

    #[test]
    fn from_bytes() {
        let input = [1u8, 2u8, 3u8].as_slice();
        let oprnd = Operand16::from(input);
        assert_eq!((1u16 << 8) + 2u16, oprnd.value())
    }

    #[test]
    fn from_u16() {
        let oprnd = Operand16::from(10u16);
        assert_eq!(10, oprnd.value());
    }

    #[test]
    fn to_u16() {
        let oprnd = u16::from(Operand16::make(10));
        assert_eq!(10, oprnd)
    }

    #[test]
    fn try_from_string() {
        let res = Operand16::try_from("10");
        assert!(res.is_ok());
        assert_eq!(10u16, res.unwrap().into())
    }

    #[test]
    fn try_from_string_failed() {
        let res = Operand16::try_from("CA");
        assert!(res.is_err());
    }

    #[test]
    fn to_bytes() {
        let oprnd = Operand16::make(500u16);
        let bytes: [u8; 2] = oprnd.into();
        assert_eq!(1u8, bytes[0]);
        assert_eq!(0xF4u8, bytes[1]);
    }
}

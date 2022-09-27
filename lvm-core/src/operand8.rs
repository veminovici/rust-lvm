use std::{
    fmt::{Debug, Display, LowerHex, UpperHex},
    num::ParseIntError,
};

/// An operand with an `u8` value.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Operand8(u8);

impl Operand8 {
    pub const PREFIX: &str = "#";

    /// Creates a [`Operand8`] instance.
    pub fn make(value: u8) -> Self {
        Self(value)
    }

    /// Returns the internal value.
    pub const fn value(&self) -> u8 {
        self.0
    }

    /// Creates a new instance of [`Operand8`] from a given string representation.
    pub fn try_from_dec(src: &str) -> Result<Self, ParseIntError> {
        src.parse::<u8>().map(Operand8::make)
    }

    /// Creates a new instance of [`Operand8`] from a given string hex representation.
    pub fn try_from_hex(src: &str) -> Result<Self, ParseIntError> {
        u8::from_str_radix(src, 16).map(Operand8::make)
    }
}

/// Used for the regular string representation.
///
/// # Examples
///
/// [`Operand8`] implements `Display`.
///
/// ```
/// use lvm_core::Operand8;
///
/// let oprnd = Operand8::make(10);
/// assert_eq!("#10", oprnd.to_string())
/// ```
impl Display for Operand8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", Self::PREFIX, self.0)
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Operand8`] implements `UpperHex`.
///
/// ```
/// use lvm_core::Operand8;
///
/// let oprnd = Operand8::make(10);
/// assert_eq!("0A", format!("{:#X}", oprnd))
/// ```
impl UpperHex for Operand8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}", self.0)
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Operand8`] implements `LowerHex`.
///
/// ```
/// use lvm_core::Operand8;
///
/// let oprnd = Operand8::make(10);
/// assert_eq!("0a", format!("{:#x}", oprnd))
/// ```
impl LowerHex for Operand8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

/// Creates a [`Operand8`] instance from an u8 value.
///
/// # Examples
///
/// [`Operand8`] implements `From<u8>`.
///
/// ```
/// use lvm_core::Operand8;
///
/// let oprnd = Operand8::from(10u8);
/// assert_eq!(10u8, oprnd.value())
/// ```
impl From<u8> for Operand8 {
    fn from(idx: u8) -> Self {
        Self::make(idx)
    }
}

/// Obtains a [`Operand8`] instance from a slice of u8 values.
///
/// # Examples
///
/// ```
/// use lvm_core::Operand8;
///
/// let input = [10u8, 2u8].as_slice();
/// let oprnd = Operand8::from(input);
/// assert_eq!(10, oprnd.value())
/// ```
impl From<&[u8]> for Operand8 {
    fn from(xs: &[u8]) -> Self {
        Self::make(xs[0])
    }
}

/// Obtains an u8 from a [`Operand8`] value.
///
/// # Examples
///
/// ```
/// use lvm_core::Operand8;
///
/// let oprnd = Operand8::make(10);
/// assert_eq!(10, oprnd.value())
/// ```
impl From<Operand8> for u8 {
    fn from(oprnd: Operand8) -> Self {
        oprnd.0
    }
}

/// Attempts parsing a string into a [`Operand8`] value.
///
/// # Examples
///
/// ```
/// use lvm_core::Operand8;
///
/// let res = Operand8::try_from("10");
/// assert_eq!(10u8, res.unwrap().into())
/// ```
impl TryFrom<&str> for Operand8 {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Operand8::try_from_dec(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Operand8;

    #[test]
    fn to_string() {
        let oprnd = Operand8::make(10);
        assert_eq!("#10", oprnd.to_string())
    }

    #[test]
    fn to_upper_hex() {
        let oprnd = Operand8::make(10);
        assert_eq!("0A", format!("{:#X}", oprnd))
    }

    #[test]
    fn to_lower_hex() {
        let oprnd = Operand8::make(10);
        assert_eq!("0a", format!("{:#x}", oprnd))
    }

    #[test]
    fn from_bytes() {
        let input = [1u8, 2u8].as_slice();
        let oprnd = Operand8::from(input);
        assert_eq!(1, oprnd.value())
    }

    #[test]
    fn from_u8() {
        let oprnd = Operand8::from(10u8);
        assert_eq!(10, oprnd.value());
    }

    #[test]
    fn to_u8() {
        let oprnd = u8::from(Operand8::make(10));
        assert_eq!(10, oprnd)
    }

    #[test]
    fn try_from_string() {
        let res = Operand8::try_from("10");
        assert!(res.is_ok());
        assert_eq!(10u8, res.unwrap().into())
    }

    #[test]
    fn try_from_string_failed() {
        let res = Operand8::try_from("CA");
        assert!(res.is_err());
    }
}

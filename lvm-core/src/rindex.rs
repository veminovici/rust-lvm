use std::fmt::{Display, UpperHex, LowerHex};

/// The register index.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct RIndex(u8);

impl RIndex {
    /// Creates a [`RIndex`] instance.
    pub fn make(value: u8) -> Self {
        Self(value)
    }

    /// Returns the internal value.
    pub const fn index(&self) -> u8 {
        self.0
    }
}

/// Used for the regular string representation.
/// 
/// # Examples
/// 
/// [`RIndex`] implements `Display`.
/// 
/// ```
/// use lvm_core::RIndex;
/// 
/// let rindx = RIndex::make(10);
/// assert_eq!("$10", rindx.to_string())
/// ```
impl Display for RIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

/// Used for a hex representation
/// 
/// # Examples
/// 
/// [`RIndex`] implements `UpperHex`.
/// 
/// ```
/// use lvm_core::RIndex;
/// 
/// let rindx = RIndex::make(10);
/// assert_eq!("0A", format!("{:#X}", rindx))
/// ```
impl UpperHex for RIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02X}", self.0)
    }
}

/// Used for a hex representation
/// 
/// # Examples
/// 
/// [`RIndex`] implements `LowerHex`.
/// 
/// ```
/// use lvm_core::RIndex;
/// 
/// let rindx = RIndex::make(10);
/// assert_eq!("0a", format!("{:#x}", rindx))
/// ```
impl LowerHex for RIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}", self.0)
    }
}

/// Obtains a register index from an u8 value
/// 
/// # Examples
/// 
/// [`RIndex`] implements `From<u8>`.
/// 
/// ```
/// use lvm_core::RIndex;
/// 
/// let rindx = RIndex::from(10u8);
/// assert_eq!(10u8, rindx.index())
/// ```
impl From<u8> for RIndex {
    fn from(idx: u8) -> Self {
        RIndex(idx)
    }
}

/// Obtains an u8 from a register index value.
/// 
/// # Examples
/// 
/// ```
/// use lvm_core::RIndex;
/// 
/// let rindx = RIndex::make(10);
/// assert_eq!(10, rindx.index())
/// ```
impl From<RIndex> for u8 {
    fn from(idx: RIndex) -> Self {
        idx.0
    }
}

/// Attempts parsing a string into a [`RIndex`] value.
/// 
impl TryFrom<&str> for RIndex {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse::<u8>().map(RIndex::from)
    }
}

#[cfg(test)]
mod tests {
    use crate::RIndex;

    #[test]
    fn to_string() {
        let rindx = RIndex::make(10);
        assert_eq!("$10", rindx.to_string())
    }

    #[test]
    fn to_upper_hex() {
        let rindx = RIndex::make(10);
        assert_eq!("0A", format!("{:#X}", rindx))
    }

    #[test]
    fn to_lower_hex() {
        let rindx = RIndex::make(10);
        assert_eq!("0a", format!("{:#x}", rindx))
    }

    #[test]
    fn from_u8() {
        let rindx = RIndex::from(10u8);
        assert_eq!(10, rindx.index());
    }

    #[test]
    fn to_u8() {
        let idx = u8::from(RIndex::make(10));
        assert_eq!(10, idx)
    }

    #[test]
    fn try_from_string() {
        let res = RIndex::try_from("10");
        assert!(res.is_ok());
        assert_eq!(10u8, res.unwrap().into())
    }

    #[test]
    fn try_from_string_failed() {
        let res = RIndex::try_from("CA");
        assert!(res.is_err());
    }
}
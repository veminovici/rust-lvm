use std::fmt::{Display, UpperHex, LowerHex};

/// The register index.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct RIndex(u8);

impl RIndex {
    pub fn make(value: u8) -> Self {
        Self(value)
    }
}

/// Used for the regular string representation.
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
}
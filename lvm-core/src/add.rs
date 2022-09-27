use std::fmt::{Debug, Display, LowerHex, UpperHex};

use crate::RIndex;

/// Structure that represents the load instruction.

#[derive(Debug, PartialEq, Eq)]
pub struct Add {
    rindx1: RIndex,
    rindx2: RIndex,
    rindx3: RIndex,
}

impl Add {
    pub const PREFIX: &str = "ADD";
    pub const ID: u8 = 2;

    /// Creates a [`Add`] instance.
    pub fn make(rindx1: RIndex, rindx2: RIndex, rindx3: RIndex) -> Self {
        Self {
            rindx1,
            rindx2,
            rindx3,
        }
    }

    /// Returns the register index.
    pub const fn index1(&self) -> RIndex {
        self.rindx1
    }

    /// Returns the register index.
    pub const fn index2(&self) -> RIndex {
        self.rindx2
    }

    /// Returns the register index.
    pub const fn index3(&self) -> RIndex {
        self.rindx3
    }
}

/// Used for the regular string representation.
///
/// # Examples
///
/// [`Add`] implements `Display`.
///
/// ```
/// use lvm_core::{Add, RIndex};
///
/// let rindx1 = RIndex::make(10u8);
/// let rindx2 = RIndex::make(20u8);
/// let rindx3 = RIndex::make(30u8);
/// let add = Add::make(rindx1, rindx2, rindx3);
/// assert_eq!("ADD $10 $20 $30", add.to_string())
/// ```
impl Display for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            Self::PREFIX,
            self.rindx1,
            self.rindx2,
            self.rindx3
        )
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Add`] implements `UpperHex`.
///
/// ```
/// use lvm_core::{Add, RIndex};
///
/// let rindx1 = RIndex::make(10u8);
/// let rindx2 = RIndex::make(20u8);
/// let rindx3 = RIndex::make(30u8);
/// let add = Add::make(rindx1, rindx2, rindx3);
/// assert_eq!("ADD 0A 14 1E", format!("{:#X}", add))
/// ```
impl UpperHex for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:X} {:X} {:X}",
            Self::PREFIX,
            self.rindx1,
            self.rindx2,
            self.rindx3
        )
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Add`] implements `LowerHex`.
///
/// ```
/// use lvm_core::{Add, RIndex};
///
/// let rindx1 = RIndex::make(10u8);
/// let rindx2 = RIndex::make(20u8);
/// let rindx3 = RIndex::make(30u8);
/// let add = Add::make(rindx1, rindx2, rindx3);
/// assert_eq!("ADD 0a 14 1e", format!("{:#x}", add))
/// ```
impl LowerHex for Add {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:x} {:x} {:x}",
            Self::PREFIX,
            self.rindx1,
            self.rindx2,
            self.rindx3
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let rindx1 = RIndex::make(10u8);
        let rindx2 = RIndex::make(20u8);
        let rindx3 = RIndex::make(30u8);
        let add = Add::make(rindx1, rindx2, rindx3);
        assert_eq!("ADD $10 $20 $30", add.to_string())
    }

    #[test]
    fn to_upper_hex() {
        let rindx1 = RIndex::make(10u8);
        let rindx2 = RIndex::make(20u8);
        let rindx3 = RIndex::make(30u8);
        let add = Add::make(rindx1, rindx2, rindx3);
        assert_eq!("ADD 0A 14 1E", format!("{:#X}", add))
    }

    #[test]
    fn to_lower_hex() {
        let rindx1 = RIndex::make(10u8);
        let rindx2 = RIndex::make(20u8);
        let rindx3 = RIndex::make(30u8);
        let add = Add::make(rindx1, rindx2, rindx3);
        assert_eq!("ADD 0a 14 1e", format!("{:#x}", add))
    }
}

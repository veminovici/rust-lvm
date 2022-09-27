use std::fmt::{Debug, Display, LowerHex, UpperHex};

use crate::{Operand16, RIndex};

/// Structure that represents the load instruction.

#[derive(Debug, PartialEq, Eq)]
pub struct Load {
    rindx: RIndex,
    oprnd: Operand16,
}

impl Load {
    pub const PREFIX: &str = "LOAD";
    pub const ID: u8 = 1;

    /// Creates a [`Load`] instance.
    pub fn make(rindx: RIndex, oprnd: Operand16) -> Self {
        Self { rindx, oprnd }
    }

    /// Returns the register index.
    pub const fn index(&self) -> RIndex {
        self.rindx
    }

    /// Returns the operand.
    pub const fn operand(&self) -> Operand16 {
        self.oprnd
    }
}

/// Used for the regular string representation.
///
/// # Examples
///
/// [`Load`] implements `Display`.
///
/// ```
/// use lvm_core::{Load, Operand16, RIndex};
///
/// let rindx = RIndex::make(10u8);
/// let oprnd = Operand16::make(500u16);
/// let load = Load::make(rindx, oprnd);
/// assert_eq!("LOAD $10 #500", load.to_string())
/// ```
impl Display for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", Self::PREFIX, self.rindx, self.oprnd)
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Load`] implements `UpperHex`.
///
/// ```
/// use lvm_core::{Load, Operand16, RIndex};
///
/// let rindx = RIndex::make(10u8);
/// let oprnd = Operand16::make(500u16);
/// let load = Load::make(rindx, oprnd);
/// assert_eq!("LOAD 0A 01F4", format!("{:#X}", load))
/// ```
impl UpperHex for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:X} {:X}", Self::PREFIX, self.rindx, self.oprnd)
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Load`] implements `LowerHex`.
///
/// ```
/// use lvm_core::{Load, Operand16, RIndex};
///
/// let rindx = RIndex::make(10u8);
/// let oprnd = Operand16::make(500u16);
/// let load = Load::make(rindx, oprnd);
/// assert_eq!("LOAD 0a 01f4", format!("{:#x}", load))
/// ```
impl LowerHex for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:x} {:x}", Self::PREFIX, self.rindx, self.oprnd)
    }
}

impl From<Load> for [u8; 4] {
    fn from(load: Load) -> Self {
        let oprnd: [u8; 2] = load.operand().into();
        [Load::ID, load.index().into(), oprnd[0], oprnd[1]]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        assert_eq!("LOAD $10 #500", load.to_string())
    }

    #[test]
    fn to_upper_hex() {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        assert_eq!("LOAD 0A 01F4", format!("{:#X}", load))
    }

    #[test]
    fn to_lower_hex() {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        assert_eq!("LOAD 0a 01f4", format!("{:#x}", load))
    }

    #[test]
    fn to_bytes() {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        let bytes: [u8; 4] = load.into();

        assert_eq!(1, bytes[0]);
        assert_eq!(10, bytes[1]);
        assert_eq!(1, bytes[2]);
        assert_eq!(0xF4u8, bytes[3]);
    }
}

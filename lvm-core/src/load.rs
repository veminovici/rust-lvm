use std::fmt::{Debug, Display, LowerHex, UpperHex};

use crate::{Operand16, RIndex};

/// Structure that represents the load instruction.

#[derive(Debug)]
pub struct Load {
    rindx: RIndex,
    oprnd: Operand16,
}

impl Load {
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
        write!(f, "LOAD {} {}", self.rindx, self.oprnd)
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
        write!(f, "LOAD {:X} {:X}", self.rindx, self.oprnd)
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
        write!(f, "LOAD {:x} {:x}", self.rindx, self.oprnd)
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
}

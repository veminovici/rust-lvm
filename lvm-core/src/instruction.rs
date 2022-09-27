use std::fmt::{Debug, Display, LowerHex, UpperHex};

use crate::{Add, Load};

#[derive(Debug)]
pub enum Instruction {
    LoadI(Load),
    AddI(Add),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadI(load) => write!(f, "{}", load),
            Self::AddI(add) => write!(f, "{}", add),
        }
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Instruction`] implements `UpperHex`.
///
/// ```
/// use lvm_core::{Instruction, Load, Operand16, RIndex};
///
/// let rindx = RIndex::make(10u8);
/// let oprnd = Operand16::make(500u16);
/// let load = Load::make(rindx, oprnd);
/// let instruction = Instruction::LoadI(load);
/// assert_eq!("LOAD 0A 01F4", format!("{:#X}", instruction))
/// ```
impl UpperHex for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadI(load) => write!(f, "{:X}", load),
            Self::AddI(add) => write!(f, "{:X}", add),
        }
    }
}

/// Used for a hex representation
///
/// # Examples
///
/// [`Instruction`] implements `UpperHex`.
///
/// ```
/// use lvm_core::{Instruction, Load, Operand16, RIndex};
///
/// let rindx = RIndex::make(10u8);
/// let oprnd = Operand16::make(500u16);
/// let load = Load::make(rindx, oprnd);
/// let instruction = Instruction::LoadI(load);
/// assert_eq!("LOAD 0a 01f4", format!("{:#x}", instruction))
/// ```
impl LowerHex for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadI(load) => write!(f, "{:x}", load),
            Self::AddI(add) => write!(f, "{:x}", add),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{RIndex, Operand16};

    use super::*;

    #[test]
    fn to_string() {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        let instruction = Instruction::LoadI(load);
        assert_eq!("LOAD $10 #500", instruction.to_string());

        let rindx1 = RIndex::make(10u8);
        let rindx2 = RIndex::make(20u8);
        let rindx3 = RIndex::make(30u8);
        let add = Add::make(rindx1, rindx2, rindx3);
        assert_eq!("ADD $10 $20 $30", add.to_string())
    }

    #[test]
    fn to_upper_hex() {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        let instruction = Instruction::LoadI(load);
        assert_eq!("LOAD 0A 01F4", format!("{:#X}", instruction));

        let rindx1 = RIndex::make(10u8);
        let rindx2 = RIndex::make(20u8);
        let rindx3 = RIndex::make(30u8);
        let add = Add::make(rindx1, rindx2, rindx3);
        assert_eq!("ADD 0A 14 1E", format!("{:#X}", add))
    }

    #[test]
    fn to_lower_hex() {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        let instruction = Instruction::LoadI(load);
        assert_eq!("LOAD 0a 01f4", format!("{:#x}", instruction));

        let rindx1 = RIndex::make(10u8);
        let rindx2 = RIndex::make(20u8);
        let rindx3 = RIndex::make(30u8);
        let add = Add::make(rindx1, rindx2, rindx3);
        assert_eq!("ADD 0a 14 1e", format!("{:#x}", add))
    }
}

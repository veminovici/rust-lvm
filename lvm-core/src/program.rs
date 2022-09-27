use std::fmt::{Debug, Display, LowerHex, UpperHex};

use crate::Instruction;

#[derive(Debug)]
pub struct Program(Vec<Instruction>);

impl Program {
    pub fn make(xs: Vec<Instruction>) -> Self {
        Self(xs)
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|i| format!("{}", i))
            .enumerate()
            .for_each(|(i, s)| {
                if i != 0 {
                    writeln!(f).unwrap();
                }

                write!(f, "{}", s).unwrap();
            });
        write!(f, "")
    }
}

impl UpperHex for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|i| format!("{:X}", i))
            .enumerate()
            .for_each(|(i, s)| {
                if i != 0 {
                    writeln!(f).unwrap();
                }

                write!(f, "{}", s).unwrap();
            });
        write!(f, "")
    }
}

impl LowerHex for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .map(|i| format!("{:x}", i))
            .enumerate()
            .for_each(|(i, s)| {
                if i != 0 {
                    writeln!(f).unwrap();
                }

                write!(f, "{}", s).unwrap();
            });
        write!(f, "")
    }
}

impl IntoIterator for Program {
    type Item = Instruction;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<Program> for Vec<u8> {
    fn from(program: Program) -> Self {
        program.0.into_iter().fold(vec![], |acc, i| {
            let bytes: [u8; 4] = i.into();
            let mut acc1 = acc;
            acc1.extend_from_slice(bytes.as_slice());
            acc1
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{Add, Load, Operand16, RIndex};

    fn create_load() -> Instruction {
        let rindx = RIndex::make(10u8);
        let oprnd = Operand16::make(500u16);
        let load = Load::make(rindx, oprnd);
        Instruction::LoadI(load)
    }

    fn create_add() -> Instruction {
        let rindx1 = RIndex::make(10u8);
        let rindx2 = RIndex::make(20u8);
        let rindx3 = RIndex::make(30u8);
        let add = Add::make(rindx1, rindx2, rindx3);
        Instruction::AddI(add)
    }

    #[test]
    fn to_string() {
        let i1 = create_load();
        let i2 = create_add();
        let program = Program::make(vec![i1, i2]);

        let s = program.to_string();
        let e = "LOAD $10 #500\nADD $10 $20 $30";

        assert_eq!(e, s)
    }

    #[test]
    fn to_upper_hex() {
        let i1 = create_load();
        let i2 = create_add();
        let program = Program::make(vec![i1, i2]);

        let s = format!("{:#X}", program);
        let e = "LOAD 0A 01F4\nADD 0A 14 1E";

        assert_eq!(e, s)
    }

    #[test]
    fn to_lower_hex() {
        let i1 = create_load();
        let i2 = create_add();
        let program = Program::make(vec![i1, i2]);

        let s = format!("{:#x}", program);
        let e = "LOAD 0a 01f4\nADD 0a 14 1e";

        assert_eq!(e, s)
    }

    #[test]
    fn to_bytes() {
        let i1 = create_load();
        let i2 = create_add();
        let program = Program::make(vec![i1, i2]);
        let bytes: Vec<u8> = program.into();

        assert_eq!(8, bytes.len());
        assert_eq!(1, bytes[0]);
        assert_eq!(10, bytes[1]);
        assert_eq!(1, bytes[2]);
        assert_eq!(0xF4u8, bytes[3]);
        assert_eq!(2, bytes[4]);
        assert_eq!(10, bytes[5]);
        assert_eq!(20, bytes[6]);
        assert_eq!(30, bytes[7]);
    }
}

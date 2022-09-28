use lvm_core::Instruction;
use lvm_parser::ParseBytes;

pub struct PBytes(Vec<u8>);

impl PBytes {
    pub fn make(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn iter(&self) -> PBytesIter {
        PBytesIter {
            bytes: &self.0,
            cur: 0,
        }
    }
}

pub struct PBytesIter<'a> {
    bytes: &'a Vec<u8>,
    cur: usize,
}

impl<'a> Iterator for PBytesIter<'a> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur > self.bytes.len() {
            return None;
        }

        let res = Instruction::parse_bytes(&self.bytes[self.cur..])
            .map(|(_, i)| i)
            .ok();

        self.cur += 4;

        res
    }
}

#[cfg(test)]
mod tests {
    use lvm_core::{Add, Load, Operand16, RIndex};

    use super::*;

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
    fn iter_pbytes() {
        let input = vec![1u8, 10u8, 1u8, 0xF4u8, 2u8, 10u8, 20u8, 30u8];
        let pbytes = PBytes::make(input);
        let mut iter = pbytes.iter();

        let load = create_load();
        assert_eq!(iter.next(), Some(load));

        let add = create_add();
        assert_eq!(iter.next(), Some(add));

        assert!(iter.next().is_none());
    }
}

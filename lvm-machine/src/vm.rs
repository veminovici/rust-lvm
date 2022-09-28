use lvm_core::{Add, Load};

pub struct VM {
    registers: [u16; 32],
}

impl VM {
    pub fn new() -> Self {
        Self { registers: [0; 32] }
    }

    pub fn run_load(&mut self, load: Load) {
        let idx: u8 = load.index().into();
        let value: u16 = load.operand().into();
        self.registers[idx as usize] = value;
    }

    pub fn run_add(&mut self, add: Add) {
        let a: u8 = add.index1().into();
        let b: u8 = add.index2().into();
        let c: u8 = add.index3().into();

        let ttl = self.registers[a as usize] + self.registers[b as usize];
        self.registers[c as usize] = ttl;
    }
}

#[cfg(test)]
mod tests {
    use lvm_core::{Operand16, RIndex};

    use super::*;

    fn make_load(indx: u8, value: u16) -> Load {
        let rindx = RIndex::make(indx);
        let oprnd = Operand16::make(value);
        Load::make(rindx, oprnd)
    }

    fn make_add(i1: u8, i2: u8, i3: u8) -> Add {
        let rindx1 = RIndex::make(i1);
        let rindx2 = RIndex::make(i2);
        let rindx3 = RIndex::make(i3);
        Add::make(rindx1, rindx2, rindx3)
    }

    #[test]
    fn load() {
        let mut vm = VM::new();

        vm.run_load(make_load(1, 200));
        assert_eq!(200, vm.registers[1]);

        vm.run_load(make_load(2, 300));
        assert_eq!(300, vm.registers[2]);

        vm.run_add(make_add(1, 2, 10));
        assert_eq!(200, vm.registers[1]);
        assert_eq!(300, vm.registers[2]);
        assert_eq!(500, vm.registers[10]);
    }
}

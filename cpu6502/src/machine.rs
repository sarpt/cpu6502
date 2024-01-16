use crate::memory::VecMemory;

use super::cpu::CPU;

pub struct Machine {
    cpu: CPU,
}

impl Machine {
    pub fn new() -> Self {
        let mut cpu = CPU::new(Box::new(VecMemory::new()));
        cpu.reset();
        return Machine { cpu };
    }

    pub fn execute_cycles(&mut self, program: &[(u16, u8)], cycles: u64) {
        self.cpu.set_memory(Box::new(VecMemory::from(program)));
        self.cpu.execute(cycles);
    }
}

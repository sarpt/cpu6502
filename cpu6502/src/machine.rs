use crate::memory::VecMemory;

use super::cpu::CPU;

pub struct Machine {
    cpu: CPU,
}

impl Machine {
    pub fn new() -> Self {
        return Machine {
            cpu: CPU::new(Box::new(VecMemory::new())),
        };
    }

    pub fn execute_until_break(&mut self, program: &[(u16, u8)]) {
        self.cpu.set_memory(Box::new(VecMemory::from(program)));
        self.cpu.reset();
        self.cpu.execute_until_break();
    }
}

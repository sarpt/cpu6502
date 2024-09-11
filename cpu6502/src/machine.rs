use std::cell::RefCell;

use crate::memory::VecMemory;

use super::cpu::CPU;

pub struct Machine<'a> {
    memory: &'a RefCell<VecMemory>,
    cpu: CPU<'a>,
}

impl<'a> Machine<'a> {
    pub fn new(memory: &'a RefCell<VecMemory>) -> Self {
        return Machine {
            memory: memory,
            cpu: CPU::new_nmos(memory),
        };
    }

    pub fn execute_until_break(&mut self, program: &[(u16, u8)]) {
        self.memory.borrow_mut().store(program);
        self.cpu.reset();
        self.cpu.execute_until_break();
    }
}

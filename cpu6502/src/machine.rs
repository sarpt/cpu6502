use std::{cell::RefCell, rc::Rc};

use crate::memory::VecMemory;

use super::cpu::CPU;

pub struct Machine {
    cpu: CPU,
}

impl Machine {
    pub fn new(program: &[(u16, u8)]) -> Self {
        return Machine {
            cpu: CPU::new(Rc::new(RefCell::new(VecMemory::from(program)))),
        };
    }

    pub fn execute_until_break(&mut self) {
        self.cpu.reset();
        self.cpu.execute_until_break();
    }
}

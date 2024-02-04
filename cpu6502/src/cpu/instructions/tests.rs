#[cfg(test)]
mod nop {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::nop, tests::MemoryMock, CPU};

    #[test]
    fn should_increment_program_counter() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.program_counter = 0x05;

        nop(&mut cpu);

        assert_eq!(cpu.program_counter, 0x06);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.program_counter = 0x05;
        cpu.cycle = 0;

        nop(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

mod branch;
mod compare;
mod decrement;
mod flags;
mod increment;
mod jump;
mod load;
mod logic;
mod store;

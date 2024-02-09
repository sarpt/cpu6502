#[cfg(test)]
mod brk {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        consts::Byte,
        cpu::{instructions::brk, tests::MemoryMock, CPU},
    };

    #[test]
    fn should_put_program_counter_incremented_by_one_and_processor_status_on_stack() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.processor_status.set(0b11111111);
        cpu.stack_pointer = 0xFF;
        cpu.program_counter = 0xABCD;

        brk(&mut cpu);

        assert_eq!(cpu.memory.borrow()[0x01FF], 0xCE);
        assert_eq!(cpu.memory.borrow()[0x01FE], 0xAB);
        assert_eq!(cpu.memory.borrow()[0x01FD], 0b11111111);
    }

    #[test]
    fn should_jump_to_address_stored_in_brk_vector() {
        const ADDR_LO: Byte = 0xAD;
        const ADDR_HI: Byte = 0x9B;
        let mut memory = MemoryMock::default();
        memory[0xFFFE] = ADDR_LO;
        memory[0xFFFF] = ADDR_HI;

        let mut cpu = CPU::new(Rc::new(RefCell::new(memory)));
        cpu.program_counter = 0x00;

        brk(&mut cpu);

        assert_eq!(cpu.program_counter, 0x9BAD);
    }

    #[test]
    fn should_set_break_processor_status_flag() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.program_counter = 0x00;
        cpu.processor_status.change_break_flag(false);

        brk(&mut cpu);

        assert_eq!(cpu.processor_status.get_break_flag(), true);
    }

    #[test]
    fn should_take_six_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::default())));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        brk(&mut cpu);

        assert_eq!(cpu.cycle, 6);
    }
}

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

#[cfg(test)]
mod jsr_a {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::jsr_a, tests::MemoryMock, CPU};

    #[test]
    fn should_fetch_address_pointed_by_program_counter_and_put_in_program_counter() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]))));
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        jsr_a(&mut cpu);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_save_program_counter_shifted_once_into_stack_pointer() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]))));
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        jsr_a(&mut cpu);

        assert_eq!(cpu.memory.borrow()[0x01FF], 0x01);
        assert_eq!(cpu.memory.borrow()[0x01FE], 0x00);
    }

    #[test]
    fn should_decrement_stack_pointer_twice() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]))));
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        jsr_a(&mut cpu);

        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn should_take_five_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]))));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jsr_a(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod rts {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::rts, tests::MemoryMock, CPU};

    #[test]
    fn should_fetch_address_from_stack_and_put_it_in_program_counter_incremented_by_one() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]))));
        cpu.program_counter = 0x00;
        cpu.memory.borrow_mut()[0x01FF] = 0x44;
        cpu.memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;

        rts(&mut cpu);

        assert_eq!(cpu.program_counter, 0x4452);
    }

    #[test]
    fn should_increment_stack_pointer_twice() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]))));
        cpu.program_counter = 0x00;
        cpu.memory.borrow_mut()[0x01FF] = 0x44;
        cpu.memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;

        rts(&mut cpu);

        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn should_take_five_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]))));
        cpu.program_counter = 0x00;
        cpu.memory.borrow_mut()[0x01FF] = 0x44;
        cpu.memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;
        cpu.cycle = 0;

        rts(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod jmp_a {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::jmp_a, tests::MemoryMock, CPU};

    #[test]
    fn should_put_address_stored_in_memory_at_program_counter_as_a_new_program_counter() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]))));
        cpu.program_counter = 0x00;

        jmp_a(&mut cpu);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]))));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jmp_a(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod jmp_in {
    use std::{cell::RefCell, rc::Rc};

    use crate::cpu::{instructions::jmp_in, tests::MemoryMock, CPU};

    #[test]
    fn should_fetch_indirect_address_from_memory_and_put_in_program_counter() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x02, 0x00, 0x01, 0x00,
        ]))));
        cpu.program_counter = 0x00;

        jmp_in(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_four_cycles() {
        let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
            0x02, 0x00, 0x01, 0x00,
        ]))));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jmp_in(&mut cpu);

        assert_eq!(cpu.cycle, 4);
    }
}

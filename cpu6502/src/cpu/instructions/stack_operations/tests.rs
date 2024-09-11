#[cfg(test)]
mod pha {
    use std::cell::RefCell;

    use crate::cpu::{instructions::pha, tests::MemoryMock, CPU};

    #[test]
    fn should_push_accumulator_into_stack() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFF;
        cpu.accumulator = 0xDE;

        pha(&mut cpu);

        assert_eq!(memory.borrow()[0x01FF], 0xDE);
    }

    #[test]
    fn should_take_two_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.accumulator = 0xDE;
        cpu.stack_pointer = 0xFF;
        cpu.cycle = 0;

        pha(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod pla {
    use std::cell::RefCell;

    use crate::cpu::{instructions::pla, tests::MemoryMock, CPU};

    #[test]
    fn should_pull_stack_into_accumulator() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.accumulator = 0x00;

        pla(&mut cpu);

        assert_eq!(cpu.accumulator, 0xDE);
    }

    #[test]
    fn should_take_three_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.cycle = 0;

        pla(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }

    #[test]
    fn should_set_processor_status_based_on_accumulator_value() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        pla(&mut cpu);

        assert_eq!(cpu.processor_status, 0b10000000);
    }
}

#[cfg(test)]
mod php {
    use std::cell::RefCell;

    use crate::cpu::{instructions::php, tests::MemoryMock, CPU};

    #[test]
    fn should_push_processor_status_into_stack() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status = (0b10101010 as u8).into();
        cpu.stack_pointer = 0xFF;

        php(&mut cpu);

        assert_eq!(memory.borrow()[0x01FF], 0b10101010);
    }

    #[test]
    fn should_take_two_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status = (0b10101010 as u8).into();
        cpu.stack_pointer = 0xFF;
        cpu.cycle = 0;

        php(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod plp {
    use std::cell::RefCell;

    use crate::cpu::{instructions::plp, tests::MemoryMock, CPU};

    #[test]
    fn should_pull_stack_into_accumulator() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        plp(&mut cpu);

        assert_eq!(cpu.processor_status, 0xDE);
    }

    #[test]
    fn should_take_three_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xFE;
        memory.borrow_mut()[0x01FF] = 0xDE;
        cpu.processor_status = (0x00 as u8).into();
        cpu.cycle = 0;

        plp(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod txs {
    use std::cell::RefCell;

    use crate::cpu::{instructions::txs, tests::MemoryMock, CPU};

    #[test]
    fn should_push_index_x_register_into_stack_pointer_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0xDE;

        txs(&mut cpu);

        assert_eq!(cpu.stack_pointer, 0xDE);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.index_register_x = 0xDE;
        cpu.cycle = 0;

        txs(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

#[cfg(test)]
mod tsx {
    use std::cell::RefCell;

    use crate::cpu::{instructions::tsx, tests::MemoryMock, CPU};

    #[test]
    fn should_push_stack_pointer_into_index_x_register_register() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xDE;

        tsx(&mut cpu);

        assert_eq!(cpu.index_register_x, 0xDE);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xDE;
        cpu.cycle = 0;

        tsx(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_set_processor_status_based_on_index_x_register_value() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.stack_pointer = 0xDE;
        cpu.processor_status = (0x00 as u8).into();

        tsx(&mut cpu);

        assert_eq!(cpu.processor_status, 0b10000000);
    }
}

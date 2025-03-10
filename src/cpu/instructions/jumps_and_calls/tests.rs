#[cfg(test)]
mod jsr_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::jsr_a,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_fetch_address_pointed_by_program_counter_and_put_in_program_counter() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        let tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_save_program_counter_shifted_once_into_stack_pointer() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        let tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(memory.borrow()[0x01FF], 0x00);
        assert_eq!(memory.borrow()[0x01FE], 0x01);
    }

    #[test]
    fn should_decrement_stack_pointer_twice() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        let tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let tasks = jsr_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod rts {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::rts,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_fetch_address_from_stack_and_put_it_in_program_counter_incremented_by_one() {
        let memory = &RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        memory.borrow_mut()[0x01FF] = 0x44;
        memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;

        let tasks = rts(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.program_counter, 0x4452);
    }

    #[test]
    fn should_increment_stack_pointer_twice() {
        let memory = &RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        memory.borrow_mut()[0x01FF] = 0x44;
        memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;

        let tasks = rts(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[0x01, 0x02, 0x03]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        memory.borrow_mut()[0x01FF] = 0x44;
        memory.borrow_mut()[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;
        cpu.cycle = 0;

        let tasks = rts(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod jmp_a {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::jmp_a,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_put_address_stored_in_memory_at_program_counter_as_a_new_program_counter() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;

        let tasks = jmp_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_take_two_cycles() {
        let memory = &RefCell::new(MemoryMock::new(&[0x44, 0x51, 0x88]));
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let tasks = jmp_a(&mut cpu);
        run_tasks(&mut cpu, tasks);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod jmp_in {
    #[cfg(test)]
    mod common {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::jmp_in,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_fetch_indirect_address_from_memory_and_put_in_program_counter() {
            let memory = &RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;

            let tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x0001);
        }
    }

    #[cfg(test)]
    mod nmos {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::jmp_in,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_take_four_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod cmos {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::jmp_in,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_take_five_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_rockwell_cmos(memory);
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let tasks = jmp_in(&mut cpu);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 5);
        }
    }
}

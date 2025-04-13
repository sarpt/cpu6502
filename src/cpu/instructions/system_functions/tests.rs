#[cfg(test)]
mod brk {
    #[cfg(test)]
    mod common {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{
                instructions::brk,
                tests::{run_tasks, MemoryMock},
                CPU,
            },
        };

        #[test]
        fn should_put_program_counter_incremented_by_one_and_processor_status_on_stack() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new_nmos(memory);
            cpu.processor_status.set(0b11111111);
            cpu.stack_pointer = 0xFF;
            cpu.program_counter = 0xABCD;

            let mut tasks = brk(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks);

            assert_eq!(memory.borrow()[0x01FF], 0xAB);
            assert_eq!(memory.borrow()[0x01FE], 0xCE);
            assert_eq!(memory.borrow()[0x01FD], 0b11111111);
        }

        #[test]
        fn should_jump_to_address_stored_in_brk_vector() {
            const ADDR_LO: Byte = 0xAD;
            const ADDR_HI: Byte = 0x9B;
            let memory = &RefCell::new(MemoryMock::default());
            memory.borrow_mut()[0xFFFE] = ADDR_LO;
            memory.borrow_mut()[0xFFFF] = ADDR_HI;

            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;

            let mut tasks = brk(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks);

            assert_eq!(cpu.program_counter, 0x9BAD);
        }

        #[test]
        fn should_set_break_processor_status_flag() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.processor_status.change_break_flag(false);

            let mut tasks = brk(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks);

            assert_eq!(cpu.processor_status.get_break_flag(), true);
        }

        #[test]
        fn should_take_six_cycles() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let mut tasks = brk(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks);

            assert_eq!(cpu.cycle, 6);
        }
    }

    #[cfg(test)]
    mod cmos {
        use std::cell::RefCell;

        use crate::cpu::{instructions::brk, tests::MemoryMock, CPU};

        #[cfg(test)]
        mod rockwell {
            use crate::cpu::tests::run_tasks;

            use super::*;

            #[test]
            fn should_clear_decimal_processor_status_flag() {
                let memory = &RefCell::new(MemoryMock::default());
                let mut cpu = CPU::new_rockwell_cmos(memory);
                cpu.program_counter = 0x00;
                cpu.processor_status.change_decimal_mode_flag(true);

                let mut tasks = brk(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks);

                assert_eq!(cpu.processor_status.get_decimal_mode_flag(), false);
            }
        }

        #[cfg(test)]
        mod wdc {
            use crate::cpu::tests::run_tasks;

            use super::*;

            #[test]
            fn should_clear_decimal_processor_status_flag() {
                let memory = &RefCell::new(MemoryMock::default());
                let mut cpu = CPU::new_wdc_cmos(memory);
                cpu.program_counter = 0x00;
                cpu.processor_status.change_decimal_mode_flag(true);

                let mut tasks = brk(&mut cpu);
                run_tasks(&mut cpu, &mut *tasks);

                assert_eq!(cpu.processor_status.get_decimal_mode_flag(), false);
            }
        }
    }

    #[cfg(test)]
    mod nmos {
        use std::cell::RefCell;

        use crate::cpu::{
            instructions::brk,
            tests::{run_tasks, MemoryMock},
            CPU,
        };

        #[test]
        fn should_not_clear_decimal_processor_status_flag() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.processor_status.change_decimal_mode_flag(true);

            let mut tasks = brk(&mut cpu);
            run_tasks(&mut cpu, &mut *tasks);

            assert_eq!(cpu.processor_status.get_decimal_mode_flag(), true);
        }
    }
}

#[cfg(test)]
mod rti {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::rti,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_pop_processor_status_and_program_counter_from_stack() {
        let memory = &RefCell::new(MemoryMock::default());
        memory.borrow_mut()[0x01FF] = 0xAB;
        memory.borrow_mut()[0x01FE] = 0xCD;
        memory.borrow_mut()[0x01FD] = 0b11111111;

        let mut cpu = CPU::new_nmos(memory);
        cpu.processor_status.set(0b00000000);
        cpu.stack_pointer = 0xFC;
        cpu.program_counter = 0x00;

        let mut tasks = rti(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.processor_status, 0b11111111);
        assert_eq!(cpu.program_counter, 0xABCD);
    }

    #[test]
    fn should_take_five_cycles() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        let mut tasks = rti(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod nop {
    use std::cell::RefCell;

    use crate::cpu::{
        instructions::nop,
        tests::{run_tasks, MemoryMock},
        CPU,
    };

    #[test]
    fn should_increment_program_counter() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x05;

        let mut tasks = nop(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.program_counter, 0x06);
    }

    #[test]
    fn should_take_one_cycle() {
        let memory = &RefCell::new(MemoryMock::default());
        let mut cpu = CPU::new_nmos(memory);
        cpu.program_counter = 0x05;
        cpu.cycle = 0;

        let mut tasks = nop(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks);

        assert_eq!(cpu.cycle, 1);
    }
}

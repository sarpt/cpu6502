#[cfg(test)]
mod asl {
    #[cfg(test)]
    mod common {
        use std::cell::RefCell;

        use crate::cpu::{instructions::shifts::asl_acc, tests::MemoryMock, Registers, CPU};

        #[test]
        fn should_set_carry_when_bit_7_is_set() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new(memory);
            cpu.accumulator = 0b10000000;

            assert_eq!(cpu.processor_status.get_carry_flag(), false);

            asl_acc(&mut cpu);

            assert_eq!(cpu.processor_status.get_carry_flag(), true);
        }

        #[test]
        fn should_not_change_carry_when_bit_7_is_not_set() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new(memory);
            cpu.accumulator = 0b01111111;

            assert_eq!(cpu.processor_status.get_carry_flag(), false);

            asl_acc(&mut cpu);

            assert_eq!(cpu.processor_status.get_carry_flag(), false);
        }

        #[test]
        fn should_set_zero_when_value_after_shift_is_zero() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new(memory);
            cpu.accumulator = 0b10000000;

            assert_eq!(cpu.processor_status.get_zero_flag(), false);

            asl_acc(&mut cpu);

            assert_eq!(cpu.processor_status.get_zero_flag(), true);
        }

        #[test]
        fn should_set_negative_when_value_after_shift_is_negative() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new(memory);
            cpu.accumulator = 0b01000000;

            assert_eq!(cpu.processor_status.get_negative_flag(), false);

            asl_acc(&mut cpu);

            assert_eq!(cpu.processor_status.get_negative_flag(), true);
        }
    }

    #[cfg(test)]
    mod asl_acc {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{instructions::shifts::asl_acc, tests::MemoryMock, CPU},
        };
        const VALUE: Byte = 0x02;

        #[test]
        fn should_shift_left_value_in_accumulator() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new(memory);
            cpu.accumulator = VALUE;
            cpu.program_counter = 0x00;

            asl_acc(&mut cpu);

            assert_eq!(cpu.accumulator, 0x04);
        }

        #[test]
        fn should_take_one_cycle() {
            let memory = &RefCell::new(MemoryMock::default());
            let mut cpu = CPU::new(memory);
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            asl_acc(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod asl_zp {
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{instructions::shifts::asl_zp, tests::MemoryMock, CPU},
        };

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const VALUE: Byte = 0x02;

        #[test]
        fn should_shift_left_value_in_memory_at_zero_page() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]));
            let mut cpu = CPU::new(memory);
            cpu.program_counter = 0x00;

            asl_zp(&mut cpu);

            assert_eq!(memory.borrow()[ZERO_PAGE_ADDR as Word], 0x04);
        }

        #[test]
        fn should_take_four_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, VALUE]));
            let mut cpu = CPU::new(memory);
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            asl_zp(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod asl_zpx {
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{instructions::shifts::asl_zpx, tests::MemoryMock, CPU},
        };

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const OFFSET: Byte = 0x01;
        const VALUE: Byte = 0x02;

        #[test]
        fn should_shift_left_value_in_memory_at_zero_page_summed_with_index_register_x() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]));
            let mut cpu = CPU::new(memory);
            cpu.index_register_x = OFFSET;
            cpu.program_counter = 0x00;

            asl_zpx(&mut cpu);

            assert_eq!(memory.borrow()[(ZERO_PAGE_ADDR + OFFSET) as Word], 0x04);
        }

        #[test]
        fn should_take_five_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0x00, VALUE]));
            let mut cpu = CPU::new(memory);
            cpu.index_register_x = OFFSET;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            asl_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod asl_a {
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{instructions::shifts::asl_a, tests::MemoryMock, CPU},
        };

        const ABSOLUTE_ADDR_HI: Byte = 0x00;
        const ABSOLUTE_ADDR_LO: Byte = 0x03;
        const VALUE: Byte = 0x02;

        #[test]
        fn should_shift_left_value_in_memory_at_absolute_address() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ABSOLUTE_ADDR_LO,
                ABSOLUTE_ADDR_HI,
                0x00,
                VALUE,
            ]));
            let mut cpu = CPU::new(memory);
            cpu.program_counter = 0x00;

            asl_a(&mut cpu);

            assert_eq!(memory.borrow()[ABSOLUTE_ADDR_LO as Word], 0x04);
        }

        #[test]
        fn should_take_five_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ABSOLUTE_ADDR_LO,
                ABSOLUTE_ADDR_HI,
                0x00,
                VALUE,
            ]));
            let mut cpu = CPU::new(memory);
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            asl_a(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod asl_ax {
        use std::cell::RefCell;

        use crate::{
            consts::{Byte, Word},
            cpu::{instructions::shifts::asl_ax, tests::MemoryMock, CPU},
        };

        const ABSOLUTE_ADDR_HI: Byte = 0x00;
        const ABSOLUTE_ADDR_LO: Byte = 0x03;
        const OFFSET: Byte = 0x01;
        const VALUE: Byte = 0x02;

        #[test]
        fn should_shift_left_value_in_memory_at_absolute_address() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ABSOLUTE_ADDR_LO,
                ABSOLUTE_ADDR_HI,
                0x00,
                0x00,
                VALUE,
            ]));
            let mut cpu = CPU::new(memory);
            cpu.index_register_x = OFFSET;
            cpu.program_counter = 0x00;

            asl_ax(&mut cpu);

            assert_eq!(memory.borrow()[(ABSOLUTE_ADDR_LO + OFFSET) as Word], 0x04);
        }

        #[test]
        fn should_take_six_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ABSOLUTE_ADDR_LO,
                ABSOLUTE_ADDR_HI,
                0x00,
                0x00,
                VALUE,
            ]));
            let mut cpu = CPU::new(memory);
            cpu.index_register_x = OFFSET;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            asl_ax(&mut cpu);

            assert_eq!(cpu.cycle, 6);
        }
    }
}

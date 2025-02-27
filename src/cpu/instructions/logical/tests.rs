#[cfg(test)]
mod ora {
    #[cfg(test)]
    mod ora_im {
        use std::cell::RefCell;

        use crate::cpu::{instructions::ora_im, tests::MemoryMock, CPU};

        #[test]
        fn should_or_accumulator_with_a_value_from_immediate_address() {
            let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x16;

            ora_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x86;

            ora_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_one_cycle() {
            let memory = &RefCell::new(MemoryMock::new(&[0x22, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x14;
            cpu.cycle = 0;

            ora_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod ora_zp {
        use std::cell::RefCell;

        use crate::cpu::{instructions::ora_zp, tests::MemoryMock, Byte, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x03;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_from_zero_page_address() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;

            ora_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;

            ora_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod ora_zpx {
        use std::cell::RefCell;

        use crate::cpu::{instructions::ora_zpx, tests::MemoryMock, Byte, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_at_a_zero_page_address_summed_with_index_register_x()
        {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            ora_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            ora_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ora_a {
        use std::cell::RefCell;

        use crate::cpu::{instructions::ora_a, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;

            ora_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;

            ora_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ora_ax {
        use std::cell::RefCell;

        use crate::cpu::{instructions::ora_ax, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const VALUE: Byte = 0x22;
        const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

        #[test]
        fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            ora_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            ora_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            ora_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDR_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            ora_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod ora_ay {
        use std::cell::RefCell;

        use crate::cpu::{instructions::ora_ay, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const VALUE: Byte = 0x22;
        const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

        #[test]
        fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_y(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            ora_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            ora_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            ora_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDR_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            ora_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod ora_inx {
        use std::cell::RefCell;

        use crate::cpu::{instructions::ora_inx, tests::MemoryMock, Byte, CPU};

        const ZP_ADDRESS: Byte = 0x02;
        const OFFSET: Byte = 0x01;
        const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
        const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x16;
            cpu.index_register_x = OFFSET;

            ora_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x86;
            cpu.index_register_x = OFFSET;

            ora_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_five_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x16;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            ora_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod ora_iny {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{instructions::ora_iny, tests::MemoryMock, CPU},
        };

        const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            ora_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            ora_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut payload: [Byte; 512] = [0x00; 512];
            payload[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            payload[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
            payload[0x0002] = ADDRESS_HI;
            payload[0x0101] = VALUE;

            let memory = &RefCell::new(MemoryMock::new(&payload));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x16;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 5);
        }
    }
}

#[cfg(test)]
mod bit {
    #[cfg(test)]
    mod bit_zp {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{instructions::bit_zp, tests::MemoryMock, CPU},
        };

        const ZERO_PAGE_ADDR_LO: Byte = 0x01;

        #[test]
        fn should_set_zero_flag_when_logic_and_on_accumulator_and_value_from_zero_page_is_zero() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR_LO, 0x0F]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xF0;

            bit_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0b00000010);
        }

        #[test]
        fn should_set_carry_flag_when_logic_and_on_accumulator_and_value_from_zero_page_is_has_seventh_bit_set(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR_LO, 0b01000000]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xF0;

            bit_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0b01000000);
        }

        #[test]
        fn should_set_negative_flag_when_logic_and_on_accumulator_and_value_from_zero_page_is_has_eight_bit_set(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR_LO, 0b10000000]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xF0;

            bit_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR_LO, 0x0F]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xF0;

            bit_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod bit_a {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{instructions::bit_a, tests::MemoryMock, CPU},
        };

        const ABSOLUTE_ADDR_LO: Byte = 0x03;
        const ABSOLUTE_ADDR_HI: Byte = 0x00;

        #[test]
        fn should_set_zero_flag_when_logic_and_on_accumulator_and_value_from_absolute_address_is_zero(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                ABSOLUTE_ADDR_LO,
                ABSOLUTE_ADDR_HI,
                0x00,
                0x0F,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xF0;

            bit_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0b00000010);
        }

        #[test]
        fn should_set_carry_flag_when_logic_and_on_accumulator_and_value_from_absolute_address_is_has_seventh_bit_set(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                ABSOLUTE_ADDR_LO,
                ABSOLUTE_ADDR_HI,
                0x00,
                0b01000000,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xF0;

            bit_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0b01000000);
        }

        #[test]
        fn should_set_negative_flag_when_logic_and_on_accumulator_and_value_from_absolute_address_is_has_eight_bit_set(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                ABSOLUTE_ADDR_LO,
                ABSOLUTE_ADDR_HI,
                0x00,
                0b10000000,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xF0;

            bit_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ABSOLUTE_ADDR_LO,
                ABSOLUTE_ADDR_HI,
                0x00,
                0x0F,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xF0;

            bit_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }
    }
}

#[cfg(test)]
mod and {
    #[cfg(test)]
    mod and_im {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{instructions::and_im, tests::MemoryMock, CPU},
        };

        const VALUE: Byte = 0x82;

        #[test]
        fn should_and_accumulator_with_a_value_from_immediate_address() {
            let memory = &RefCell::new(MemoryMock::new(&[VALUE, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;

            and_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x02);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[VALUE, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x86;

            and_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_one_cycle() {
            let memory = &RefCell::new(MemoryMock::new(&[VALUE, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;
            cpu.cycle = 0;

            and_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod and_zp {
        use std::cell::RefCell;

        use crate::cpu::{instructions::and_zp, tests::MemoryMock, Byte, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x03;
        const VALUE: Byte = 0x82;

        #[test]
        fn should_and_accumulator_with_a_value_from_zero_page_address() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;

            and_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x02);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;

            and_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            and_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod and_zpx {
        use std::cell::RefCell;

        use crate::cpu::{instructions::and_zpx, tests::MemoryMock, Byte, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const VALUE: Byte = 0x82;

        #[test]
        fn should_and_accumulator_with_a_value_at_a_zero_page_address_summed_with_index_register_x()
        {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            and_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x02);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            and_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            and_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod and_a {
        use std::cell::RefCell;

        use crate::cpu::{instructions::and_a, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const VALUE: Byte = 0x82;

        #[test]
        fn should_and_accumulator_with_a_value_in_memory_at_an_absolute_address() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;

            and_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x02);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;

            and_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            and_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod and_ax {
        use std::cell::RefCell;

        use crate::cpu::{instructions::and_ax, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const VALUE: Byte = 0x82;
        const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

        #[test]
        fn should_and_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            and_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x02);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            and_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            and_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDR_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            and_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod and_ay {
        use std::cell::RefCell;

        use crate::cpu::{instructions::and_ay, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const VALUE: Byte = 0x82;
        const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

        #[test]
        fn should_and_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_y(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            and_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x02);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            and_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            and_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDR_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            and_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod and_inx {
        use std::cell::RefCell;

        use crate::cpu::{instructions::and_inx, tests::MemoryMock, Byte, CPU};

        const ZP_ADDRESS: Byte = 0x02;
        const OFFSET: Byte = 0x01;
        const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
        const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x82;

        #[test]
        fn should_and_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;
            cpu.index_register_x = OFFSET;

            and_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x02);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x86;
            cpu.index_register_x = OFFSET;

            and_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_five_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            and_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod and_iny {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{instructions::and_iny, tests::MemoryMock, CPU},
        };

        const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x82;

        #[test]
        fn should_and_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            and_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x02);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x86;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            and_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            and_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut payload: [Byte; 512] = [0x00; 512];
            payload[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            payload[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
            payload[0x0002] = ADDRESS_HI;
            payload[0x0101] = VALUE;

            let memory = &RefCell::new(MemoryMock::new(&payload));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            and_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 5);
        }
    }
}

#[cfg(test)]
mod eor {
    #[cfg(test)]
    mod eor_im {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{instructions::eor_im, tests::MemoryMock, CPU},
        };

        const VALUE: Byte = 0x85;

        #[test]
        fn should_eor_accumulator_with_a_value_from_immediate_address() {
            let memory = &RefCell::new(MemoryMock::new(&[VALUE, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;

            eor_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x82);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[VALUE, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;

            eor_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_one_cycle() {
            let memory = &RefCell::new(MemoryMock::new(&[VALUE, 0x00]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;
            cpu.cycle = 0;

            eor_im(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod eor_zp {
        use std::cell::RefCell;

        use crate::cpu::{instructions::eor_zp, tests::MemoryMock, Byte, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x03;
        const VALUE: Byte = 0x85;

        #[test]
        fn should_eor_accumulator_with_a_value_from_zero_page_address() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;

            eor_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x82);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;

            eor_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            eor_zp(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod eor_zpx {
        use std::cell::RefCell;

        use crate::cpu::{instructions::eor_zpx, tests::MemoryMock, Byte, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const VALUE: Byte = 0x85;

        #[test]
        fn should_eor_accumulator_with_a_value_at_a_zero_page_address_summed_with_index_register_x()
        {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            eor_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x82);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            eor_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            eor_zpx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod eor_a {
        use std::cell::RefCell;

        use crate::cpu::{instructions::eor_a, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const VALUE: Byte = 0x85;

        #[test]
        fn should_eor_accumulator_with_a_value_in_memory_at_an_absolute_address() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;

            eor_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x82);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;

            eor_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            eor_a(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod eor_ax {
        use std::cell::RefCell;

        use crate::cpu::{instructions::eor_ax, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const VALUE: Byte = 0x85;
        const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

        #[test]
        fn should_eor_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            eor_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x82);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            eor_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            eor_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDR_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            eor_ax(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod eor_ay {
        use std::cell::RefCell;

        use crate::cpu::{instructions::eor_ay, tests::MemoryMock, Byte, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const VALUE: Byte = 0x85;
        const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

        #[test]
        fn should_eor_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_y(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            eor_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x82);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            eor_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            eor_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDR_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            eor_ay(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod eor_inx {
        use std::cell::RefCell;

        use crate::cpu::{instructions::eor_inx, tests::MemoryMock, Byte, CPU};

        const ZP_ADDRESS: Byte = 0x02;
        const OFFSET: Byte = 0x01;
        const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
        const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x85;

        #[test]
        fn should_eor_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;
            cpu.index_register_x = OFFSET;

            eor_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x82);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;
            cpu.index_register_x = OFFSET;

            eor_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_five_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x07;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            eor_inx(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod eor_iny {
        use std::cell::RefCell;

        use crate::{
            consts::Byte,
            cpu::{instructions::eor_iny, tests::MemoryMock, CPU},
        };

        const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x85;

        #[test]
        fn should_eor_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            eor_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.accumulator, 0x82);
        }

        #[test]
        fn should_set_processor_status() {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            eor_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.processor_status, 0x80);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            eor_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut payload: [Byte; 512] = [0x00; 512];
            payload[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            payload[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
            payload[0x0002] = ADDRESS_HI;
            payload[0x0101] = VALUE;

            let memory = &RefCell::new(MemoryMock::new(&payload));
            let mut cpu = CPU::new_nmos(memory);
            cpu.accumulator = 0x07;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            eor_iny(&mut cpu);
            cpu.execute_next_instruction();

            assert_eq!(cpu.cycle, 5);
        }
    }
}

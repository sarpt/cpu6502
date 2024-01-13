#[cfg(test)]
mod lda {
    #[cfg(test)]
    mod lda_im {
        use crate::cpu::{instructions::lda_im, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_pointed_by_program_counter_into_accumulator() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_im(&mut cpu);

            assert_eq!(cpu.accumulator, 0x44);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x04;

            lda_im(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod lda_zp {
        use crate::cpu::{instructions::lda_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_accumulator(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_zp(&mut cpu);

            assert_eq!(cpu.accumulator, 0x45);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            lda_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod lda_zpx {
        use crate::cpu::{instructions::lda_zpx, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_x_into_accumulator(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_zpx(&mut cpu);

            assert_eq!(cpu.accumulator, 0x55);
        }

        #[test]
        fn should_overflow_over_byte_when_summing_address_from_memory_with_register_x() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0xFF, 0x88, 0x00])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            lda_zpx(&mut cpu);

            assert_eq!(cpu.accumulator, 0x88);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0xFF])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            lda_zpx(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod lda_a {
        use crate::cpu::{instructions::lda_a, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_accumulator(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_a(&mut cpu);

            assert_eq!(cpu.accumulator, 0x45);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            lda_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod lda_a_x {
        use crate::{
            consts::Byte,
            cpu::{instructions::lda_a_x, tests::MemoryMock, CPU},
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_x_into_accumulator()
        {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            assert_eq!(cpu.accumulator, 0x0);

            lda_a_x(&mut cpu);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;

            lda_a_x(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            lda_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            lda_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod lda_a_y {
        use crate::{
            consts::Byte,
            cpu::{instructions::lda_a_y, tests::MemoryMock, CPU},
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_y_into_accumulator()
        {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            assert_eq!(cpu.accumulator, 0x0);

            lda_a_y(&mut cpu);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;

            lda_a_y(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            lda_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            lda_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod lda_in_y {
        use crate::{
            consts::Byte,
            cpu::{instructions::lda_in_y, tests::MemoryMock, CPU},
        };

        const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.accumulator, 0x0);

            lda_in_y(&mut cpu);

            assert_eq!(cpu.accumulator, VALUE);
        }

        #[test]
        fn should_set_load_accumulator_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            lda_in_y(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_in_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut memory: [Byte; 512] = [0x00; 512];
            memory[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            memory[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
            memory[0x0002] = ADDRESS_HI;
            memory[0x0101] = VALUE;

            let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            lda_in_y(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }
}

#[cfg(test)]
mod ldx {
    #[cfg(test)]
    mod ldx_im {
        use crate::cpu::{instructions::ldx_im, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_pointed_by_program_counter_into_index_register_x() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_im(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x44);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x04;

            ldx_im(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldx_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod ldx_zp {
        use crate::cpu::{instructions::ldx_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_zp(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x45);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            ldx_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldx_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod ldx_zpy {
        use crate::cpu::{instructions::ldx_zpy, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_y_into_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_zpy(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x55);
        }

        #[test]
        fn should_overflow_over_byte_when_summing_address_from_memory_with_register_y() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0xFF, 0x88, 0x00])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            ldx_zpy(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x88);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0xFF])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            ldx_zpy(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldx_zpy(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldx_a {
        use crate::cpu::{instructions::ldx_a, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_a(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x45);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            ldx_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldx_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldx_a_y {
        use crate::{
            consts::Byte,
            cpu::{instructions::ldx_a_y, tests::MemoryMock, CPU},
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_y_into_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            assert_eq!(cpu.index_register_x, 0x0);

            ldx_a_y(&mut cpu);

            assert_eq!(cpu.index_register_x, VALUE);
        }

        #[test]
        fn should_set_load_index_register_x_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;

            ldx_a_y(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            ldx_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            ldx_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }
}

#[cfg(test)]
mod ldy {
    #[cfg(test)]
    mod ldy_im {
        use crate::cpu::{instructions::ldy_im, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_pointed_by_program_counter_into_index_register_y() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_y, 0x0);

            ldy_im(&mut cpu);

            assert_eq!(cpu.index_register_y, 0x44);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x04;

            ldy_im(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldy_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod ldy_zp {
        use crate::cpu::{instructions::ldy_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_a_zero_page_address_stored_in_a_place_pointed_by_program_counter_into_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_y, 0x0);

            ldy_zp(&mut cpu);

            assert_eq!(cpu.index_register_y, 0x45);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            ldy_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldy_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod ldy_zpx {
        use crate::cpu::{instructions::ldy_zpx, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_address_stored_in_program_counter_pointed_place_summed_with_index_register_x_into_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_y, 0x0);

            ldy_zpx(&mut cpu);

            assert_eq!(cpu.index_register_y, 0x55);
        }

        #[test]
        fn should_overflow_over_byte_when_summing_address_from_memory_with_register_x() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0xFF, 0x88, 0x00])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            ldy_zpx(&mut cpu);

            assert_eq!(cpu.index_register_y, 0x88);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0xFF])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            ldy_zpx(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x55])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldy_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldy_a {
        use crate::cpu::{instructions::ldy_a, tests::MemoryMock, CPU};

        #[test]
        fn should_fetch_byte_from_an_absolute_address_stored_in_a_place_pointed_by_program_counter_into_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x45])));
            cpu.program_counter = 0x00;
            assert_eq!(cpu.index_register_y, 0x0);

            ldy_a(&mut cpu);

            assert_eq!(cpu.index_register_y, 0x45);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0xFF])));
            cpu.program_counter = 0x00;

            ldy_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x05])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ldy_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ldy_a_x {
        use crate::{
            consts::Byte,
            cpu::{instructions::ldy_a_x, tests::MemoryMock, CPU},
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0xDB;

        #[test]
        fn should_fetch_byte_from_an_absolute_address_offset_by_index_register_x_into_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            assert_eq!(cpu.index_register_y, 0x0);

            ldy_a_x(&mut cpu);

            assert_eq!(cpu.index_register_y, VALUE);
        }

        #[test]
        fn should_set_load_index_register_y_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;

            ldy_a_x(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            ldy_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            ldy_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }
}

#[cfg(test)]
mod jsr_a {
    use super::super::*;
    use crate::cpu::tests::MemoryMock;

    #[test]
    fn should_fetch_address_pointed_by_program_counter_and_put_in_program_counter() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        jsr_a(&mut cpu);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_save_program_counter_shifted_once_into_stack_pointer() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        jsr_a(&mut cpu);

        assert_eq!(cpu.memory[0x01FF], 0x01);
        assert_eq!(cpu.memory[0x01FE], 0x00);
    }

    #[test]
    fn should_decrement_stack_pointer_twice() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;
        cpu.stack_pointer = 0xFF;

        jsr_a(&mut cpu);

        assert_eq!(cpu.stack_pointer, 0xFD);
    }

    #[test]
    fn should_take_five_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jsr_a(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod rts {
    use super::super::*;
    use crate::cpu::tests::MemoryMock;

    #[test]
    fn should_fetch_address_from_stack_and_put_it_in_program_counter_incremented_by_one() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x02, 0x03])));
        cpu.program_counter = 0x00;
        cpu.memory[0x01FF] = 0x44;
        cpu.memory[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;

        rts(&mut cpu);

        assert_eq!(cpu.program_counter, 0x4452);
    }

    #[test]
    fn should_increment_stack_pointer_twice() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x02, 0x03])));
        cpu.program_counter = 0x00;
        cpu.memory[0x01FF] = 0x44;
        cpu.memory[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;

        rts(&mut cpu);

        assert_eq!(cpu.stack_pointer, 0xFF);
    }

    #[test]
    fn should_take_five_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x02, 0x03])));
        cpu.program_counter = 0x00;
        cpu.memory[0x01FF] = 0x44;
        cpu.memory[0x01FE] = 0x51;
        cpu.stack_pointer = 0xFD;
        cpu.cycle = 0;

        rts(&mut cpu);

        assert_eq!(cpu.cycle, 5);
    }
}

#[cfg(test)]
mod jmp_a {
    use super::super::*;
    use crate::cpu::tests::MemoryMock;

    #[test]
    fn should_put_address_stored_in_memory_at_program_counter_as_a_new_program_counter() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;

        jmp_a(&mut cpu);

        assert_eq!(cpu.program_counter, 0x5144);
    }

    #[test]
    fn should_take_two_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x44, 0x51, 0x88])));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jmp_a(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }
}

#[cfg(test)]
mod jmp_in {
    use super::super::*;
    use crate::cpu::tests::MemoryMock;

    #[test]
    fn should_fetch_indirect_address_from_memory_and_put_in_program_counter() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
        cpu.program_counter = 0x00;

        jmp_in(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_four_cycles() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        jmp_in(&mut cpu);

        assert_eq!(cpu.cycle, 4);
    }
}

#[cfg(test)]
mod beq {
    use super::super::*;
    use crate::{consts::Byte, cpu::tests::MemoryMock};

    #[test]
    fn should_not_take_branch_when_zero_flag_is_clear_and_advance_past_operand() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_zero_flag(false);
        cpu.program_counter = 0x00;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set_and_offset_program_counter_by_operand() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_zero_flag(true);
        cpu.program_counter = 0x00;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set_and_offset_program_counter_backwards_by_negative_operand(
    ) {
        const OFFSET: Byte = 0x83;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00, OFFSET, 0x00])));
        cpu.processor_status.set_zero_flag(true);
        cpu.program_counter = 0x02;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set_and_offset_program_counter_over_page_flip_by_operand(
    ) {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
        cpu.processor_status.set_zero_flag(true);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_set_and_offset_program_counter_backwards_over_page_flip_by_negative_operand(
    ) {
        const OFFSET: Byte = 0x83;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x00, 0x00])));
        cpu.processor_status.set_zero_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_zero_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_zero_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
        cpu.processor_status.set_zero_flag(true);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        beq(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod bne {
    use super::super::*;
    use crate::{consts::Byte, cpu::tests::MemoryMock};

    #[test]
    fn should_not_take_branch_when_zero_flag_is_set_and_advance_past_operand() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_zero_flag(true);
        cpu.program_counter = 0x00;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear_and_offset_program_counter_by_operand() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_zero_flag(false);
        cpu.program_counter = 0x00;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear_and_offset_program_counter_backwards_by_negative_operand(
    ) {
        const OFFSET: Byte = 0x83;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00, OFFSET, 0x00])));
        cpu.processor_status.set_zero_flag(false);
        cpu.program_counter = 0x02;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear_and_offset_program_counter_over_page_flip_by_operand(
    ) {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
        cpu.processor_status.set_zero_flag(false);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_when_zero_flag_is_clear_and_offset_program_counter_backwards_over_page_flip_by_negative_operand(
    ) {
        const OFFSET: Byte = 0x83;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x00, 0x00])));
        cpu.processor_status.set_zero_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_zero_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_zero_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
        cpu.processor_status.set_zero_flag(false);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bne(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod bcs {
    use super::super::*;
    use crate::{consts::Byte, cpu::tests::MemoryMock};

    #[test]
    fn should_not_take_branch_when_carry_flag_is_clear_and_advance_past_operand() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_carry_flag(false);
        cpu.program_counter = 0x00;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set_and_offset_program_counter_by_operand() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_carry_flag(true);
        cpu.program_counter = 0x00;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set_and_offset_program_counter_backwards_by_negative_operand(
    ) {
        const OFFSET: Byte = 0x83;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00, OFFSET, 0x00])));
        cpu.processor_status.set_carry_flag(true);
        cpu.program_counter = 0x02;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set_and_offset_program_counter_over_page_flip_by_operand(
    ) {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
        cpu.processor_status.set_carry_flag(true);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_set_and_offset_program_counter_backwards_over_page_flip_by_negative_operand(
    ) {
        const OFFSET: Byte = 0x83;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x00, 0x00])));
        cpu.processor_status.set_carry_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_carry_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_carry_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
        cpu.processor_status.set_carry_flag(true);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bcs(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod bcc {
    use super::super::*;
    use crate::{consts::Byte, cpu::tests::MemoryMock};

    #[test]
    fn should_not_take_branch_when_carry_flag_is_set_and_advance_past_operand() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_carry_flag(true);
        cpu.program_counter = 0x00;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0001);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear_and_offset_program_counter_by_operand() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_carry_flag(false);
        cpu.program_counter = 0x00;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0004);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear_and_offset_program_counter_backwards_by_negative_operand(
    ) {
        const OFFSET: Byte = 0x83;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00, OFFSET, 0x00])));
        cpu.processor_status.set_carry_flag(false);
        cpu.program_counter = 0x02;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x00);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear_and_offset_program_counter_over_page_flip_by_operand(
    ) {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
        cpu.processor_status.set_carry_flag(false);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0x0103);
    }

    #[test]
    fn should_take_branch_when_carry_flag_is_clear_and_offset_program_counter_backwards_over_page_flip_by_negative_operand(
    ) {
        const OFFSET: Byte = 0x83;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x00, 0x00])));
        cpu.processor_status.set_carry_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.program_counter, 0xFFFE);
    }

    #[test]
    fn should_take_one_cycle_when_not_branching() {
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_carry_flag(true);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }

    #[test]
    fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
        const OFFSET: Byte = 0x03;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00])));
        cpu.processor_status.set_carry_flag(false);
        cpu.program_counter = 0x00;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.cycle, 2);
    }

    #[test]
    fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
        const OFFSET: Byte = 0x04;
        let mut memory: [Byte; 512] = [0x00; 512];
        memory[0x00FE] = OFFSET;
        let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
        cpu.processor_status.set_carry_flag(false);
        cpu.program_counter = 0x00FE;
        cpu.cycle = 0;

        bcc(&mut cpu);

        assert_eq!(cpu.cycle, 3);
    }
}

#[cfg(test)]
mod cmp {
    #[cfg(test)]
    mod cmp_im {
        use crate::cpu::{instructions::cmp_im, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_accumulator_with_next_byte_from_memory() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cmp_im(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod cmp_zp {
        use crate::cpu::{instructions::cmp_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_accumulator_with_a_value_from_a_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cmp_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod cmp_zpx {
        use crate::cpu::{instructions::cmp_zpx, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_accumulator_with_a_value_from_a_zero_page_summed_with_index_register_x() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x03])));
            cpu.accumulator = 0x02;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cmp_zpx(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x01, 0x00, 0x00, 0x03])));
            cpu.accumulator = 0x02;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod cmp_a {
        use crate::cpu::{instructions::cmp_a, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_accumulator_with_a_value_from_an_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x03])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cmp_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x03])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod cmp_a_x {
        use crate::{
            consts::Byte,
            cpu::{instructions::cmp_a_x, tests::MemoryMock, CPU},
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x03;

        #[test]
        fn should_compare_accumulator_with_a_value_stored_in_address_ofset_by_x_register() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cmp_a_x(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            cmp_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            cmp_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod cmp_a_y {
        use crate::{
            consts::Byte,
            cpu::{instructions::cmp_a_y, tests::MemoryMock, CPU},
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x03;

        #[test]
        fn should_compare_accumulator_with_a_value_stored_in_address_ofset_by_y_register() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cmp_a_y(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            cmp_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            cmp_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod cmp_in_y {
        use crate::{
            consts::Byte,
            cpu::{instructions::cmp_in_y, tests::MemoryMock, CPU},
        };

        const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x03;

        #[test]
        fn should_compare_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.accumulator = 0x02;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cmp_in_y(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.accumulator = 0x02;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_in_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut memory: [Byte; 512] = [0x00; 512];
            memory[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            memory[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
            memory[0x0002] = ADDRESS_HI;
            memory[0x0101] = VALUE;

            let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
            cpu.accumulator = 0x02;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_in_y(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }
}

#[cfg(test)]
mod cpy {
    #[cfg(test)]
    mod cpy_im {
        use crate::cpu::{instructions::cpy_im, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_y_register_with_next_byte_from_memory() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cpy_im(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpy_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod cpy_zp {
        use crate::cpu::{instructions::cpy_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_y_register_with_a_value_from_a_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cpy_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpy_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod cpy_a {
        use crate::cpu::{instructions::cpy_a, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_y_register_with_a_value_from_an_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x03])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cpy_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x03])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpy_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }
}

#[cfg(test)]
mod cpx {
    #[cfg(test)]
    mod cpx_im {
        use crate::cpu::{instructions::cpx_im, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_x_register_with_next_byte_from_memory() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cpx_im(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpx_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod cpx_zp {
        use crate::cpu::{instructions::cpx_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_x_register_with_a_value_from_a_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cpx_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0x00, 0x04])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpx_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod cpy_a {
        use crate::cpu::{instructions::cpx_a, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_x_register_with_a_value_from_an_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x03])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status.flags, 0b00000000);

            cpx_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x03, 0x00, 0x00, 0x03])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpx_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }
}

#[cfg(test)]
mod increment {
    #[cfg(test)]
    mod inx_im {
        use crate::cpu::{instructions::inx_im, tests::MemoryMock, CPU};

        #[test]
        fn should_increment_x_register() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.index_register_x = 0x02;

            inx_im(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x03);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            inx_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod iny_im {
        use crate::cpu::{instructions::iny_im, tests::MemoryMock, CPU};

        #[test]
        fn should_increment_y_register() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.index_register_y = 0x02;

            iny_im(&mut cpu);

            assert_eq!(cpu.index_register_y, 0x03);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            iny_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod inc_zp {
        use crate::cpu::{instructions::inc_zp, tests::MemoryMock, Byte, Word, CPU};

        const VALUE: Byte = 0x02;
        const ZERO_PAGE_ADDR: Byte = 0x03;

        #[test]
        fn should_increment_value_stored_in_memory_at_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.program_counter = 0x00;

            inc_zp(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR as Word], 0x03);
        }

        #[test]
        fn should_take_four_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            inc_zp(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod inc_zpx {
        use crate::cpu::{instructions::inc_zpx, tests::MemoryMock, Byte, Word, CPU};

        const VALUE: Byte = 0x09;
        const ZERO_PAGE_ADDR: Byte = 0x01;
        const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

        #[test]
        fn should_increment_value_stored_in_memory_at_zero_page_address_summed_with_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;

            inc_zpx(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR_SUM_X as Word], 0x0A);
        }

        #[test]
        fn should_take_five_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            inc_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod inc_a {
        use crate::cpu::{instructions::inc_a, tests::MemoryMock, Byte, Word, CPU};

        const VALUE: Byte = 0x09;
        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const ADDR: Word = 0x0004;

        #[test]
        fn should_increment_value_stored_in_memory_at_absolute_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.program_counter = 0x00;

            inc_a(&mut cpu);

            assert_eq!(cpu.memory[ADDR as Word], 0x0A);
        }

        #[test]
        fn should_take_five_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            inc_a(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod inc_a_x {
        use crate::cpu::{instructions::inc_a_x, tests::MemoryMock, Byte, Word, CPU};

        const VALUE: Byte = 0x09;
        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const ADDR_OFFSET_BY_X: Word = 0x0004;

        #[test]
        fn should_increment_value_stored_in_memory_at_absolute_address_offset_by_index_register_x()
        {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            inc_a_x(&mut cpu);

            assert_eq!(cpu.memory[ADDR_OFFSET_BY_X], 0x0A);
        }

        #[test]
        fn should_take_six_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            inc_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 6);
        }
    }
}
#[cfg(test)]
mod decrement {
    #[cfg(test)]
    mod dex_im {
        use crate::cpu::{instructions::dex_im, tests::MemoryMock, CPU};

        #[test]
        fn should_decrement_x_register() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.index_register_x = 0x02;

            dex_im(&mut cpu);

            assert_eq!(cpu.index_register_x, 0x01);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            dex_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod dey_im {
        use crate::cpu::{instructions::dey_im, tests::MemoryMock, CPU};

        #[test]
        fn should_decrement_y_register() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.index_register_y = 0x02;

            dey_im(&mut cpu);

            assert_eq!(cpu.index_register_y, 0x01);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::default()));
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            dey_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod dec_zp {
        use crate::cpu::{instructions::dec_zp, tests::MemoryMock, Byte, Word, CPU};

        const VALUE: Byte = 0x02;
        const ZERO_PAGE_ADDR: Byte = 0x03;

        #[test]
        fn should_decrement_value_stored_in_memory_at_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.program_counter = 0x00;

            dec_zp(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR as Word], 0x01);
        }

        #[test]
        fn should_take_four_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            dec_zp(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod dec_zpx {
        use crate::cpu::{instructions::dec_zpx, tests::MemoryMock, Byte, Word, CPU};

        const VALUE: Byte = 0x09;
        const ZERO_PAGE_ADDR: Byte = 0x01;
        const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

        #[test]
        fn should_decrement_value_stored_in_memory_at_zero_page_address_summed_with_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;

            dec_zpx(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR_SUM_X as Word], 0x08);
        }

        #[test]
        fn should_take_five_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            dec_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod dec_a {
        use crate::cpu::{instructions::dec_a, tests::MemoryMock, Byte, Word, CPU};

        const VALUE: Byte = 0x09;
        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const ADDR: Word = 0x0004;

        #[test]
        fn should_decrement_value_stored_in_memory_at_absolute_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.program_counter = 0x00;

            dec_a(&mut cpu);

            assert_eq!(cpu.memory[ADDR as Word], 0x08);
        }

        #[test]
        fn should_take_five_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            dec_a(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod dec_a_x {
        use crate::cpu::{instructions::dec_a_x, tests::MemoryMock, Byte, Word, CPU};

        const VALUE: Byte = 0x09;
        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const ADDR_OFFSET_BY_X: Word = 0x0004;

        #[test]
        fn should_decrement_value_stored_in_memory_at_absolute_address_offset_by_index_register_x()
        {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            dec_a_x(&mut cpu);

            assert_eq!(cpu.memory[ADDR_OFFSET_BY_X], 0x08);
        }

        #[test]
        fn should_take_six_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            dec_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 6);
        }
    }
}

#[cfg(test)]
mod store {
    #[cfg(test)]
    mod sta_zp {
        use crate::cpu::{instructions::sta_zp, tests::MemoryMock, Byte, Word, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x03;

        #[test]
        fn should_store_accumulator_in_memory_at_a_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;

            sta_zp(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR as Word], 0x02);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            sta_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod sta_zpx {
        use crate::cpu::{instructions::sta_zpx, tests::MemoryMock, Byte, Word, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

        #[test]
        fn should_store_accumulator_in_memory_at_a_zero_page_address_summed_with_index_register_x()
        {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.accumulator = 0x05;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            sta_zpx(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR_SUM_X], 0x05);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.accumulator = 0x05;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            sta_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod sta_a {
        use crate::cpu::{instructions::sta_a, tests::MemoryMock, Byte, Word, CPU};

        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const ADDR: Word = 0x0004;

        #[test]
        fn should_store_accumulator_in_memory_at_an_absolute_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.accumulator = 0x0A;
            cpu.program_counter = 0x00;

            sta_a(&mut cpu);

            assert_eq!(cpu.memory[ADDR as Word], 0x0A);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.accumulator = 0x0A;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            sta_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod sta_a_x {
        use crate::cpu::{instructions::sta_a_x, tests::MemoryMock, Byte, Word, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const ADDR_OFFSET_BY_X: Word = 0x0004;

        #[test]
        fn should_store_accumulator_in_memory_at_an_absolute_address_offset_by_index_register_x() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.accumulator = 0x08;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            sta_a_x(&mut cpu);

            assert_eq!(cpu.memory[ADDR_OFFSET_BY_X], 0x08);
        }

        #[test]
        fn should_take_four_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.accumulator = 0x08;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            sta_a_x(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod sta_a_y {
        use crate::cpu::{instructions::sta_a_y, tests::MemoryMock, Byte, Word, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const ADDR_OFFSET_BY_Y: Word = 0x0004;

        #[test]
        fn should_store_accumulator_in_memory_at_an_absolute_address_offset_by_index_register_y() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.accumulator = 0x08;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            sta_a_y(&mut cpu);

            assert_eq!(cpu.memory[ADDR_OFFSET_BY_Y], 0x08);
        }

        #[test]
        fn should_take_four_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.accumulator = 0x08;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            sta_a_y(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod sta_in_x {
        use crate::cpu::{instructions::sta_in_x, tests::MemoryMock, Byte, Word, CPU};

        const ZP_ADDRESS: Byte = 0x02;
        const OFFSET: Byte = 0x01;
        const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
        const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
        const EFFECTIVE_ADDRESS: Word = 0x0005;

        #[test]
        fn should_store_accumulator_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                0x00,
                0x00,
            ])));
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xA9;
            cpu.index_register_x = OFFSET;

            sta_in_x(&mut cpu);

            assert_eq!(cpu.memory[EFFECTIVE_ADDRESS], 0xA9);
        }

        #[test]
        fn should_take_five_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                0x00,
                0x00,
            ])));
            cpu.program_counter = 0x00;
            cpu.accumulator = 0xA9;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            sta_in_x(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod sta_in_y {
        use crate::cpu::{instructions::sta_in_y, tests::MemoryMock, Byte, Word, CPU};

        const ZP_ADDRESS: Byte = 0x01;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const OFFSET: Byte = 0x01;
        const EFFECTIVE_ADDRESS: Word = 0x0004;

        #[test]
        fn should_store_accumulator_in_offset_with_index_register_y_indirect_adress_stored_in_zero_page(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZP_ADDRESS, ADDRESS_LO, ADDRESS_HI, 0x00, 0x00,
            ])));
            cpu.accumulator = 0xDF;
            cpu.index_register_y = OFFSET;
            cpu.program_counter = 0x00;

            sta_in_y(&mut cpu);

            assert_eq!(cpu.memory[EFFECTIVE_ADDRESS], 0xDF);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZP_ADDRESS, ADDRESS_LO, ADDRESS_HI, 0x00, 0x00,
            ])));
            cpu.accumulator = 0xDF;
            cpu.index_register_y = OFFSET;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            sta_in_y(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod stx_zp {
        use crate::cpu::{instructions::stx_zp, tests::MemoryMock, Byte, Word, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x03;

        #[test]
        fn should_store_index_register_x_in_memory_at_a_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            stx_zp(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR as Word], 0x02);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            stx_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod stx_zpy {
        use crate::cpu::{instructions::stx_zpy, tests::MemoryMock, Byte, Word, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const ZERO_PAGE_ADDR_SUM_Y: Word = 0x03;

        #[test]
        fn should_store_index_register_x_in_memory_at_a_zero_page_address_summed_with_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.index_register_x = 0x05;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            stx_zpy(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR_SUM_Y], 0x05);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.index_register_x = 0x05;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            stx_zpy(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod stx_a {
        use crate::cpu::{instructions::stx_a, tests::MemoryMock, Byte, Word, CPU};

        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const ADDR: Word = 0x0004;

        #[test]
        fn should_store_index_register_x_in_memory_at_an_absolute_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.index_register_x = 0x0A;
            cpu.program_counter = 0x00;

            stx_a(&mut cpu);

            assert_eq!(cpu.memory[ADDR as Word], 0x0A);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.index_register_x = 0x0A;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            stx_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod sty_zp {
        use crate::cpu::{instructions::sty_zp, tests::MemoryMock, Byte, Word, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x03;

        #[test]
        fn should_store_index_register_y_in_memory_at_a_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            sty_zp(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR as Word], 0x02);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            sty_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod sty_zpx {
        use crate::cpu::{instructions::sty_zpx, tests::MemoryMock, Byte, Word, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

        #[test]
        fn should_store_index_register_y_in_memory_at_a_zero_page_address_summed_with_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.index_register_y = 0x05;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            sty_zpx(&mut cpu);

            assert_eq!(cpu.memory[ZERO_PAGE_ADDR_SUM_X], 0x05);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                0x00,
            ])));
            cpu.index_register_y = 0x05;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            sty_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod sty_a {
        use crate::cpu::{instructions::sty_a, tests::MemoryMock, Byte, Word, CPU};

        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const ADDR: Word = 0x0004;

        #[test]
        fn should_store_index_register_y_in_memory_at_an_absolute_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.index_register_y = 0x0A;
            cpu.program_counter = 0x00;

            sty_a(&mut cpu);

            assert_eq!(cpu.memory[ADDR as Word], 0x0A);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, 0x00,
            ])));
            cpu.index_register_y = 0x0A;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            sty_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }
}

#[cfg(test)]
mod ora {
    #[cfg(test)]
    mod ora_im {
        use crate::cpu::{instructions::ora_im, tests::MemoryMock, CPU};

        #[test]
        fn should_or_accumulator_with_a_value_from_immediate_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00])));
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x16;

            ora_im(&mut cpu);

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00])));
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x86;

            ora_im(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0x80);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[0x22, 0x00])));
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x14;
            cpu.cycle = 0;

            ora_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod ora_zp {
        use crate::cpu::{instructions::ora_zp, tests::MemoryMock, Byte, Word, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x03;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_from_zero_page_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;

            ora_zp(&mut cpu);

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;

            ora_zp(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0x80);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod ora_zpx {
        use crate::cpu::{instructions::ora_zpx, tests::MemoryMock, Byte, Word, CPU};

        const ZERO_PAGE_ADDR: Byte = 0x01;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_at_a_zero_page_address_summed_with_index_register_x()
        {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            ora_zpx(&mut cpu);

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.accumulator = 0x86;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;

            ora_zpx(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0x80);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZERO_PAGE_ADDR,
                0xFF,
                0x00,
                VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_zpx(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ora_a {
        use crate::cpu::{instructions::ora_a, tests::MemoryMock, Byte, Word, CPU};

        const ADDR_LO: Byte = 0x04;
        const ADDR_HI: Byte = 0x00;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;

            ora_a(&mut cpu);

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;

            ora_a(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0x80);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod ora_ax {
        use crate::cpu::{instructions::ora_ax, tests::MemoryMock, Byte, Word, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const VALUE: Byte = 0x22;
        const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

        #[test]
        fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            ora_ax(&mut cpu);

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;

            ora_ax(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0x80);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            ora_ax(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDR_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            ora_ax(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod ora_ay {
        use crate::cpu::{instructions::ora_ay, tests::MemoryMock, Byte, Word, CPU};

        const ADDR_LO: Byte = 0x02;
        const ADDR_HI: Byte = 0x00;
        const OFFSET: Byte = 0x02;
        const VALUE: Byte = 0x22;
        const ADDR_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;

        #[test]
        fn should_or_accumulator_with_a_value_in_memory_at_an_absolute_address_offset_by_index_register_y(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            ora_ay(&mut cpu);

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x86;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;

            ora_ay(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0x80);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            ora_ay(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ADDR_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDR_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.program_counter = 0x00;
            cpu.index_register_y = OFFSET;
            cpu.cycle = 0;

            ora_ay(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod ora_inx {
        use crate::cpu::{instructions::ora_inx, tests::MemoryMock, Byte, Word, CPU};

        const ZP_ADDRESS: Byte = 0x02;
        const OFFSET: Byte = 0x01;
        const EFFECTIVE_ADDRESS_LO: Byte = 0x05;
        const EFFECTIVE_ADDRESS_HI: Byte = 0x00;
        const EFFECTIVE_ADDRESS: Word = 0x0005;
        const VALUE: Byte = 0x22;

        #[test]
        fn should_or_accumulator_with_a_value_in_an_indirect_adress_stored_in_zero_page_offset_with_index_register_x(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x16;
            cpu.index_register_x = OFFSET;

            ora_inx(&mut cpu);

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x86;
            cpu.index_register_x = OFFSET;

            ora_inx(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0x80);
        }

        #[test]
        fn should_take_five_cycles() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                ZP_ADDRESS,
                0x00,
                0x00,
                EFFECTIVE_ADDRESS_LO,
                EFFECTIVE_ADDRESS_HI,
                VALUE,
            ])));
            cpu.program_counter = 0x00;
            cpu.accumulator = 0x16;
            cpu.index_register_x = OFFSET;
            cpu.cycle = 0;

            ora_inx(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod ora_iny {
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
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            ora_iny(&mut cpu);

            assert_eq!(cpu.accumulator, 0x36);
        }

        #[test]
        fn should_set_processor_status() {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.accumulator = 0x86;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            ora_iny(&mut cpu);

            assert_eq!(cpu.processor_status.flags, 0x80);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let mut cpu = CPU::new(Box::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ])));
            cpu.accumulator = 0x16;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_iny(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut memory: [Byte; 512] = [0x00; 512];
            memory[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            memory[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
            memory[0x0002] = ADDRESS_HI;
            memory[0x0101] = VALUE;

            let mut cpu = CPU::new(Box::new(MemoryMock::new(&memory)));
            cpu.accumulator = 0x16;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            ora_iny(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }
}

#[cfg(test)]
mod nop {
    use crate::cpu::{instructions::nop, tests::MemoryMock, CPU};

    #[test]
    fn should_increment_program_counter() {
        let mut cpu = CPU::new(Box::new(MemoryMock::default()));
        cpu.program_counter = 0x05;

        nop(&mut cpu);

        assert_eq!(cpu.program_counter, 0x06);
    }

    #[test]
    fn should_take_one_cycle() {
        let mut cpu = CPU::new(Box::new(MemoryMock::default()));
        cpu.program_counter = 0x05;
        cpu.cycle = 0;

        nop(&mut cpu);

        assert_eq!(cpu.cycle, 1);
    }
}

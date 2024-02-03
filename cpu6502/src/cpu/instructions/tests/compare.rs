#[cfg(test)]
mod cmp {
    #[cfg(test)]
    mod cmp_im {
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cmp_im, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_accumulator_with_next_byte_from_memory() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x03, 0xFF]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cmp_im(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x03, 0xFF]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod cmp_zp {
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cmp_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_accumulator_with_a_value_from_a_zero_page_address() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0xFF, 0x00, 0x04,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cmp_zp(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0xFF, 0x00, 0x04,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod cmp_zpx {
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cmp_zpx, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_accumulator_with_a_value_from_a_zero_page_summed_with_index_register_x() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x01, 0x00, 0x00, 0x03,
            ]))));
            cpu.accumulator = 0x02;
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cmp_zpx(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x01, 0x00, 0x00, 0x03,
            ]))));
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
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cmp_a, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_accumulator_with_a_value_from_an_address() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0x00, 0x00, 0x03,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cmp_a(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0x00, 0x00, 0x03,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }

    #[cfg(test)]
    mod cmp_ax {
        use std::{cell::RefCell, rc::Rc};

        use crate::{
            consts::Byte,
            cpu::{instructions::cmp_ax, tests::MemoryMock, CPU},
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x03;

        #[test]
        fn should_compare_accumulator_with_a_value_stored_in_address_ofset_by_x_register() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            assert_eq!(cpu.processor_status, 0b00000000);

            cmp_ax(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            cmp_ax(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x02;
            cpu.cycle = 0;

            cmp_ax(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod cmp_ay {
        use std::{cell::RefCell, rc::Rc};

        use crate::{
            consts::Byte,
            cpu::{instructions::cmp_ay, tests::MemoryMock, CPU},
        };

        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x03;

        #[test]
        fn should_compare_accumulator_with_a_value_stored_in_address_ofset_by_y_register() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            assert_eq!(cpu.processor_status, 0b00000000);

            cmp_ay(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                ADDRESS_LO, ADDRESS_HI, 0x45, 0xAF, 0xDD, VALUE,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            cmp_ay(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_adding_offset_crosses_over_page_flip() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY,
                ADDRESS_HI,
                0x45,
                0xAF,
                0xDD,
                VALUE,
            ]))));
            cpu.accumulator = 0x02;
            cpu.program_counter = 0x00;
            cpu.index_register_y = 0x02;
            cpu.cycle = 0;

            cmp_ay(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod cmp_iny {
        use std::{cell::RefCell, rc::Rc};

        use crate::{
            consts::Byte,
            cpu::{instructions::cmp_iny, tests::MemoryMock, CPU},
        };

        const INDIRECT_ZERO_PAGE_ADDRESS_PLACE: Byte = 0x01;
        const ADDRESS_LO: Byte = 0x03;
        const ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY: Byte = 0xFF;
        const ADDRESS_HI: Byte = 0x00;
        const VALUE: Byte = 0x03;

        #[test]
        fn should_compare_accumulator_with_a_value_from_an_indirect_adress_stored_in_memory_at_zero_page_and_offset_with_value_from_index_register_y(
        ) {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]))));
            cpu.accumulator = 0x02;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cmp_iny(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_four_cycles_when_summing_indirect_address_with_index_y_does_not_cross_page_flip(
        ) {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                INDIRECT_ZERO_PAGE_ADDRESS_PLACE,
                ADDRESS_LO,
                ADDRESS_HI,
                0x45,
                0xAF,
                VALUE,
            ]))));
            cpu.accumulator = 0x02;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_iny(&mut cpu);

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_summing_indirect_address_with_index_y_crosses_page_flip() {
            let mut memory: [Byte; 512] = [0x00; 512];
            memory[0x0000] = INDIRECT_ZERO_PAGE_ADDRESS_PLACE;
            memory[0x0001] = ADDRESS_LO_ON_ZERO_PAGE_BOUNDARY;
            memory[0x0002] = ADDRESS_HI;
            memory[0x0101] = VALUE;

            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&memory))));
            cpu.accumulator = 0x02;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cmp_iny(&mut cpu);

            assert_eq!(cpu.cycle, 5);
        }
    }
}

#[cfg(test)]
mod cpy {
    #[cfg(test)]
    mod cpy_im {
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cpy_im, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_y_register_with_next_byte_from_memory() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x03, 0xFF]))));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cpy_im(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x03, 0xFF]))));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpy_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod cpy_zp {
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cpy_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_y_register_with_a_value_from_a_zero_page_address() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0xFF, 0x00, 0x04,
            ]))));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cpy_zp(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0xFF, 0x00, 0x04,
            ]))));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpy_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod cpy_a {
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cpy_a, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_y_register_with_a_value_from_an_address() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0x00, 0x00, 0x03,
            ]))));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cpy_a(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0x00, 0x00, 0x03,
            ]))));
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
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cpx_im, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_x_register_with_next_byte_from_memory() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x03, 0xFF]))));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cpx_im(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[0x03, 0xFF]))));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpx_im(&mut cpu);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod cpx_zp {
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cpx_zp, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_x_register_with_a_value_from_a_zero_page_address() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0xFF, 0x00, 0x04,
            ]))));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cpx_zp(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0xFF, 0x00, 0x04,
            ]))));
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpx_zp(&mut cpu);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod cpx_a {
        use std::{cell::RefCell, rc::Rc};

        use crate::cpu::{instructions::cpx_a, tests::MemoryMock, CPU};

        #[test]
        fn should_compare_x_register_with_a_value_from_an_address() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0x00, 0x00, 0x03,
            ]))));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            assert_eq!(cpu.processor_status, 0b00000000);

            cpx_a(&mut cpu);

            assert_eq!(cpu.processor_status, 0b10000000);
        }

        #[test]
        fn should_take_three_cycles() {
            let mut cpu = CPU::new(Rc::new(RefCell::new(MemoryMock::new(&[
                0x03, 0x00, 0x00, 0x03,
            ]))));
            cpu.index_register_x = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            cpx_a(&mut cpu);

            assert_eq!(cpu.cycle, 3);
        }
    }
}

mod absolute;
mod indirect;
mod zero_page;

use absolute::{AbsoluteAddressingTasks, AbsoluteOffsetAddressingTasks};
use indirect::{
    IndexIndirectXAddressingTasks, IndirectAddressingTasks, IndirectIndexYAddressingTasks,
};
use zero_page::{ZeroPageAddressingTasks, ZeroPageOffsetAddressingTasks};

use super::{
    tasks::{GenericTasks, Tasks},
    AddressingMode, ChipVariant, CPU,
};

enum OffsetVariant {
    X,
    Y,
}

pub fn get_addressing_tasks(cpu: &CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    match addr_mode {
        AddressingMode::ZeroPage => {
            return Box::new(ZeroPageAddressingTasks::new());
        }
        AddressingMode::ZeroPageX => {
            return Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_x());
        }
        AddressingMode::ZeroPageY => {
            return Box::new(ZeroPageOffsetAddressingTasks::new_offset_by_y());
        }
        AddressingMode::Absolute => {
            return Box::new(AbsoluteAddressingTasks::new());
        }
        AddressingMode::AbsoluteX => {
            return Box::new(AbsoluteOffsetAddressingTasks::new_offset_by_x());
        }
        AddressingMode::AbsoluteY => {
            return Box::new(AbsoluteOffsetAddressingTasks::new_offset_by_y());
        }
        AddressingMode::Indirect => {
            if cpu.chip_variant == ChipVariant::NMOS {
                return Box::new(IndirectAddressingTasks::new_incorrect_addressing());
            } else {
                return Box::new(IndirectAddressingTasks::new_fixed_addressing());
            }
        }
        AddressingMode::IndexIndirectX => {
            return Box::new(IndexIndirectXAddressingTasks::new());
        }
        AddressingMode::IndirectIndexY => {
            return Box::new(IndirectIndexYAddressingTasks::new());
        }
        _ => {
            return Box::new(GenericTasks::new());
        }
    }
}

#[cfg(test)]
mod get_addressing_tasks {
    #[cfg(test)]
    mod absolute_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_from_next_word_in_memory_relative_to_program_counter() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x01;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Absolute);
            run_tasks(&mut cpu, tasks);
            assert_eq!(cpu.address_output, 0xCBFF);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x01;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Absolute);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x01;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Absolute);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod absolute_x_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_offset_by_index_register_x_from_next_word_in_memory_relative_to_program_counter(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x01;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x52CC);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x01;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x04);
        }

        #[test]
        fn should_take_three_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x01;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0xFF;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod absolute_y_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_offset_by_index_register_y_from_next_word_in_memory_relative_to_program_counter(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.index_register_y = 0x01;
            cpu.program_counter = 0x02;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x52CC);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.index_register_y = 0x01;
            cpu.program_counter = 0x02;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x04);
        }

        #[test]
        fn should_take_three_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_y = 0x01;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_y = 0xFF;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::AbsoluteY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod index_indirect_x_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_stored_in_place_pointed_by_zero_page_address_in_next_byte_relative_to_program_counter_summed_with_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.address_output = 0x0;
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x01;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndexIndirectX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0xDD03);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = &RefCell::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x01;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndexIndirectX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x01);
        }

        #[test]
        fn should_take_four_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x00;
            cpu.index_register_x = 0x01;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndexIndirectX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }
    }

    #[cfg(test)]
    mod indirect_index_y_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_offset_by_index_register_y_which_is_stored_at_zero_page_address() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.address_output = 0x0;
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndirectIndexY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0xDD05);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndirectIndexY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x01);
        }

        #[test]
        fn should_take_four_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.index_register_y = 0x02;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndirectIndexY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.index_register_y = 0xFF;
            cpu.program_counter = 0x00;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::IndirectIndexY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 5);
        }
    }

    #[cfg(test)]
    mod zero_page_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter()
        {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPage);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x00CB);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPage);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x03);
        }

        #[test]
        fn should_take_one_cycle() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPage);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 1);
        }
    }

    #[cfg(test)]
    mod zero_page_x_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_x(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x03;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x00CE);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x03;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_x = 0x03;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageX);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod zero_page_y_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_y(
        ) {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x03;
            cpu.index_register_y = 0x03;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x0055);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_y = 0x03;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let memory = &RefCell::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52]));
            let mut cpu = CPU::new_nmos(memory);
            cpu.program_counter = 0x02;
            cpu.index_register_y = 0x03;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::ZeroPageY);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 2);
        }
    }

    #[cfg(test)]
    mod indirect_addressing {
        #[cfg(test)]
        mod common {
            use std::cell::RefCell;

            use crate::cpu::{
                addressing::get_addressing_tasks,
                tests::{run_tasks, MemoryMock},
                AddressingMode, CPU,
            };

            #[test]
            fn should_return_address_from_place_in_memory_stored_in_next_word_relative_to_program_counter(
            ) {
                let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
                let mut cpu = CPU::new_nmos(&memory);
                cpu.program_counter = 0x00;
                cpu.address_output = 0x0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.address_output, 0x0001);
            }

            #[test]
            fn should_advance_program_counter_twice() {
                let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
                let mut cpu = CPU::new_nmos(&memory);
                cpu.program_counter = 0x00;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.program_counter, 0x02);
            }
        }

        #[cfg(test)]
        mod nmos {
            use std::cell::RefCell;

            use crate::{
                consts::Byte,
                cpu::{
                    addressing::get_addressing_tasks,
                    tests::{run_tasks, MemoryMock},
                    AddressingMode, CPU,
                },
            };

            #[test]
            fn should_take_four_cycles() {
                let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
                let mut cpu = CPU::new_nmos(&memory);
                cpu.program_counter = 0x02;
                cpu.cycle = 0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.cycle, 4);
            }

            #[test]
            fn should_incorrectly_fetch_target_address_when_indirect_address_is_falling_on_a_page_boundary_and_take_lo_from_correct_address_but_use_indirect_address_for_hi(
            ) {
                const INDIRECT_ADDR_LO: Byte = 0xFF;
                const INDIRECT_ADDR_HI: Byte = 0x00;
                const TARGET_ADDR_LO: Byte = 0xA5;
                const TARGET_ADDR_HI: Byte = 0xCC;
                const INCORRECT_TARGET_ADDR_HI: Byte = 0x09;

                let mut program: [Byte; 512] = [0x00; 512];
                program[0x0000] = INCORRECT_TARGET_ADDR_HI;
                program[0x0001] = INDIRECT_ADDR_LO;
                program[0x0002] = INDIRECT_ADDR_HI;
                program[0x00FF] = TARGET_ADDR_LO;
                program[0x0100] = TARGET_ADDR_HI;

                let memory = RefCell::new(MemoryMock::new(&program));
                let mut cpu = CPU::new_nmos(&memory);
                cpu.program_counter = 0x0001;
                cpu.address_output = 0x0;
                cpu.cycle = 0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.address_output, 0x09A5);
            }
        }

        #[cfg(test)]
        mod cmos {
            use std::cell::RefCell;

            use crate::{
                consts::Byte,
                cpu::{
                    addressing::get_addressing_tasks,
                    tests::{run_tasks, MemoryMock},
                    AddressingMode, CPU,
                },
            };

            #[test]
            fn should_take_five_cycles() {
                let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
                let mut cpu = CPU::new_rockwell_cmos(&memory);
                cpu.program_counter = 0x02;
                cpu.cycle = 0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.cycle, 5);
            }

            #[test]
            fn should_correctly_fetch_target_address_when_indirect_address_is_falling_on_a_page_boundary(
            ) {
                const INDIRECT_ADDR_LO: Byte = 0xFF;
                const INDIRECT_ADDR_HI: Byte = 0x00;
                const TARGET_ADDR_LO: Byte = 0xA5;
                const TARGET_ADDR_HI: Byte = 0xCC;

                let mut program: [Byte; 512] = [0x00; 512];
                program[0x0001] = INDIRECT_ADDR_LO;
                program[0x0002] = INDIRECT_ADDR_HI;
                program[0x00FF] = TARGET_ADDR_LO;
                program[0x0100] = TARGET_ADDR_HI;

                let memory = RefCell::new(MemoryMock::new(&program));
                let mut cpu = CPU::new_rockwell_cmos(&memory);
                cpu.program_counter = 0x0001;
                cpu.address_output = 0x0;
                cpu.cycle = 0;

                let tasks = get_addressing_tasks(&cpu, AddressingMode::Indirect);
                run_tasks(&mut cpu, tasks);

                assert_eq!(cpu.address_output, 0xCCA5);
            }
        }
    }

    #[cfg(test)]
    mod implicit_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_not_change_address_output() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.address_output = 0x0;
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Implicit);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x0);
        }

        #[test]
        fn should_not_advance_program_counter() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Implicit);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x00);
        }

        #[test]
        fn should_take_zero_cycles() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x02;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Implicit);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 0);
        }
    }

    #[cfg(test)]
    mod relative_addressing {
        use std::cell::RefCell;

        use crate::cpu::{
            addressing::get_addressing_tasks,
            tests::{run_tasks, MemoryMock},
            AddressingMode, CPU,
        };

        #[test]
        fn should_not_change_address_output() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x00;
            cpu.address_output = 0x0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Relative);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.address_output, 0x0);
        }

        #[test]
        fn should_not_advance_program_counter() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x00;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Relative);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.program_counter, 0x00);
        }

        #[test]
        fn should_take_zero_cycles() {
            let memory = RefCell::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]));
            let mut cpu = CPU::new_nmos(&memory);
            cpu.program_counter = 0x02;
            cpu.cycle = 0;

            let tasks = get_addressing_tasks(&cpu, AddressingMode::Relative);
            run_tasks(&mut cpu, tasks);

            assert_eq!(cpu.cycle, 0);
        }
    }
}

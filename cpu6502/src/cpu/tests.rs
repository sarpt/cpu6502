use crate::{
    consts::{Byte, Word},
    memory::Memory,
};
use std::ops::{Index, IndexMut};

pub struct MemoryMock {
    data: [u8; 512],
}
impl Memory for MemoryMock {}

impl MemoryMock {
    pub fn new(payload: &[u8]) -> Self {
        let mut mock = MemoryMock { data: [0; 512] };
        mock.data[..payload.len()].copy_from_slice(payload);

        return mock;
    }
}

impl Default for MemoryMock {
    fn default() -> Self {
        const DATA: [u8; 5] = [0x44, 0x51, 0x88, 0x42, 0x99];
        return MemoryMock::new(&DATA);
    }
}

impl Index<Word> for MemoryMock {
    type Output = Byte;

    fn index(&self, index: Word) -> &Self::Output {
        let addr: usize = index.into();
        return &self.data[addr];
    }
}

impl IndexMut<Word> for MemoryMock {
    fn index_mut(&mut self, index: Word) -> &mut Self::Output {
        let addr: usize = index.into();
        return &mut self.data[addr];
    }
}

#[cfg(test)]
mod new {
    use super::super::*;
    use super::MemoryMock;

    #[test]
    fn should_be_in_reset_state_after_creation() {
        let uut = CPU::new(Box::new(MemoryMock::default()));

        assert_eq!(uut.accumulator, 0);
        assert_eq!(uut.cycle, 0);
        assert_eq!(uut.index_register_x, 0);
        assert_eq!(uut.index_register_y, 0);
        assert_eq!(uut.stack_pointer, 0);
        assert_eq!(uut.processor_status.flags, 0);
        assert_eq!(uut.program_counter, 0xFFFC);
    }
}

#[cfg(test)]
mod reset {
    use super::super::*;
    use super::MemoryMock;

    #[test]
    fn should_set_program_counter_to_fffc_after_reset() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0xFFFF;

        uut.reset();

        assert_eq!(uut.program_counter, 0xFFFC);
    }

    #[test]
    fn should_set_negative_flag_in_processor_status_to_zero_after_reset() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b11111111;

        uut.reset();

        assert_eq!(uut.processor_status.flags, 0b11110111);
    }
}

#[cfg(test)]
mod access_memory {
    use super::MemoryMock;
    use crate::consts::Word;
    use crate::cpu::CPU;

    const ADDR: Word = 0x0003;

    #[test]
    fn should_return_a_byte() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));

        let result = uut.access_memory(ADDR);

        assert_eq!(result, 0x42);
    }
}

#[cfg(test)]
#[cfg(test)]
mod fetch_address {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_an_address_pointed_by_a_program_counter_in_little_endian() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        let result = uut.fetch_address();

        assert_eq!(result, 0xFF03);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter_twice() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        assert_eq!(uut.cycle, 0);

        uut.fetch_address();

        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.program_counter, 0x0002);
    }
}

#[cfg(test)]
mod fetch_zero_page_address {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_a_zero_page_address_pointed_by_a_program_counter_in_little_endian() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        let result = uut.fetch_zero_page_address();

        assert_eq!(result, 0x003);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter_once() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        assert_eq!(uut.cycle, 0);

        uut.fetch_zero_page_address();

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0001);
    }
}

#[cfg(test)]
mod fetch_zero_page_address_with_x_offset {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_a_zero_page_address_pointed_by_a_program_counter_summed_with_index_register_x()
    {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.index_register_x = 0x20;
        uut.program_counter = 0x00;

        let result = uut.fetch_zero_page_address_with_x_offset();

        assert_eq!(result, 0x0023);
    }

    #[test]
    fn should_increase_cycle_counter_two_times() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        assert_eq!(uut.cycle, 0);

        uut.fetch_zero_page_address_with_x_offset();

        assert_eq!(uut.cycle, 2);
    }

    #[test]
    fn should_increase_program_counter_once() {
        let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF])));
        uut.program_counter = 0x00;

        assert_eq!(uut.cycle, 0);

        uut.fetch_zero_page_address_with_x_offset();

        assert_eq!(uut.program_counter, 0x0001);
    }
}

#[cfg(test)]
mod fetch_instruction {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_return_an_instruction_pointed_by_a_program_counter() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0x0001;

        let result = uut.fetch_instruction();

        assert_eq!(result, 0x51);
    }

    #[test]
    fn should_increase_cycle_counter_and_a_program_counter() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.program_counter = 0x0001;

        assert_eq!(uut.cycle, 0);

        uut.fetch_instruction();

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.program_counter, 0x0002);
    }
}

#[cfg(test)]
mod get_register {
    use super::MemoryMock;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn should_return_accumulator() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.accumulator = 0xdf;

        let result = uut.get_register(Registers::Accumulator);

        assert_eq!(result, 0xdf);
    }

    #[test]
    fn should_return_index_register_x() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_x = 0xdf;

        let result = uut.get_register(Registers::IndexX);

        assert_eq!(result, 0xdf);
    }

    #[test]
    fn should_return_index_register_y() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_y = 0xdf;

        let result = uut.get_register(Registers::IndexY);

        assert_eq!(result, 0xdf);
    }
}

#[cfg(test)]
mod set_register {
    use super::MemoryMock;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn should_set_accumulator() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.accumulator = 0x00;

        let value = 0xf5;
        uut.set_register(Registers::Accumulator, value);

        assert_eq!(uut.accumulator, 0xf5);
    }

    #[test]
    fn should_set_index_register_x() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_x = 0x00;

        let value = 0xf5;
        uut.set_register(Registers::IndexX, value);

        assert_eq!(uut.index_register_x, 0xf5);
    }

    #[test]
    fn should_set_index_register_y() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_y = 0x00;

        let value = 0xf5;
        uut.set_register(Registers::IndexY, value);

        assert_eq!(uut.index_register_y, 0xf5);
    }
}

#[cfg(test)]
mod push_byte_to_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_push_a_byte_to_a_place_to_the_first_page_in_memory_pointed_by_a_stack_pointer() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0xFF;

        let value: u8 = 0xDF;
        uut.push_byte_to_stack(value);

        assert_eq!(uut.memory[0x01FF], 0xDF);
    }

    #[test]
    fn should_increase_cycle_counter_and_decrease_stack_pointer_by_one() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0xFF;

        assert_eq!(uut.cycle, 0);

        let value: u8 = 0xDF;
        uut.push_byte_to_stack(value);

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.stack_pointer, 0xFE);
    }
}

#[cfg(test)]
mod push_word_to_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_push_a_byte_to_a_place_to_the_first_page_in_memory_pointed_by_a_stack_pointer() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0xFF;

        let value: u16 = 0x56DF;
        uut.push_word_to_stack(value);

        assert_eq!(uut.memory[0x01FF], 0xDF);
        assert_eq!(uut.memory[0x01FE], 0x56);
    }

    #[test]
    fn should_increase_cycle_counter_and_decrease_stack_pointer_by_two() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.stack_pointer = 0xFF;
        assert_eq!(uut.cycle, 0);

        let value: u16 = 0x56DF;
        uut.push_word_to_stack(value);

        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.stack_pointer, 0xFD);
    }
}

#[cfg(test)]
mod pop_byte_from_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_pop_byte_from_stack() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.memory[0x01FF] = 0xDF;
        uut.memory[0x01FE] = 0x48;
        uut.stack_pointer = 0xFD;

        let value = uut.pop_byte_from_stack();

        assert_eq!(value, 0x48);
    }

    #[test]
    fn should_increment_cycle_count_and_stack_pointer_once() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.memory[0x01FF] = 0xDF;
        uut.memory[0x01FE] = 0x48;
        uut.stack_pointer = 0xFD;

        assert_eq!(uut.cycle, 0);

        uut.pop_byte_from_stack();

        assert_eq!(uut.cycle, 1);
        assert_eq!(uut.stack_pointer, 0xFE);
    }
}

#[cfg(test)]
mod pop_word_from_stack {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_pop_word_from_stack() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.memory[0x01FF] = 0xDF;
        uut.memory[0x01FE] = 0x48;
        uut.stack_pointer = 0xFD;

        let val = uut.pop_word_from_stack();

        assert_eq!(val, 0xDF48);
    }

    #[test]
    fn should_increment_cycle_count_and_stack_pointer_twice() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.memory[0x01FF] = 0xDF;
        uut.memory[0x01FE] = 0x48;
        uut.stack_pointer = 0xFD;
        assert_eq!(uut.cycle, 0);

        uut.pop_word_from_stack();

        assert_eq!(uut.cycle, 2);
        assert_eq!(uut.stack_pointer, 0xFF);
    }
}

#[cfg(test)]
mod sum_with_x {
    use super::MemoryMock;
    use crate::cpu::CPU;

    #[test]
    fn should_sum_provided_value_with_x_register_contents() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_x = 0x02;

        let value: u8 = 0x03;
        let result = uut.sum_with_x(value);

        assert_eq!(result, 0x05);
    }

    #[test]
    fn sum_should_wrap_around_byte() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_x = 0xFF;

        let value: u8 = 0x03;
        let result = uut.sum_with_x(value);

        assert_eq!(result, 0x02);
    }

    #[test]
    fn should_increase_cycle_counter_by_one() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.index_register_x = 0xFF;
        assert_eq!(uut.cycle, 0);

        let value: u8 = 0x03;
        uut.sum_with_x(value);

        assert_eq!(uut.cycle, 1);
    }
}

#[cfg(test)]
mod set_load_status {
    use super::MemoryMock;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn should_set_zero_flag_on_processor_status_when_register_is_zero() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0x00;

        let register = Registers::Accumulator;
        uut.set_load_status(register);

        assert_eq!(uut.processor_status.flags, 0b00000010);
    }

    #[test]
    fn should_unset_zero_flag_on_processor_status_when_register_is_not_zero() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b11111111;
        uut.accumulator = 0xFF;

        let register = Registers::Accumulator;
        uut.set_load_status(register);

        assert_eq!(uut.processor_status.flags, 0b11111101);
    }

    #[test]
    fn should_set_negative_flag_on_processor_status_when_register_has_bit_7_set() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0x80;

        let register = Registers::Accumulator;
        uut.set_load_status(register);

        assert_eq!(uut.processor_status.flags, 0b10000000);
    }

    #[test]
    fn should_unset_negative_flag_on_processor_status_when_register_has_bit_7_unset() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b11111111;
        uut.accumulator = 0x00;

        let register = Registers::Accumulator;
        uut.set_load_status(register);

        assert_eq!(uut.processor_status.flags, 0b01111111);
    }
}

#[cfg(test)]
mod set_cmp_status {
    use super::MemoryMock;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn should_set_zero_flag_on_processor_status_when_register_is_the_same_as_provided_value() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0xd3;

        let value = 0xd3;
        let register = Registers::Accumulator;
        uut.set_cmp_status(register, value);

        assert_eq!(uut.processor_status.get_zero_flag(), true);
    }

    #[test]
    fn should_clear_zero_flag_on_processor_status_when_register_is_different_as_provided_value() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000010;
        uut.accumulator = 0xd5;

        let value = 0xd3;
        let register = Registers::Accumulator;
        uut.set_cmp_status(register, value);

        assert_eq!(uut.processor_status.get_zero_flag(), false);
    }

    #[test]
    fn should_set_carry_flag_on_processor_status_when_register_is_the_same_as_provided_value() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0xd3;

        let value = 0xd3;
        let register = Registers::Accumulator;
        uut.set_cmp_status(register, value);

        assert_eq!(uut.processor_status.get_carry_flag(), true);
    }

    #[test]
    fn should_set_carry_flag_on_processor_status_when_register_is_bigger_than_provided_value() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0xd5;

        let value = 0xd3;
        let register = Registers::Accumulator;
        uut.set_cmp_status(register, value);

        assert_eq!(uut.processor_status.get_carry_flag(), true);
    }

    #[test]
    fn should_clear_zero_flag_on_processor_status_when_register_is_smaller_than_provided_value() {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000001;
        uut.accumulator = 0x01;

        let value = 0xd3;
        let register = Registers::Accumulator;
        uut.set_cmp_status(register, value);

        assert_eq!(uut.processor_status.get_carry_flag(), false);
    }

    #[test]
    fn should_set_negative_flag_on_processor_status_when_difference_with_provided_value_has_most_significant_byte_set(
    ) {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000000;
        uut.accumulator = 0xd3;

        let value = 0xd5;
        let register = Registers::Accumulator;
        uut.set_cmp_status(register, value);

        assert_eq!(uut.processor_status.get_negative_flag(), true);
    }

    #[test]
    fn should_clear_negative_flag_on_processor_status_when_difference_with_provided_value_has_most_significant_byte_clear(
    ) {
        let mut uut = CPU::new(Box::new(MemoryMock::default()));
        uut.processor_status.flags = 0b00000010;
        uut.accumulator = 0xd5;

        let value = 0xd3;
        let register = Registers::Accumulator;
        uut.set_cmp_status(register, value);

        assert_eq!(uut.processor_status.get_negative_flag(), false);
    }
}

#[cfg(test)]
mod get_address {

    #[cfg(test)]
    mod immediate_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_program_counter_address() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0xCB;

            let result = uut.get_address(AddressingMode::Immediate, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0xCB);
        }

        #[test]
        fn should_not_change_program_counter() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0xCB;

            uut.get_address(AddressingMode::Immediate, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0xCB);
        }

        #[test]
        fn should_not_take_any_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0xCB;
            uut.cycle = 0;

            uut.get_address(AddressingMode::Immediate, MemoryOperation::Read);

            assert_eq!(uut.cycle, 0);
        }
    }

    #[cfg(test)]
    mod absolute_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_address_from_next_word_in_memory_relative_to_program_counter() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x01;

            let result = uut.get_address(AddressingMode::Absolute, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0xCBFF);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x01;

            uut.get_address(AddressingMode::Absolute, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x01;
            uut.cycle = 0;

            uut.get_address(AddressingMode::Absolute, MemoryOperation::Read);

            assert_eq!(uut.cycle, 2);
        }
    }

    #[cfg(test)]
    mod absolute_x_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_address_offset_by_index_register_x_from_next_word_in_memory_relative_to_program_counter(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x01;

            let result = uut.get_address(AddressingMode::AbsoluteX, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0x52CC);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x01;

            uut.get_address(AddressingMode::AbsoluteX, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x04);
        }

        #[test]
        fn should_take_three_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x01;
            uut.cycle = 0;

            uut.get_address(AddressingMode::AbsoluteX, MemoryOperation::Read);

            assert_eq!(uut.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0xFF;
            uut.cycle = 0;

            uut.get_address(AddressingMode::AbsoluteX, MemoryOperation::Read);

            assert_eq!(uut.cycle, 4);
        }

        #[test]
        fn should_take_four_cycles_for_a_modification_operation_address() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x01;
            uut.cycle = 0;

            uut.get_address(AddressingMode::AbsoluteX, MemoryOperation::Modify);

            assert_eq!(uut.cycle, 4);
        }

        #[test]
        fn should_take_four_cycles_for_a_write_operation_address() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x01;
            uut.cycle = 0;

            uut.get_address(AddressingMode::AbsoluteX, MemoryOperation::Write);

            assert_eq!(uut.cycle, 4);
        }
    }

    #[cfg(test)]
    mod absolute_y_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_address_offset_by_index_register_y_from_next_word_in_memory_relative_to_program_counter(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.index_register_y = 0x01;
            uut.program_counter = 0x02;

            let result = uut.get_address(AddressingMode::AbsoluteY, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0x52CC);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.index_register_y = 0x01;
            uut.program_counter = 0x02;

            uut.get_address(AddressingMode::AbsoluteY, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x04);
        }

        #[test]
        fn should_take_three_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_y = 0x01;
            uut.cycle = 0;

            uut.get_address(AddressingMode::AbsoluteY, MemoryOperation::Read);

            assert_eq!(uut.cycle, 3);
        }

        #[test]
        fn should_take_four_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_y = 0xFF;
            uut.cycle = 0;

            uut.get_address(AddressingMode::AbsoluteY, MemoryOperation::Read);

            assert_eq!(uut.cycle, 4);
        }

        #[test]
        fn should_take_four_cycles_for_a_modification_operation_address() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_y = 0x01;
            uut.cycle = 0;

            uut.get_address(AddressingMode::AbsoluteY, MemoryOperation::Modify);

            assert_eq!(uut.cycle, 4);
        }

        #[test]
        fn should_take_four_cycles_for_a_write_operation_address() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_y = 0x01;
            uut.cycle = 0;

            uut.get_address(AddressingMode::AbsoluteY, MemoryOperation::Write);

            assert_eq!(uut.cycle, 4);
        }
    }

    #[cfg(test)]
    mod zero_page_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter()
        {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;

            let result = uut.get_address(AddressingMode::ZeroPage, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0x00CB);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;

            uut.get_address(AddressingMode::ZeroPage, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x03);
        }

        #[test]
        fn should_take_one_cycle() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.cycle = 0;

            uut.get_address(AddressingMode::ZeroPage, MemoryOperation::Read);

            assert_eq!(uut.cycle, 1);
        }
    }

    #[cfg(test)]
    mod zero_page_x_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_x(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x03;

            let result = uut.get_address(AddressingMode::ZeroPageX, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0x00CE);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x03;

            uut.get_address(AddressingMode::ZeroPageX, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_x = 0x03;
            uut.cycle = 0;

            uut.get_address(AddressingMode::ZeroPageX, MemoryOperation::Read);

            assert_eq!(uut.cycle, 2);
        }
    }

    #[cfg(test)]
    mod zero_page_y_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_address_in_zero_page_from_next_byte_in_memory_relative_to_program_counter_summed_with_index_register_y(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x03;
            uut.index_register_y = 0x03;

            let result = uut.get_address(AddressingMode::ZeroPageY, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0x0055);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_y = 0x03;

            uut.get_address(AddressingMode::ZeroPageY, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x03);
        }

        #[test]
        fn should_take_two_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x03, 0xFF, 0xCB, 0x52])));
            uut.program_counter = 0x02;
            uut.index_register_y = 0x03;
            uut.cycle = 0;

            uut.get_address(AddressingMode::ZeroPageY, MemoryOperation::Read);

            assert_eq!(uut.cycle, 2);
        }
    }

    #[cfg(test)]
    mod index_indirect_x_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_address_stored_in_place_pointed_by_zero_page_address_in_next_byte_relative_to_program_counter_summed_with_index_register_x(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;
            uut.index_register_x = 0x01;

            let result = uut.get_address(AddressingMode::IndexIndirectX, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0xDD03);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;
            uut.index_register_x = 0x01;

            uut.get_address(AddressingMode::IndexIndirectX, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x01);
        }

        #[test]
        fn should_take_four_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x01, 0xFF, 0x03, 0xDD, 0x25])));
            uut.program_counter = 0x00;
            uut.index_register_x = 0x01;
            uut.cycle = 0;

            uut.get_address(AddressingMode::IndexIndirectX, MemoryOperation::Read);

            assert_eq!(uut.cycle, 4);
        }
    }

    #[cfg(test)]
    mod indirect_index_y_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_address_offset_by_index_register_y_which_is_stored_at_zero_page_address() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25])));
            uut.index_register_y = 0x02;
            uut.program_counter = 0x00;

            let result = uut.get_address(AddressingMode::IndirectIndexY, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0xDD05);
        }

        #[test]
        fn should_advance_program_counter_once() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25])));
            uut.index_register_y = 0x02;
            uut.program_counter = 0x00;

            uut.get_address(AddressingMode::IndirectIndexY, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x01);
        }

        #[test]
        fn should_take_four_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25])));
            uut.index_register_y = 0x02;
            uut.program_counter = 0x00;
            uut.cycle = 0;

            uut.get_address(AddressingMode::IndirectIndexY, MemoryOperation::Read);

            assert_eq!(uut.cycle, 4);
        }

        #[test]
        fn should_take_five_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25])));
            uut.index_register_y = 0xFF;
            uut.program_counter = 0x00;
            uut.cycle = 0;

            uut.get_address(AddressingMode::IndirectIndexY, MemoryOperation::Read);

            assert_eq!(uut.cycle, 5);
        }
    }

    #[cfg(test)]
    mod indirect_addressing {
        use super::super::MemoryMock;
        use crate::{
            consts::Byte,
            cpu::{AddressingMode, MemoryOperation, CPU},
        };

        #[test]
        fn should_return_address_from_place_in_memory_stored_in_next_word_relative_to_program_counter(
        ) {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x00;

            let result = uut.get_address(AddressingMode::Indirect, MemoryOperation::Read);

            assert_eq!(result.unwrap(), 0x0001);
        }

        #[test]
        fn should_advance_program_counter_twice() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x00;

            uut.get_address(AddressingMode::Indirect, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x02);
        }

        #[test]
        fn should_take_four_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x02;
            uut.cycle = 0;

            uut.get_address(AddressingMode::Indirect, MemoryOperation::Read);

            assert_eq!(uut.cycle, 4);
        }

        #[test]
        fn should_incorrectly_fetch_fetch_target_address_when_indirect_address_is_falling_on_a_page_boundary_and_take_lo_from_correct_address_but_use_indirect_address_for_hi(
        ) {
            const INDIRECT_ADDR_LO: Byte = 0xFF;
            const INDIRECT_ADDR_HI: Byte = 0x00;
            const TARGET_ADDR_LO: Byte = 0xA5;
            const TARGET_ADDR_HI: Byte = 0xCC;
            const INCORRECT_TARGET_ADDR_HI: Byte = 0x09;

            let mut memory: [Byte; 512] = [0x00; 512];
            memory[0x0000] = INCORRECT_TARGET_ADDR_HI;
            memory[0x0001] = INDIRECT_ADDR_LO;
            memory[0x0002] = INDIRECT_ADDR_HI;
            memory[0x00FF] = TARGET_ADDR_LO;
            memory[0x0100] = TARGET_ADDR_HI;

            let mut uut = CPU::new(Box::new(MemoryMock::new(&memory)));
            uut.program_counter = 0x0001;
            uut.cycle = 0;

            let result = uut.get_address(AddressingMode::Indirect, MemoryOperation::Read);

            assert_eq!(result, Some(0x09A5));
        }
    }

    #[cfg(test)]
    mod implicit_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_none() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x00;

            let result = uut.get_address(AddressingMode::Implicit, MemoryOperation::Read);

            assert_eq!(result.is_none(), true);
        }

        #[test]
        fn should_not_advance_program_counter() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x00;

            uut.get_address(AddressingMode::Implicit, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x00);
        }

        #[test]
        fn should_take_zero_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x02;
            uut.cycle = 0;

            uut.get_address(AddressingMode::Implicit, MemoryOperation::Read);

            assert_eq!(uut.cycle, 0);
        }
    }

    #[cfg(test)]
    mod relative_addressing {
        use super::super::MemoryMock;
        use crate::cpu::{AddressingMode, MemoryOperation, CPU};

        #[test]
        fn should_return_none() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x00;

            let result = uut.get_address(AddressingMode::Relative, MemoryOperation::Read);

            assert_eq!(result.is_none(), true);
        }

        #[test]
        fn should_not_advance_program_counter() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x00;

            uut.get_address(AddressingMode::Relative, MemoryOperation::Read);

            assert_eq!(uut.program_counter, 0x00);
        }

        #[test]
        fn should_take_zero_cycles() {
            let mut uut = CPU::new(Box::new(MemoryMock::new(&[0x02, 0x00, 0x01, 0x00])));
            uut.program_counter = 0x02;
            uut.cycle = 0;

            uut.get_address(AddressingMode::Relative, MemoryOperation::Read);

            assert_eq!(uut.cycle, 0);
        }
    }
}

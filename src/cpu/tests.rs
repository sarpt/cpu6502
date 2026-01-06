use crate::{
  consts::{Byte, Word},
  memory::Memory,
};
use std::ops::{Index, IndexMut};

pub struct MemoryMock {
  data: [u8; 64 * 1024],
}
impl Memory for MemoryMock {}

impl MemoryMock {
  pub fn new(payload: &[u8]) -> Self {
    let mut mock = MemoryMock {
      data: [0; 64 * 1024],
    };
    mock.data[..payload.len()].copy_from_slice(payload);

    mock
  }
}

impl Default for MemoryMock {
  fn default() -> Self {
    const DATA: [u8; 5] = [0x44, 0x51, 0x88, 0x42, 0x99];
    MemoryMock::new(&DATA)
  }
}

impl Index<Word> for MemoryMock {
  type Output = Byte;

  fn index(&self, index: Word) -> &Self::Output {
    let addr: usize = index.into();
    &self.data[addr]
  }
}

impl IndexMut<Word> for MemoryMock {
  fn index_mut(&mut self, index: Word) -> &mut Self::Output {
    let addr: usize = index.into();
    &mut self.data[addr]
  }
}

#[cfg(test)]
mod new {
  use super::super::*;

  #[test]
  fn should_be_in_reset_state_after_creation() {
    let uut = CPU::new_nmos();

    assert_eq!(uut.accumulator, 0);
    assert_eq!(uut.cycle, 0);
    assert_eq!(uut.index_register_x, 0);
    assert_eq!(uut.index_register_y, 0);
    assert_eq!(uut.stack_pointer, 0);
    assert_eq!(uut.processor_status, 0);
    assert_eq!(uut.program_counter, 0xFFFC);
  }
}

#[cfg(test)]
mod reset {
  use super::super::*;
  use super::MemoryMock;

  #[test]
  fn should_set_program_counter_to_address_found_at_fffc_after_reset() {
    const RESET_VECTOR_HI: Byte = 0x00;
    const RESET_VECTOR_LO: Byte = 0xAD;

    let mut memory = MemoryMock::default();
    memory[0xFFFC] = RESET_VECTOR_LO;
    memory[0xFFFD] = RESET_VECTOR_HI;
    let mut uut = CPU::new_nmos();
    uut.program_counter = 0xFFFF;

    uut.reset(&memory);

    assert_eq!(uut.program_counter, 0x00AD);
  }

  #[test]
  fn should_set_negative_flag_in_processor_status_to_zero_after_reset() {
    let memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b11111111);

    uut.reset(&memory);

    assert_eq!(uut.processor_status, 0b11110111);
  }
}

#[cfg(test)]
mod fetch_instruction {
  use super::MemoryMock;
  use crate::cpu::CPU;

  #[test]
  fn should_return_an_instruction_pointed_by_a_program_counter() {
    let memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();
    uut.program_counter = 0x0001;

    let (opcode_result, opcode_addr) = uut.fetch_opcode(&memory);

    assert_eq!(opcode_result, 0x51);
    assert_eq!(opcode_addr, 0x0001);
  }

  #[test]
  fn should_increase_cycle_counter_and_a_program_counter() {
    let memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();
    uut.program_counter = 0x0001;

    assert_eq!(uut.cycle, 0);

    uut.fetch_opcode(&memory);

    assert_eq!(uut.cycle, 1);
    assert_eq!(uut.program_counter, 0x0002);
  }
}

#[cfg(test)]
mod get_register {
  use crate::cpu::{Registers, CPU};

  #[test]
  fn should_return_accumulator() {
    let mut uut = CPU::new_nmos();
    uut.accumulator = 0xdf;

    let result = uut.get_register(Registers::Accumulator);

    assert_eq!(result, 0xdf);
  }

  #[test]
  fn should_return_index_register_x() {
    let mut uut = CPU::new_nmos();
    uut.index_register_x = 0xdf;

    let result = uut.get_register(Registers::IndexX);

    assert_eq!(result, 0xdf);
  }

  #[test]
  fn should_return_index_register_y() {
    let mut uut = CPU::new_nmos();
    uut.index_register_y = 0xdf;

    let result = uut.get_register(Registers::IndexY);

    assert_eq!(result, 0xdf);
  }
}

#[cfg(test)]
mod set_register {
  use crate::cpu::{Registers, CPU};

  #[test]
  fn should_set_accumulator() {
    let mut uut = CPU::new_nmos();
    uut.accumulator = 0x00;

    let value = 0xF5;
    uut.set_register(Registers::Accumulator, value);

    assert_eq!(uut.accumulator, 0xF5);
  }

  #[test]
  fn should_set_index_register_x() {
    let mut uut = CPU::new_nmos();
    uut.index_register_x = 0x00;

    let value = 0xF5;
    uut.set_register(Registers::IndexX, value);

    assert_eq!(uut.index_register_x, 0xF5);
  }

  #[test]
  fn should_set_index_register_y() {
    let mut uut = CPU::new_nmos();
    uut.index_register_y = 0x00;

    let value = 0xF5;
    uut.set_register(Registers::IndexY, value);

    assert_eq!(uut.index_register_y, 0xF5);
  }

  #[test]
  fn should_set_stack_pointer() {
    let mut uut = CPU::new_nmos();
    uut.stack_pointer = 0x00;

    let value = 0xF5;
    uut.set_register(Registers::StackPointer, value);

    assert_eq!(uut.stack_pointer, 0xF5);
  }

  #[test]
  fn should_set_processor_status() {
    let mut uut = CPU::new_nmos();
    uut.processor_status = 0x00_u8.into();

    let value = 0xF5;
    uut.set_register(Registers::ProcessorStatus, value);

    assert_eq!(uut.processor_status, 0xF5);
  }

  #[test]
  fn should_set_processor_status_when_provided_accumulator_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status = 0x00_u8.into();

    let value = 0xF5;
    uut.set_register(Registers::Accumulator, value);

    assert_eq!(uut.processor_status, 0b10000000);
  }

  #[test]
  fn should_set_processor_status_when_provided_index_register_x_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status = 0x00_u8.into();

    let value = 0xF5;
    uut.set_register(Registers::IndexX, value);

    assert_eq!(uut.processor_status, 0b10000000);
  }

  #[test]
  fn should_set_processor_status_when_provided_index_register_y_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status = 0x00_u8.into();

    let value = 0xF5;
    uut.set_register(Registers::IndexY, value);

    assert_eq!(uut.processor_status, 0b10000000);
  }

  #[test]
  fn should_not_set_processor_status_when_provided_stack_pointer_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status = 0x00_u8.into();

    let value = 0xF5;
    uut.set_register(Registers::StackPointer, value);

    assert_eq!(uut.processor_status, 0x00);
  }
}

#[cfg(test)]
mod push_byte_to_stack {
  use super::MemoryMock;
  use crate::cpu::CPU;

  #[test]
  fn should_push_a_byte_to_a_place_to_the_first_page_in_memory_pointed_by_a_stack_pointer() {
    let mut memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();
    uut.stack_pointer = 0xFF;

    let value: u8 = 0xDF;
    uut.push_byte_to_stack(value, &mut memory);

    assert_eq!(memory[0x01FF], 0xDF);
  }

  #[test]
  fn should_decrease_stack_pointer_by_one() {
    let mut memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();
    uut.stack_pointer = 0xFF;

    let value: u8 = 0xDF;
    uut.push_byte_to_stack(value, &mut memory);

    assert_eq!(uut.stack_pointer, 0xFE);
  }
}

#[cfg(test)]
mod pop_byte_from_stack {
  use super::MemoryMock;
  use crate::cpu::CPU;

  #[test]
  fn should_pop_byte_from_stack() {
    let mut memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();
    memory[0x01FF] = 0xDF;
    memory[0x01FE] = 0x48;
    uut.stack_pointer = 0xFD;

    let value = uut.pop_byte_from_stack(&memory);

    assert_eq!(value, 0x48);
  }

  #[test]
  fn should_increment_stack_pointer_once() {
    let mut memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();
    memory[0x01FF] = 0x00;
    memory[0x01FE] = 0x00;
    uut.stack_pointer = 0xFD;

    uut.pop_byte_from_stack(&memory);

    assert_eq!(uut.stack_pointer, 0xFE);
  }
}

#[cfg(test)]
mod set_status_of_register {
  use crate::cpu::{Registers, CPU};

  #[test]
  fn should_set_zero_flag_on_processor_status_when_register_is_zero() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000000);
    uut.accumulator = 0x00;

    let register = Registers::Accumulator;
    uut.set_status_of_register(register);

    assert_eq!(uut.processor_status, 0b00000010);
  }

  #[test]
  fn should_unset_zero_flag_on_processor_status_when_register_is_not_zero() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b11111111);
    uut.accumulator = 0xFF;

    let register = Registers::Accumulator;
    uut.set_status_of_register(register);

    assert_eq!(uut.processor_status, 0b11111101);
  }

  #[test]
  fn should_set_negative_flag_on_processor_status_when_register_has_bit_7_set() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000000);
    uut.accumulator = 0x80;

    let register = Registers::Accumulator;
    uut.set_status_of_register(register);

    assert_eq!(uut.processor_status, 0b10000000);
  }

  #[test]
  fn should_unset_negative_flag_on_processor_status_when_register_has_bit_7_unset() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b11111111);
    uut.accumulator = 0x00;

    let register = Registers::Accumulator;
    uut.set_status_of_register(register);

    assert_eq!(uut.processor_status, 0b01111111);
  }
}

#[cfg(test)]
mod set_cmp_status {
  use crate::cpu::{Registers, CPU};

  #[test]
  fn should_set_zero_flag_on_processor_status_when_register_is_the_same_as_provided_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000000);
    uut.accumulator = 0xd3;

    let value = 0xd3;
    let register = Registers::Accumulator;
    uut.set_cmp_status(register, value);

    assert!(uut.processor_status.get_zero_flag());
  }

  #[test]
  fn should_clear_zero_flag_on_processor_status_when_register_is_different_as_provided_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000010);
    uut.accumulator = 0xd5;

    let value = 0xd3;
    let register = Registers::Accumulator;
    uut.set_cmp_status(register, value);

    assert!(!uut.processor_status.get_zero_flag());
  }

  #[test]
  fn should_change_carry_flag_on_processor_status_when_register_is_the_same_as_provided_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000000);
    uut.accumulator = 0xd3;

    let value = 0xd3;
    let register = Registers::Accumulator;
    uut.set_cmp_status(register, value);

    assert!(uut.processor_status.get_carry_flag());
  }

  #[test]
  fn should_change_carry_flag_on_processor_status_when_register_is_bigger_than_provided_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000000);
    uut.accumulator = 0xd5;

    let value = 0xd3;
    let register = Registers::Accumulator;
    uut.set_cmp_status(register, value);

    assert!(uut.processor_status.get_carry_flag());
  }

  #[test]
  fn should_clear_zero_flag_on_processor_status_when_register_is_smaller_than_provided_value() {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000001);
    uut.accumulator = 0x01;

    let value = 0xd3;
    let register = Registers::Accumulator;
    uut.set_cmp_status(register, value);

    assert!(!uut.processor_status.get_carry_flag());
  }

  #[test]
  fn should_set_negative_flag_on_processor_status_when_difference_with_provided_value_has_most_significant_byte_set(
  ) {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000000);
    uut.accumulator = 0xd3;

    let value = 0xd5;
    let register = Registers::Accumulator;
    uut.set_cmp_status(register, value);

    assert!(uut.processor_status.get_negative_flag());
  }

  #[test]
  fn should_clear_negative_flag_on_processor_status_when_difference_with_provided_value_has_most_significant_byte_clear(
  ) {
    let mut uut = CPU::new_nmos();
    uut.processor_status.set(0b00000010);
    uut.accumulator = 0xd5;

    let value = 0xd3;
    let register = Registers::Accumulator;
    uut.set_cmp_status(register, value);

    assert!(!uut.processor_status.get_negative_flag());
  }
}

#[cfg(test)]
mod sync {
  use crate::cpu::tests::MemoryMock;

  use super::super::*;

  #[test]
  fn should_be_true_during_opcode_fetching_cycle() {
    let mut memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();

    assert!(!uut.sync());

    uut.tick(&mut memory);

    assert!(uut.sync());
  }

  #[test]
  fn should_be_false_after_opcode_fetching_cycle() {
    let mut memory = MemoryMock::default();
    let mut uut = CPU::new_nmos();

    assert!(!uut.sync());

    uut.tick(&mut memory);

    assert!(uut.sync());

    uut.tick(&mut memory);

    assert!(!uut.sync());
  }
}

#[cfg(test)]
pub fn run_tasks(cpu: &mut super::CPU, tasks: &mut dyn super::Tasks, memory: &mut dyn Memory) {
  while !tasks.done() {
    _ = tasks.tick(cpu, memory);
    cpu.cycle += 1;
  }
}

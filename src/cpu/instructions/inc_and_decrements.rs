use crate::cpu::{
  AddressingMode, CPU, Registers, Tasks,
  addressing::get_addressing_tasks,
  tasks::{modify_memory::ModifyMemoryTasks, modify_register::ModifyRegisterTasks},
};

fn decrement_memory(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
  let addr_tasks = get_addressing_tasks(cpu, addr_mode);
  Box::new(ModifyMemoryTasks::new_dec(addr_tasks))
}

fn decrement_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
  match register {
    Registers::IndexX | Registers::IndexY => Box::new(ModifyRegisterTasks::new_dec(register)),
    _ => panic!("decrement_register used with incorrect register"),
  }
}

pub fn dec_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  decrement_memory(cpu, AddressingMode::ZeroPage)
}

pub fn dec_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
  decrement_memory(cpu, AddressingMode::ZeroPageX)
}

pub fn dec_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  decrement_memory(cpu, AddressingMode::Absolute)
}

pub fn dec_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
  decrement_memory(cpu, AddressingMode::AbsoluteX)
}

pub fn dex_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  decrement_register(cpu, Registers::IndexX)
}

pub fn dey_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  decrement_register(cpu, Registers::IndexY)
}

fn increment_memory(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
  let addr_tasks = get_addressing_tasks(cpu, addr_mode);
  Box::new(ModifyMemoryTasks::new_inc(addr_tasks))
}

fn increment_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
  match register {
    Registers::IndexX | Registers::IndexY => Box::new(ModifyRegisterTasks::new_inc(register)),
    _ => panic!("increment_register used with incorrect register"),
  }
}

pub fn inc_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
  increment_memory(cpu, AddressingMode::ZeroPage)
}

pub fn inc_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
  increment_memory(cpu, AddressingMode::ZeroPageX)
}

pub fn inc_a(cpu: &mut CPU) -> Box<dyn Tasks> {
  increment_memory(cpu, AddressingMode::Absolute)
}

pub fn inc_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
  increment_memory(cpu, AddressingMode::AbsoluteX)
}

pub fn inx_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  increment_register(cpu, Registers::IndexX)
}

pub fn iny_im(cpu: &mut CPU) -> Box<dyn Tasks> {
  increment_register(cpu, Registers::IndexY)
}

#[cfg(test)]
mod inx_im {
  use crate::cpu::{
    CPU,
    instructions::inx_im,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_increment_x_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0x02;

    let mut tasks = inx_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.index_register_x, 0x03);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0x02;
    cpu.cycle = 0;

    let mut tasks = inx_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_processor_status_of_x_register_after_increment() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0xFF;

    let mut tasks = inx_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod iny_im {
  use crate::cpu::{
    CPU,
    instructions::iny_im,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_increment_y_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0x02;

    let mut tasks = iny_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.index_register_y, 0x03);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0x02;
    cpu.cycle = 0;

    let mut tasks = iny_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_processor_status_of_x_register_after_increment() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0xFF;

    let mut tasks = iny_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod inc_zp {
  use crate::cpu::{
    Byte, CPU, Word,
    instructions::inc_zp,
    tests::{MemoryMock, run_tasks},
  };

  const VALUE: Byte = 0x02;
  const ZERO_PAGE_ADDR: Byte = 0x03;

  #[test]
  fn should_increment_value_stored_in_memory_at_zero_page_address() {
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = inc_zp(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0x03);
  }

  #[test]
  fn should_take_four_cycles() {
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let mut tasks = inc_zp(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 4);
  }

  #[test]
  fn should_set_processor_status_of_value_in_memory() {
    const VALUE: Byte = 0xFF;
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = inc_zp(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod inc_zpx {
  use crate::cpu::{
    Byte, CPU, Word,
    instructions::inc_zpx,
    tests::{MemoryMock, run_tasks},
  };

  const VALUE: Byte = 0x09;
  const ZERO_PAGE_ADDR: Byte = 0x01;
  const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

  #[test]
  fn should_increment_value_stored_in_memory_at_zero_page_address_summed_with_index_register_x() {
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = 0x02;

    let mut tasks = inc_zpx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[ZERO_PAGE_ADDR_SUM_X as Word], 0x0A);
  }

  #[test]
  fn should_take_five_cycles() {
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = 0x02;
    cpu.cycle = 0;

    let mut tasks = inc_zpx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 5);
  }

  #[test]
  fn should_set_processor_status_of_value_in_memory() {
    const VALUE: Byte = 0xFF;
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = 0x02;

    let mut tasks = inc_zpx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod inc_a {
  use crate::cpu::{
    Byte, CPU, Word,
    instructions::inc_a,
    tests::{MemoryMock, run_tasks},
  };

  const VALUE: Byte = 0x09;
  const ADDR_LO: Byte = 0x04;
  const ADDR_HI: Byte = 0x00;
  const ADDR: Word = 0x0004;

  #[test]
  fn should_increment_value_stored_in_memory_at_absolute_address() {
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = inc_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[ADDR as Word], 0x0A);
  }

  #[test]
  fn should_take_five_cycles() {
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let mut tasks = inc_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 5);
  }

  #[test]
  fn should_set_processor_status_of_value_in_memory() {
    const VALUE: Byte = 0xFF;
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = inc_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod inc_ax {
  use crate::cpu::{
    Byte, CPU, Word,
    instructions::inc_ax,
    tests::{MemoryMock, run_tasks},
  };

  const VALUE: Byte = 0x09;
  const ADDR_LO: Byte = 0x02;
  const ADDR_HI: Byte = 0x00;
  const OFFSET: Byte = 0x02;
  const ADDR_OFFSET_BY_X: Word = 0x0004;

  #[test]
  fn should_increment_value_stored_in_memory_at_absolute_address_offset_by_index_register_x() {
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = OFFSET;

    let mut tasks = inc_ax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[ADDR_OFFSET_BY_X], 0x0A);
  }

  #[test]
  fn should_take_six_cycles() {
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = OFFSET;
    cpu.cycle = 0;

    let mut tasks = inc_ax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 6);
  }

  #[test]
  fn should_set_processor_status_of_value_in_memory() {
    const VALUE: Byte = 0xFF;
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = OFFSET;

    let mut tasks = inc_ax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod dex_im {
  use crate::cpu::{
    CPU,
    instructions::dex_im,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_decrement_x_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0x02;

    let mut tasks = dex_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.index_register_x, 0x01);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0x02;
    cpu.cycle = 0;

    let mut tasks = dex_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_processor_status_of_x_register_after_decrement() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0x01;

    let mut tasks = dex_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod dey_im {
  use crate::cpu::{
    CPU,
    instructions::dey_im,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_decrement_y_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0x02;

    let mut tasks = dey_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.index_register_y, 0x01);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0x02;
    cpu.cycle = 0;

    let mut tasks = dey_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_processor_status_of_y_register_after_decrement() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0x01;

    let mut tasks = dey_im(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod dec_zp {
  use crate::cpu::{
    Byte, CPU, Word,
    instructions::dec_zp,
    tests::{MemoryMock, run_tasks},
  };

  const VALUE: Byte = 0x02;
  const ZERO_PAGE_ADDR: Byte = 0x03;

  #[test]
  fn should_decrement_value_stored_in_memory_at_zero_page_address() {
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = dec_zp(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[ZERO_PAGE_ADDR as Word], 0x01);
  }

  #[test]
  fn should_take_four_cycles() {
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let mut tasks = dec_zp(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 4);
  }

  #[test]
  fn should_set_processor_status_of_value_in_memory() {
    const VALUE: Byte = 0x01;
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = dec_zp(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod dec_zpx {
  use crate::cpu::{
    Byte, CPU, Word,
    instructions::dec_zpx,
    tests::{MemoryMock, run_tasks},
  };

  const VALUE: Byte = 0x09;
  const ZERO_PAGE_ADDR: Byte = 0x01;
  const ZERO_PAGE_ADDR_SUM_X: Word = 0x03;

  #[test]
  fn should_decrement_value_stored_in_memory_at_zero_page_address_summed_with_index_register_x() {
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = 0x02;

    let mut tasks = dec_zpx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[ZERO_PAGE_ADDR_SUM_X as Word], 0x08);
  }

  #[test]
  fn should_take_five_cycles() {
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = 0x02;
    cpu.cycle = 0;

    let mut tasks = dec_zpx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 5);
  }

  #[test]
  fn should_set_processor_status_of_value_in_memory() {
    const VALUE: Byte = 0x01;
    let mut memory = MemoryMock::new(&[ZERO_PAGE_ADDR, 0xFF, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = 0x02;

    let mut tasks = dec_zpx(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod dec_a {
  use crate::cpu::{
    Byte, CPU, Word,
    instructions::dec_a,
    tests::{MemoryMock, run_tasks},
  };

  const VALUE: Byte = 0x09;
  const ADDR_LO: Byte = 0x04;
  const ADDR_HI: Byte = 0x00;
  const ADDR: Word = 0x0004;

  #[test]
  fn should_decrement_value_stored_in_memory_at_absolute_address() {
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = dec_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[ADDR as Word], 0x08);
  }

  #[test]
  fn should_take_five_cycles() {
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let mut tasks = dec_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 5);
  }

  #[test]
  fn should_set_processor_status_of_value_in_memory() {
    const VALUE: Byte = 0x01;
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let mut tasks = dec_a(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

#[cfg(test)]
mod dec_ax {
  use crate::cpu::{
    Byte, CPU, Word,
    instructions::dec_ax,
    tests::{MemoryMock, run_tasks},
  };

  const VALUE: Byte = 0x09;
  const ADDR_LO: Byte = 0x02;
  const ADDR_HI: Byte = 0x00;
  const OFFSET: Byte = 0x02;
  const ADDR_OFFSET_BY_X: Word = 0x0004;

  #[test]
  fn should_decrement_value_stored_in_memory_at_absolute_address_offset_by_index_register_x() {
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = OFFSET;

    let mut tasks = dec_ax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(memory[ADDR_OFFSET_BY_X], 0x08);
  }

  #[test]
  fn should_take_six_cycles() {
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = OFFSET;
    cpu.cycle = 0;

    let mut tasks = dec_ax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 6);
  }

  #[test]
  fn should_set_processor_status_of_value_in_memory() {
    const VALUE: Byte = 0x01;
    let mut memory = MemoryMock::new(&[ADDR_LO, ADDR_HI, 0x00, 0x00, VALUE]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.index_register_x = OFFSET;

    let mut tasks = dec_ax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b00000010);
  }
}

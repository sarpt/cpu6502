use crate::cpu::{CPU, Registers, Tasks, tasks::transfer_register::TransferRegistersTasks};

pub fn tax(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(TransferRegistersTasks::new(
    Registers::Accumulator,
    Registers::IndexX,
  ))
}

pub fn txa(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(TransferRegistersTasks::new(
    Registers::IndexX,
    Registers::Accumulator,
  ))
}

pub fn tay(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(TransferRegistersTasks::new(
    Registers::Accumulator,
    Registers::IndexY,
  ))
}

pub fn tya(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(TransferRegistersTasks::new(
    Registers::IndexY,
    Registers::Accumulator,
  ))
}

#[cfg(test)]
mod tax {

  use crate::cpu::{
    CPU,
    instructions::tax,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_push_accumulator_into_index_x_register_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.accumulator = 0xDE;

    let mut tasks = tax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.index_register_x, 0xDE);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.accumulator = 0xDE;
    cpu.cycle = 0;

    let mut tasks = tax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_accumulator_based_on_index_x_register_value() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.accumulator = 0xDE;
    cpu.processor_status = 0x00_u8.into();

    let mut tasks = tax(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b10000000);
  }
}

#[cfg(test)]
mod txa {

  use crate::cpu::{
    CPU,
    instructions::txa,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_push_index_x_register_into_stack_pointer_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0xDE;

    let mut tasks = txa(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.accumulator, 0xDE);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0xDE;
    cpu.cycle = 0;

    let mut tasks = txa(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_processor_status_based_on_index_x_register_value() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_x = 0xDE;
    cpu.processor_status = 0x00_u8.into();

    let mut tasks = txa(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b10000000);
  }
}

#[cfg(test)]
mod tay {

  use crate::cpu::{
    CPU,
    instructions::tay,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_push_accumulator_into_index_y_register_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.accumulator = 0xDE;

    let mut tasks = tay(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.index_register_y, 0xDE);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.accumulator = 0xDE;
    cpu.cycle = 0;

    let mut tasks = tay(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_accumulator_based_on_index_y_register_value() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.accumulator = 0xDE;
    cpu.processor_status = 0x00_u8.into();

    let mut tasks = tay(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b10000000);
  }
}

#[cfg(test)]
mod tya {

  use crate::cpu::{
    CPU,
    instructions::tya,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_push_index_y_register_into_stack_pointer_register() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0xDE;

    let mut tasks = tya(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.accumulator, 0xDE);
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0xDE;
    cpu.cycle = 0;

    let mut tasks = tya(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_set_processor_status_based_on_index_y_register_value() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.index_register_y = 0xDE;
    cpu.processor_status = 0x00_u8.into();

    let mut tasks = tya(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b10000000);
  }
}

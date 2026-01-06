use crate::{
  cpu::{processor_status::Flags, Tasks, CPU},
  memory::Memory,
};

struct ChangeStatusFlagTasks {
  flag: Flags,
  value: bool,
  done: bool,
}

impl ChangeStatusFlagTasks {
  fn new(flag: Flags, value: bool) -> Self {
    ChangeStatusFlagTasks {
      flag,
      done: false,
      value,
    }
  }
}

impl Tasks for ChangeStatusFlagTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut CPU, _: &mut dyn Memory) -> bool {
    if self.done() {
      panic!("tick mustn't be called when done")
    }

    cpu.processor_status.change_flag(self.flag, self.value);
    self.done = true;
    self.done
  }
}

fn change_flag_value(_cpu: &mut CPU, flag: Flags, value: bool) -> Box<dyn Tasks> {
  Box::new(ChangeStatusFlagTasks::new(flag, value))
}

pub fn clc(cpu: &mut CPU) -> Box<dyn Tasks> {
  change_flag_value(cpu, Flags::Carry, false)
}

pub fn cld(cpu: &mut CPU) -> Box<dyn Tasks> {
  change_flag_value(cpu, Flags::DecimalMode, false)
}

pub fn cli(cpu: &mut CPU) -> Box<dyn Tasks> {
  change_flag_value(cpu, Flags::InterruptDisable, false)
}

pub fn clv(cpu: &mut CPU) -> Box<dyn Tasks> {
  change_flag_value(cpu, Flags::Overflow, false)
}

pub fn sec(cpu: &mut CPU) -> Box<dyn Tasks> {
  change_flag_value(cpu, Flags::Carry, true)
}

pub fn sed(cpu: &mut CPU) -> Box<dyn Tasks> {
  change_flag_value(cpu, Flags::DecimalMode, true)
}

pub fn sei(cpu: &mut CPU) -> Box<dyn Tasks> {
  change_flag_value(cpu, Flags::InterruptDisable, true)
}

#[cfg(test)]
mod clc {

  use crate::cpu::{
    instructions::clc,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_clear_carry_flag() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_carry_flag(true);

    let mut tasks = clc(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert!(!cpu.processor_status.get_carry_flag());
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_carry_flag(true);
    cpu.cycle = 0;

    let mut tasks = clc(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

#[cfg(test)]
mod cld {

  use crate::cpu::{
    instructions::cld,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_clear_decimal_flag() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_decimal_mode_flag(true);

    let mut tasks = cld(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert!(!cpu.processor_status.get_decimal_mode_flag());
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_decimal_mode_flag(true);
    cpu.cycle = 0;

    let mut tasks = cld(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

#[cfg(test)]
mod cli {

  use crate::cpu::{
    instructions::cli,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_clear_interrupt_disable_flag() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_interrupt_disable_flag(true);

    let mut tasks = cli(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert!(!cpu.processor_status.get_interrupt_disable_flag());
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_interrupt_disable_flag(true);
    cpu.cycle = 0;

    let mut tasks = cli(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

#[cfg(test)]
mod clv {

  use crate::cpu::{
    instructions::clv,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_clear_overflow_flag() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_overflow_flag(true);

    let mut tasks = clv(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert!(!cpu.processor_status.get_overflow_flag());
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_overflow_flag(true);
    cpu.cycle = 0;

    let mut tasks = clv(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

#[cfg(test)]
mod sec {

  use crate::cpu::{
    instructions::sec,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_set_carry_flag() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_carry_flag(false);

    let mut tasks = sec(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert!(cpu.processor_status.get_carry_flag());
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_carry_flag(false);
    cpu.cycle = 0;

    let mut tasks = sec(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

#[cfg(test)]
mod sed {

  use crate::cpu::{
    instructions::sed,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_set_decimal_mode_flag() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_decimal_mode_flag(false);

    let mut tasks = sed(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert!(cpu.processor_status.get_decimal_mode_flag());
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_decimal_mode_flag(false);
    cpu.cycle = 0;

    let mut tasks = sed(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

#[cfg(test)]
mod sei {

  use crate::cpu::{
    instructions::sei,
    tests::{run_tasks, MemoryMock},
    CPU,
  };

  #[test]
  fn should_set_interrupt_disable_flag() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_interrupt_disable_flag(false);

    let mut tasks = sei(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert!(cpu.processor_status.get_interrupt_disable_flag());
  }

  #[test]
  fn should_take_one_cycle() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_interrupt_disable_flag(false);
    cpu.cycle = 0;

    let mut tasks = sei(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

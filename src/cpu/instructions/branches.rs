use crate::{
  consts::{Byte, Word},
  cpu::{Tasks, CPU},
  memory::Memory,
};

#[derive(PartialEq, PartialOrd)]
enum BranchStep {
  ConditionExecution,
  OffsetProgramCounterLo,
  OffsetProgramCounterHi,
  Done,
}

struct BranchTasks {
  condition: fn(&CPU) -> bool,
  step: BranchStep,
  offset: Byte,
}

impl BranchTasks {
  pub fn new(condition: fn(&CPU) -> bool) -> Self {
    BranchTasks {
      condition,
      step: BranchStep::ConditionExecution,
      offset: 0,
    }
  }
}

impl Tasks for BranchTasks {
  fn done(&self) -> bool {
    self.step == BranchStep::Done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      BranchStep::ConditionExecution => {
        self.offset = memory[cpu.program_counter];
        cpu.increment_program_counter();

        if (self.condition)(cpu) {
          self.step = BranchStep::OffsetProgramCounterLo;
          return false;
        }

        self.step = BranchStep::Done;
        true
      }
      BranchStep::OffsetProgramCounterLo => {
        let [program_counter_lo, program_counter_hi] = cpu.program_counter.to_le_bytes();
        let negative_offset_direction = 0b10000000 & self.offset > 0;
        let directionless_offset = if negative_offset_direction {
          (self.offset ^ 0b11111111) + 1
        } else {
          self.offset
        };
        let offset_program_counter_lo: Byte;
        let carry: bool;

        if negative_offset_direction {
          (offset_program_counter_lo, carry) =
            program_counter_lo.overflowing_sub(directionless_offset);
        } else {
          (offset_program_counter_lo, carry) =
            program_counter_lo.overflowing_add(directionless_offset);
        }

        cpu.program_counter = Word::from_le_bytes([offset_program_counter_lo, program_counter_hi]);

        if !carry {
          self.step = BranchStep::Done;
          return true;
        }

        self.step = BranchStep::OffsetProgramCounterHi;
        false
      }
      BranchStep::OffsetProgramCounterHi => {
        let negative_offset_direction = 0b10000000 & self.offset > 0;
        let [program_counter_lo, program_counter_hi] = cpu.program_counter.to_le_bytes();
        let offset_program_counter_hi: Byte = if negative_offset_direction {
          program_counter_hi.wrapping_sub(1)
        } else {
          program_counter_hi.wrapping_add(1)
        };
        cpu.program_counter = Word::from_le_bytes([program_counter_lo, offset_program_counter_hi]);

        self.step = BranchStep::Done;
        true
      }
      BranchStep::Done => true,
    }
  }
}

pub fn bcc(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    !cpu.processor_status.get_carry_flag()
  }))
}

pub fn bcs(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    cpu.processor_status.get_carry_flag()
  }))
}

pub fn beq(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    cpu.processor_status.get_zero_flag()
  }))
}

pub fn bmi(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    cpu.processor_status.get_negative_flag()
  }))
}

pub fn bne(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    !cpu.processor_status.get_zero_flag()
  }))
}

pub fn bpl(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    !cpu.processor_status.get_negative_flag()
  }))
}

pub fn bvs(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    cpu.processor_status.get_overflow_flag()
  }))
}

pub fn bvc(_cpu: &mut CPU) -> Box<dyn Tasks> {
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    !cpu.processor_status.get_overflow_flag()
  }))
}

#[cfg(test)]
mod common_branching_tasks {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::branches::BranchTasks,
      tasks::Tasks,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_and_advance_past_operand_when_condition_is_false() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x0000;

    let condition: fn(&CPU) -> bool = |_| false;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_and_offset_program_counter_by_operand_when_condition_is_true() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let condition: fn(&CPU) -> bool = |_| true;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }

  #[test]
  fn should_take_branch_and_offset_program_counter_backwards_by_negative_operand_in_twos_complement_when_condition_is_true(
  ) {
    const OFFSET: Byte = 0x03;
    const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
    let mut memory = MemoryMock::new(&[0x22, 0x00, NEGATIVE_OFFSET_TWOS_COMPLEMENT, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x02;

    let condition: fn(&CPU) -> bool = |_| true;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x00);
  }

  #[test]
  fn should_take_and_offset_program_counter_over_page_flip_by_operand_when_condition_is_true() {
    const OFFSET: Byte = 0x04;
    let mut payload: [Byte; 512] = [0x00; 512];
    payload[0x00FE] = OFFSET;
    let mut memory = MemoryMock::new(&payload);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00FE;

    let condition: fn(&CPU) -> bool = |_| true;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0103);
  }

  #[test]
  fn should_take_branch_and_offset_program_counter_backwards_over_page_flip_by_negative_operand_in_twos_complement_when_condition_is_true(
  ) {
    const OFFSET: Byte = 0x03;
    const NEGATIVE_OFFSET_TWOS_COMPLEMENT: Byte = (OFFSET ^ 0xFF) + 1;
    let mut memory = MemoryMock::new(&[NEGATIVE_OFFSET_TWOS_COMPLEMENT, 0x00, 0x00, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let condition: fn(&CPU) -> bool = |_| true;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0xFFFE);
  }

  #[test]
  fn should_take_one_cycle_when_not_branching() {
    let mut memory = MemoryMock::new(&[0x02, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let condition: fn(&CPU) -> bool = |_| false;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }

  #[test]
  fn should_take_two_cycles_when_branching_without_crossing_a_page_flip() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let condition: fn(&CPU) -> bool = |_| true;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 2);
  }

  #[test]
  fn should_take_three_cycles_when_branching_with_a_page_flips_crossing() {
    const OFFSET: Byte = 0x04;
    let mut payload: [Byte; 512] = [0x00; 512];
    payload[0x00FE] = OFFSET;
    let mut memory = MemoryMock::new(&payload);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00FE;
    cpu.cycle = 0;

    let condition: fn(&CPU) -> bool = |_| true;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 3);
  }
}

#[cfg(test)]
mod bcc {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::bcc,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_when_carry_flag_is_set() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_carry_flag(true);
    cpu.program_counter = 0x00;

    let mut tasks = bcc(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_when_carry_flag_is_clear() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_carry_flag(false);
    cpu.program_counter = 0x00;

    let mut tasks = bcc(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }
}

#[cfg(test)]
mod bcs {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::bcs,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_when_carry_flag_is_clear() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_carry_flag(false);
    cpu.program_counter = 0x00;

    let mut tasks = bcs(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_when_carry_flag_is_set() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_carry_flag(true);
    cpu.program_counter = 0x00;

    let mut tasks = bcs(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }
}

#[cfg(test)]
mod beq {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::beq,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_when_zero_flag_is_clear() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_zero_flag(false);
    cpu.program_counter = 0x00;

    let mut tasks = beq(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_when_zero_flag_is_set() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_zero_flag(true);
    cpu.program_counter = 0x00;

    let mut tasks = beq(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }
}

#[cfg(test)]
mod bmi {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::bmi,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_when_negative_flag_is_clear() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_negative_flag(false);
    cpu.program_counter = 0x00;

    let mut tasks = bmi(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_when_negative_flag_is_set() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_negative_flag(true);
    cpu.program_counter = 0x00;

    let mut tasks = bmi(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }
}

#[cfg(test)]
mod bne {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::bne,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_when_zero_flag_is_set() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_zero_flag(true);
    cpu.program_counter = 0x00;

    let mut tasks = bne(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_when_zero_flag_is_clear() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_zero_flag(false);
    cpu.program_counter = 0x00;

    let mut tasks = bne(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }
}

#[cfg(test)]
mod bpl {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::bpl,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_when_negative_flag_is_set() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_negative_flag(true);
    cpu.program_counter = 0x00;

    let mut tasks = bpl(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_when_negative_flag_is_clear() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_negative_flag(false);
    cpu.program_counter = 0x00;

    let mut tasks = bpl(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }
}

#[cfg(test)]
mod bvc {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::bvc,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_when_negative_flag_is_set() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_overflow_flag(true);
    cpu.program_counter = 0x00;

    let mut tasks = bvc(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_when_negative_flag_is_clear() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_overflow_flag(false);
    cpu.program_counter = 0x00;

    let mut tasks = bvc(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }
}

#[cfg(test)]
mod bvs {

  use crate::{
    consts::Byte,
    cpu::{
      instructions::bvs,
      tests::{run_tasks, MemoryMock},
      CPU,
    },
  };

  #[test]
  fn should_not_take_branch_when_negative_flag_is_clear() {
    let mut memory = MemoryMock::new(&[0x22, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_overflow_flag(false);
    cpu.program_counter = 0x00;

    let mut tasks = bvs(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0001);
  }

  #[test]
  fn should_take_branch_when_negative_flag_is_set() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.processor_status.change_overflow_flag(true);
    cpu.program_counter = 0x00;

    let mut tasks = bvs(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.program_counter, 0x0004);
  }
}

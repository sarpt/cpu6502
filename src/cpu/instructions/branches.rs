use crate::{
  consts::{Byte, Word},
  cpu::{CPU, Tasks},
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
}

impl BranchTasks {
  pub fn new(condition: fn(&CPU) -> bool) -> Self {
    BranchTasks {
      condition,
      step: BranchStep::ConditionExecution,
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
        let offset = memory[cpu.program_counter];
        cpu.addr.set_indirect_lo(offset);
        cpu.increment_program_counter();

        if (self.condition)(cpu) {
          self.step = BranchStep::OffsetProgramCounterLo;
          return false;
        }

        cpu.addr.done = true;
        self.step = BranchStep::Done;
        true
      }
      BranchStep::OffsetProgramCounterLo => {
        let offset = cpu
          .addr
          .indirect()
          .expect("unexpected lack of indirect address in OffsetProgramCounterLo step")
          .to_le_bytes()[0] as i8;
        let [program_counter_lo, program_counter_hi] = cpu.program_counter.to_le_bytes();
        let offset_program_counter_lo: u8;
        let carry: bool;

        if offset < 0 {
          (offset_program_counter_lo, carry) =
            (program_counter_lo).overflowing_sub(offset.unsigned_abs());
        } else {
          (offset_program_counter_lo, carry) =
            (program_counter_lo).overflowing_add(offset.unsigned_abs());
        }

        cpu.program_counter = Word::from_le_bytes([offset_program_counter_lo, program_counter_hi]);
        if !carry {
          // TODO: Maybe separate this into addressing task that is called without
          // yielding it from this tick immediately? Maybe could take a concrete
          // struct instead of a trait to signal that the relative tasks are
          // treated specially. Sounds like unnecessary part but currently it throws
          // me off that this addressing is set here instead of a specific task.
          // On the other hand, Task ticks are supposed to signify one clock cycle.
          // Additionally, the offset is decided in the previous tick, and calculating
          // target_addr ahead of time is iffy since memory could be changed outside
          // cpu between cycles. Maybe swallow struct ticks over multiple steps?
          cpu.addr.set(cpu.program_counter);
          cpu.addr.done = true;
          self.step = BranchStep::Done;
          return true;
        }

        self.step = BranchStep::OffsetProgramCounterHi;
        false
      }
      BranchStep::OffsetProgramCounterHi => {
        let offset = cpu
          .addr
          .indirect()
          .expect("unexpected lack of indirect address in OffsetProgramCounterLo step")
          .to_le_bytes()[0] as i8;
        let [program_counter_lo, program_counter_hi] = cpu.program_counter.to_le_bytes();
        let offset_program_counter_hi: Byte = if offset < 0 {
          program_counter_hi.wrapping_sub(1)
        } else {
          program_counter_hi.wrapping_add(1)
        };
        cpu.program_counter = Word::from_le_bytes([program_counter_lo, offset_program_counter_hi]);
        cpu.addr.set(cpu.program_counter);
        cpu.addr.done = true;
        self.step = BranchStep::Done;
        true
      }
      BranchStep::Done => true,
    }
  }
}

pub fn bcc(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu
    .addr
    .reset(crate::cpu::addressing::AddressingMode::Relative);
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    !cpu.processor_status.get_carry_flag()
  }))
}

pub fn bcs(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu
    .addr
    .reset(crate::cpu::addressing::AddressingMode::Relative);
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    cpu.processor_status.get_carry_flag()
  }))
}

pub fn beq(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu
    .addr
    .reset(crate::cpu::addressing::AddressingMode::Relative);
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    cpu.processor_status.get_zero_flag()
  }))
}

pub fn bmi(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu
    .addr
    .reset(crate::cpu::addressing::AddressingMode::Relative);
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    cpu.processor_status.get_negative_flag()
  }))
}

pub fn bne(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu
    .addr
    .reset(crate::cpu::addressing::AddressingMode::Relative);
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    !cpu.processor_status.get_zero_flag()
  }))
}

pub fn bpl(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu
    .addr
    .reset(crate::cpu::addressing::AddressingMode::Relative);
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    !cpu.processor_status.get_negative_flag()
  }))
}

pub fn bvs(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu
    .addr
    .reset(crate::cpu::addressing::AddressingMode::Relative);
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    cpu.processor_status.get_overflow_flag()
  }))
}

pub fn bvc(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu
    .addr
    .reset(crate::cpu::addressing::AddressingMode::Relative);
  Box::new(BranchTasks::new(|cpu: &CPU| -> bool {
    !cpu.processor_status.get_overflow_flag()
  }))
}

#[cfg(test)]
mod common_branching_tasks {

  use crate::{
    consts::Byte,
    cpu::{
      CPU,
      instructions::branches::BranchTasks,
      tasks::Tasks,
      tests::{MemoryMock, run_tasks},
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
    assert!(cpu.addr.done);
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
  fn should_take_branch_and_offset_program_counter_backwards_by_negative_operand_in_twos_complement_when_condition_is_true()
   {
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
  fn should_take_branch_and_offset_program_counter_backwards_over_page_flip_by_negative_operand_in_twos_complement_when_condition_is_true()
   {
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

  #[test]
  fn should_fill_indirect_addr_with_offset_and_addr_value_with_branch_target() {
    const OFFSET: Byte = 0x03;
    let mut memory = MemoryMock::new(&[OFFSET, 0x00, 0x01, 0x00]);
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;

    let condition: fn(&CPU) -> bool = |_| true;
    let mut tasks = Box::new(BranchTasks::new(condition)) as Box<dyn Tasks>;
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.addr.indirect(), Some(0x0003));
    assert_eq!(cpu.addr.value(), Some(0x0004));
  }

  #[test]
  fn should_fill_addr_value_with_branch_target_when_a_page_flip_occurs() {
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

    assert_eq!(cpu.addr.indirect(), Some(0x0004));
    assert_eq!(cpu.addr.value(), Some(0x0103));
  }
}

#[cfg(test)]
mod bcc {

  use crate::{
    consts::Byte,
    cpu::{
      CPU,
      instructions::bcc,
      tests::{MemoryMock, run_tasks},
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
      CPU,
      instructions::bcs,
      tests::{MemoryMock, run_tasks},
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
      CPU,
      instructions::beq,
      tests::{MemoryMock, run_tasks},
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
      CPU,
      instructions::bmi,
      tests::{MemoryMock, run_tasks},
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
      CPU,
      instructions::bne,
      tests::{MemoryMock, run_tasks},
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
      CPU,
      instructions::bpl,
      tests::{MemoryMock, run_tasks},
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
      CPU,
      instructions::bvc,
      tests::{MemoryMock, run_tasks},
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
      CPU,
      instructions::bvs,
      tests::{MemoryMock, run_tasks},
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

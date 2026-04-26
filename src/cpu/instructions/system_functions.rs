use crate::{
  consts::{BRK_INTERRUPT_VECTOR, Byte},
  cpu::{CPU, ChipVariant, Tasks},
  memory::Memory,
};

struct NopTasks {
  done: bool,
}

impl NopTasks {
  fn new() -> Self {
    NopTasks { done: false }
  }
}

impl Tasks for NopTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    if self.done() {
      panic!("tick mustn't be called when done")
    }

    cpu.dummy_fetch(memory);
    self.done = true;
    true
  }
}

pub fn nop(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu.addr.reset_implicit();
  Box::new(NopTasks::new())
}

#[derive(PartialEq, PartialOrd)]
enum BrkSteps {
  InitialFetchAndDiscard,
  PushProgramCounterHi,
  PushProgramCounterLo,
  PushProcessorStatus,
  AccessBrkVectorLo,
  AccessBrkVectorHi,
  Done,
}

struct BrkTasks {
  step: BrkSteps,
}

impl BrkTasks {
  fn new() -> Self {
    BrkTasks {
      step: BrkSteps::InitialFetchAndDiscard,
    }
  }
}

impl Tasks for BrkTasks {
  fn done(&self) -> bool {
    self.step == BrkSteps::Done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      BrkSteps::InitialFetchAndDiscard => {
        _ = memory[cpu.program_counter]; // fetch and discard
        cpu.increment_program_counter();
        self.step = BrkSteps::PushProgramCounterHi;
        false
      }
      BrkSteps::PushProgramCounterHi => {
        memory[cpu.get_stack_ptr_address()] = cpu.get_program_counter_hi();
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
        self.step = BrkSteps::PushProgramCounterLo;
        false
      }
      BrkSteps::PushProgramCounterLo => {
        memory[cpu.get_stack_ptr_address()] = cpu.get_program_counter_lo();
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
        self.step = BrkSteps::PushProcessorStatus;
        false
      }
      BrkSteps::PushProcessorStatus => {
        let status: Byte = cpu.processor_status.into();
        memory[cpu.get_stack_ptr_address()] = status | 0x10;
        cpu.stack_pointer = cpu.stack_pointer.wrapping_sub(1);
        self.step = BrkSteps::AccessBrkVectorLo;
        false
      }
      BrkSteps::AccessBrkVectorLo => {
        let lo = memory[BRK_INTERRUPT_VECTOR];
        cpu.set_program_counter_lo(lo);
        self.step = BrkSteps::AccessBrkVectorHi;
        false
      }
      BrkSteps::AccessBrkVectorHi => {
        cpu.processor_status.change_interrupt_disable_flag(true);
        if cpu.chip_variant != ChipVariant::NMOS {
          cpu.processor_status.change_decimal_mode_flag(false);
        }

        let hi = memory[BRK_INTERRUPT_VECTOR + 1];
        cpu.set_program_counter_hi(hi);

        self.step = BrkSteps::Done;
        true
      }
      BrkSteps::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

pub fn brk(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu.addr.reset_implicit();
  Box::new(BrkTasks::new())
}

#[derive(PartialEq, PartialOrd)]
enum RtiSteps {
  DummyFetch,
  StackPointerPreDecrement,
  PopProcessorStatus,
  PopProgramCounterLo,
  PopProgramCounterHi,
  Done,
}

struct RtiTasks {
  step: RtiSteps,
}

impl RtiTasks {
  fn new() -> Self {
    RtiTasks {
      step: RtiSteps::DummyFetch,
    }
  }
}

impl Tasks for RtiTasks {
  fn done(&self) -> bool {
    self.step == RtiSteps::Done
  }

  fn tick(&mut self, cpu: &mut CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      RtiSteps::DummyFetch => {
        cpu.dummy_fetch(memory);

        self.step = RtiSteps::StackPointerPreDecrement;
        false
      }
      RtiSteps::StackPointerPreDecrement => {
        _ = memory[cpu.get_stack_ptr_address()]; // dummy fetch
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
        self.step = RtiSteps::PopProcessorStatus;
        false
      }
      RtiSteps::PopProcessorStatus => {
        let stack_addr = cpu.get_stack_ptr_address();
        // break flag is always ignored when restoring from stack
        cpu.processor_status = (memory[stack_addr] & 0b11101111).into();
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
        self.step = RtiSteps::PopProgramCounterLo;
        false
      }
      RtiSteps::PopProgramCounterLo => {
        let stack_addr = cpu.get_stack_ptr_address();
        let lo = memory[stack_addr];
        cpu.set_program_counter_lo(lo);
        cpu.stack_pointer = cpu.stack_pointer.wrapping_add(1);
        self.step = RtiSteps::PopProgramCounterHi;
        false
      }
      RtiSteps::PopProgramCounterHi => {
        let stack_addr = cpu.get_stack_ptr_address();
        let hi = memory[stack_addr];
        cpu.set_program_counter_hi(hi);
        self.step = RtiSteps::Done;
        true
      }
      RtiSteps::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

pub fn rti(cpu: &mut CPU) -> Box<dyn Tasks> {
  cpu.addr.reset_implicit();
  Box::new(RtiTasks::new())
}

#[cfg(test)]
mod brk {
  #[cfg(test)]
  mod common {
    use crate::{
      consts::Byte,
      cpu::{
        CPU,
        instructions::brk,
        tests::{MemoryMock, run_tasks},
      },
    };

    #[test]
    fn should_put_program_counter_incremented_by_one_and_processor_status_on_stack() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.processor_status.set(0b11101111);
      cpu.stack_pointer = 0xFF;
      cpu.program_counter = 0xABCD;

      let mut tasks = brk(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[0x01FF], 0xAB);
      assert_eq!(memory[0x01FE], 0xCE);
      assert_eq!(memory[0x01FD], 0b11111111);
    }

    #[test]
    fn should_jump_to_address_stored_in_brk_vector() {
      const ADDR_LO: Byte = 0xAD;
      const ADDR_HI: Byte = 0x9B;
      let mut memory = MemoryMock::default();
      memory[0xFFFE] = ADDR_LO;
      memory[0xFFFF] = ADDR_HI;

      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;

      let mut tasks = brk(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.program_counter, 0x9BAD);
    }

    #[test]
    fn should_set_push_processor_status_flag_with_break_status_without_changing_break_itself() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.stack_pointer = 0xFF;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_break_flag(false);

      let mut tasks = brk(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[0x01FD], 0b00110000);
      assert!(!cpu.processor_status.get_break_flag());
    }

    #[test]
    fn should_set_interrupt_disable_flag() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.stack_pointer = 0xFF;
      cpu.program_counter = 0x00;
      cpu.processor_status.change_break_flag(false);

      let mut tasks = brk(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(memory[0x01FD], 0b00110000);
      assert!(cpu.processor_status.get_interrupt_disable_flag());
    }

    #[test]
    fn should_take_six_cycles() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = brk(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 6);
    }
  }

  #[cfg(test)]
  mod cmos {
    use crate::cpu::{CPU, instructions::brk, tests::MemoryMock};

    #[cfg(test)]
    mod rockwell {
      use crate::cpu::tests::run_tasks;

      use super::*;

      #[test]
      fn should_clear_decimal_processor_status_flag() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_rockwell_cmos();
        cpu.program_counter = 0x00;
        cpu.processor_status.change_decimal_mode_flag(true);

        let mut tasks = brk(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_decimal_mode_flag());
      }
    }

    #[cfg(test)]
    mod wdc {
      use crate::cpu::tests::run_tasks;

      use super::*;

      #[test]
      fn should_clear_decimal_processor_status_flag() {
        let mut memory = MemoryMock::default();
        let mut cpu = CPU::new_wdc_cmos();
        cpu.program_counter = 0x00;
        cpu.processor_status.change_decimal_mode_flag(true);

        let mut tasks = brk(&mut cpu);
        run_tasks(&mut cpu, &mut *tasks, &mut memory);

        assert!(!cpu.processor_status.get_decimal_mode_flag());
      }
    }
  }

  #[cfg(test)]
  mod nmos {
    use crate::cpu::{
      CPU,
      instructions::brk,
      tests::{MemoryMock, run_tasks},
    };

    #[test]
    fn should_not_clear_decimal_processor_status_flag() {
      let mut memory = MemoryMock::default();
      let mut cpu = CPU::new_nmos();
      cpu.program_counter = 0x00;
      cpu.processor_status.change_decimal_mode_flag(true);

      let mut tasks = brk(&mut cpu);
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert!(cpu.processor_status.get_decimal_mode_flag());
    }
  }
}

#[cfg(test)]
mod rti {
  use crate::cpu::{
    CPU,
    instructions::rti,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_pop_processor_status_and_program_counter_from_stack_omitting_brk_flag() {
    let mut memory = MemoryMock::default();
    memory[0x01FF] = 0xAB;
    memory[0x01FE] = 0xCD;
    memory[0x01FD] = 0b11111111;

    let mut cpu = CPU::new_nmos();
    cpu.processor_status.set(0b00000000);
    cpu.stack_pointer = 0xFC;
    cpu.program_counter = 0x00;

    let mut tasks = rti(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.processor_status, 0b11101111);
    assert_eq!(cpu.program_counter, 0xABCD);
  }

  #[test]
  fn should_take_five_cycles() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x00;
    cpu.cycle = 0;

    let mut tasks = rti(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 5);
  }
}

#[cfg(test)]
mod nop {
  use crate::cpu::{
    CPU,
    instructions::nop,
    tests::{MemoryMock, run_tasks},
  };

  #[test]
  fn should_take_one_cycles() {
    let mut memory = MemoryMock::default();
    let mut cpu = CPU::new_nmos();
    cpu.program_counter = 0x05;
    cpu.cycle = 0;

    let mut tasks = nop(&mut cpu);
    run_tasks(&mut cpu, &mut *tasks, &mut memory);

    assert_eq!(cpu.cycle, 1);
  }
}

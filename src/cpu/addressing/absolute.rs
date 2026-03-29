use crate::{consts::Word, cpu::tasks::Tasks, memory::Memory};

use super::OffsetVariant;

#[derive(Eq, PartialEq)]
enum AbsoluteOffsetStep {
  MemoryAccessLo,
  MemoryAccessHi,
  OffsetLo,
  OffsetHi,
  Done,
}

pub struct AbsoluteOffsetAddressingTasks {
  step: AbsoluteOffsetStep,
  variant: OffsetVariant,
}

impl AbsoluteOffsetAddressingTasks {
  pub fn new_offset_by_x() -> Self {
    AbsoluteOffsetAddressingTasks {
      step: AbsoluteOffsetStep::MemoryAccessLo,
      variant: OffsetVariant::X,
    }
  }

  pub fn new_offset_by_y() -> Self {
    AbsoluteOffsetAddressingTasks {
      step: AbsoluteOffsetStep::MemoryAccessLo,
      variant: OffsetVariant::Y,
    }
  }
}

impl Tasks for AbsoluteOffsetAddressingTasks {
  fn done(&self) -> bool {
    self.step == AbsoluteOffsetStep::Done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      AbsoluteOffsetStep::MemoryAccessLo => {
        match self.variant {
          OffsetVariant::X => cpu
            .addr
            .reset(crate::cpu::addressing::AddressingMode::AbsoluteX),
          OffsetVariant::Y => cpu
            .addr
            .reset(crate::cpu::addressing::AddressingMode::AbsoluteY),
        }

        let addr_lo = memory[cpu.program_counter];
        cpu.addr.set_indirect_lo(addr_lo);
        cpu.increment_program_counter();
        self.step = AbsoluteOffsetStep::MemoryAccessHi;

        false
      }
      AbsoluteOffsetStep::MemoryAccessHi => {
        let addr_hi = memory[cpu.program_counter];
        cpu.addr.set_indirect_hi(addr_hi);
        cpu.increment_program_counter();
        self.step = AbsoluteOffsetStep::OffsetLo;

        false
      }
      AbsoluteOffsetStep::OffsetLo => {
        let offset = match self.variant {
          OffsetVariant::X => cpu.index_register_x,
          OffsetVariant::Y => cpu.index_register_y,
        };
        let [lo, hi] = cpu
          .addr
          .indirect()
          .expect("unexpected lack of indirect address in OffsetLo step")
          .to_le_bytes();
        let (new_lo, carry) = lo.overflowing_add(offset);
        cpu.addr.set(Word::from_le_bytes([new_lo, hi]));

        if !carry {
          cpu.addr.done = true;
          self.step = AbsoluteOffsetStep::Done;
          true
        } else {
          self.step = AbsoluteOffsetStep::OffsetHi;
          false
        }
      }
      AbsoluteOffsetStep::OffsetHi => {
        let [_, hi] = cpu
          .addr
          .indirect()
          .expect("unexpected lack of indirect address in OffsetHi step")
          .to_le_bytes();
        let new_hi = hi.wrapping_add(1);
        cpu.addr.set_hi(new_hi);

        cpu.addr.done = true;
        self.step = AbsoluteOffsetStep::Done;
        true
      }
      AbsoluteOffsetStep::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

#[derive(Eq, PartialEq)]
enum AbsoluteStep {
  MemoryLo,
  MemoryHi,
  Done,
}

pub struct AbsoluteAddressingTasks {
  step: AbsoluteStep,
}

impl AbsoluteAddressingTasks {
  pub fn new() -> Self {
    AbsoluteAddressingTasks {
      step: AbsoluteStep::MemoryLo,
    }
  }
}

impl Tasks for AbsoluteAddressingTasks {
  fn done(&self) -> bool {
    self.step == AbsoluteStep::Done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      AbsoluteStep::MemoryLo => {
        cpu.addr.reset(super::AddressingMode::Absolute);
        let addr_lo = memory[cpu.program_counter];
        cpu.addr.set_lo(addr_lo);
        cpu.increment_program_counter();
        self.step = AbsoluteStep::MemoryHi;

        false
      }
      AbsoluteStep::MemoryHi => {
        let addr_hi = memory[cpu.program_counter];
        cpu.addr.set_hi(addr_hi);
        cpu.increment_program_counter();

        cpu.addr.done = true;
        self.step = AbsoluteStep::Done;
        true
      }
      AbsoluteStep::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

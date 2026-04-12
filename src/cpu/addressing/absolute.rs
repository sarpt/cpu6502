use crate::{
  consts::Word,
  cpu::{addressing::AddressingTasks, tasks::Tasks},
  memory::Memory,
};

use super::OffsetVariant;

#[derive(Eq, PartialEq)]
enum AbsoluteOffsetStep {
  MemoryAccessLo,
  MemoryAccessHiOffsetLo,
  FixHi,
  Refetch,
  Done,
}

#[derive(Eq, PartialEq)]
pub enum AccessVariant {
  Read,
  Modify,
  Write,
}

pub struct AbsoluteOffsetAddressingTasks {
  step: AbsoluteOffsetStep,
  pub offset_variant: OffsetVariant,
  pub access_variant: AccessVariant,
  carry: bool,
}

impl AbsoluteOffsetAddressingTasks {
  pub fn new(offset_variant: OffsetVariant, access_variant: AccessVariant) -> Self {
    AbsoluteOffsetAddressingTasks {
      step: AbsoluteOffsetStep::MemoryAccessLo,
      offset_variant,
      access_variant,
      carry: false,
    }
  }
}

impl AddressingTasks for AbsoluteOffsetAddressingTasks {
  fn fetch_during_addressing(&self) -> bool {
    true
  }
}

impl Tasks for AbsoluteOffsetAddressingTasks {
  fn done(&self) -> bool {
    self.step == AbsoluteOffsetStep::Done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      AbsoluteOffsetStep::MemoryAccessLo => {
        match self.offset_variant {
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
        self.step = AbsoluteOffsetStep::MemoryAccessHiOffsetLo;

        false
      }
      AbsoluteOffsetStep::MemoryAccessHiOffsetLo => {
        let addr_hi = memory[cpu.program_counter];
        cpu.addr.set_indirect_hi(addr_hi);
        cpu.increment_program_counter();

        let offset = match self.offset_variant {
          OffsetVariant::X => cpu.index_register_x,
          OffsetVariant::Y => cpu.index_register_y,
        };
        let [lo, hi] = cpu
          .addr
          .indirect()
          .expect("unexpected lack of indirect address in MemoryAccessHiOffsetLo step")
          .to_le_bytes();
        let (new_lo, carry) = lo.overflowing_add(offset);
        cpu.addr.set(Word::from_le_bytes([new_lo, hi]));
        self.carry = carry;

        self.step = AbsoluteOffsetStep::FixHi;
        false
      }
      AbsoluteOffsetStep::FixHi => {
        let [lo, hi] = cpu
          .addr
          .value()
          .expect("unexpected lack of indirect address in FixHi step")
          .to_le_bytes();
        let tgt_addr = Word::from_le_bytes([lo, hi]);
        _ = memory[tgt_addr]; // dummy read

        if self.access_variant == AccessVariant::Read && !self.carry {
          cpu.addr.done = true;
          self.step = AbsoluteOffsetStep::Done;
          return true;
        }

        if self.carry {
          cpu.addr.set_hi(hi.wrapping_add(1));
        }

        if self.access_variant == AccessVariant::Write {
          cpu.addr.done = true;
          self.step = AbsoluteOffsetStep::Done;
          return true;
        }

        self.step = AbsoluteOffsetStep::Refetch;
        false
      }
      AbsoluteOffsetStep::Refetch => {
        let tgt_addr = cpu
          .addr
          .value()
          .expect("unexpected lack of indirect address in Refetch step");
        _ = memory[tgt_addr]; // dummy read

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

impl AddressingTasks for AbsoluteAddressingTasks {
  fn fetch_during_addressing(&self) -> bool {
    false
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

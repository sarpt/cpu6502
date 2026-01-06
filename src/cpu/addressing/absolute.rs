use crate::{consts::Word, cpu::tasks::Tasks, memory::Memory};

use super::{address::Address, AddressingTasks, OffsetVariant};

enum AbsoluteOffsetStep {
  MemoryAccessLo,
  MemoryAccessHi,
  OffsetLo,
  OffsetHi,
}

pub struct AbsoluteOffsetAddressingTasks {
  addr: Address,
  done: bool,
  step: AbsoluteOffsetStep,
  variant: OffsetVariant,
}

impl AbsoluteOffsetAddressingTasks {
  pub fn new_offset_by_x() -> Self {
    AbsoluteOffsetAddressingTasks {
      addr: Address::new(),
      done: false,
      step: AbsoluteOffsetStep::MemoryAccessLo,
      variant: OffsetVariant::X,
    }
  }

  pub fn new_offset_by_y() -> Self {
    AbsoluteOffsetAddressingTasks {
      addr: Address::new(),
      done: false,
      step: AbsoluteOffsetStep::MemoryAccessLo,
      variant: OffsetVariant::Y,
    }
  }
}

impl Tasks for AbsoluteOffsetAddressingTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    if self.done {
      return self.done;
    }

    match self.step {
      AbsoluteOffsetStep::MemoryAccessLo => {
        let addr_lo = memory[cpu.program_counter];
        self.addr.set_lo(addr_lo);
        cpu.increment_program_counter();
        self.step = AbsoluteOffsetStep::MemoryAccessHi;

        false
      }
      AbsoluteOffsetStep::MemoryAccessHi => {
        let addr_hi = memory[cpu.program_counter];
        self.addr.set_hi(addr_hi);
        cpu.increment_program_counter();
        self.step = AbsoluteOffsetStep::OffsetLo;

        false
      }
      AbsoluteOffsetStep::OffsetLo => {
        let offset = match self.variant {
          OffsetVariant::X => cpu.index_register_x,
          OffsetVariant::Y => cpu.index_register_y,
        };
        let [lo, hi] = self
          .addr
          .value()
          .expect("unexpected lack of address in OffsetLo step")
          .to_le_bytes();
        let (new_lo, carry) = lo.overflowing_add(offset);
        self.addr.set(Word::from_le_bytes([new_lo, hi]));
        self.step = AbsoluteOffsetStep::OffsetHi;

        if !carry {
          self.done = true;
        }
        self.done
      }
      AbsoluteOffsetStep::OffsetHi => {
        let [lo, hi] = self
          .addr
          .value()
          .expect("unexpected lack of address in OffsetHi step")
          .to_le_bytes();
        let new_hi = hi.wrapping_add(1);
        self.addr.set(Word::from_le_bytes([lo, new_hi]));

        self.done = true;
        self.done
      }
    }
  }
}

impl AddressingTasks for AbsoluteOffsetAddressingTasks {
  fn address(&self) -> Option<Word> {
    self.addr.value()
  }
}

enum AbsoluteStep {
  MemoryLo,
  MemoryHi,
}

pub struct AbsoluteAddressingTasks {
  addr: Address,
  done: bool,
  step: AbsoluteStep,
}

impl AbsoluteAddressingTasks {
  pub fn new() -> Self {
    AbsoluteAddressingTasks {
      addr: Address::new(),
      done: false,
      step: AbsoluteStep::MemoryLo,
    }
  }
}

impl Tasks for AbsoluteAddressingTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    if self.done {
      return self.done;
    }

    match self.step {
      AbsoluteStep::MemoryLo => {
        let addr_lo = memory[cpu.program_counter];
        self.addr.set_lo(addr_lo);
        cpu.increment_program_counter();
        self.step = AbsoluteStep::MemoryHi;

        false
      }
      AbsoluteStep::MemoryHi => {
        let addr_hi = memory[cpu.program_counter];
        self.addr.set_hi(addr_hi);
        cpu.increment_program_counter();

        self.done = true;
        self.done
      }
    }
  }
}

impl AddressingTasks for AbsoluteAddressingTasks {
  fn address(&self) -> Option<Word> {
    self.addr.value()
  }
}

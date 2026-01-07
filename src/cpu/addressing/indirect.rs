use crate::{
  consts::{Byte, Word},
  cpu::tasks::Tasks,
  memory::Memory,
};

enum IndirectIndexYStep {
  MemoryAccess,
  IndirectAccessLo,
  IndirectAccessHi,
  OffsetLo,
  OffsetHi,
}

pub struct IndirectIndexYAddressingTasks {
  done: bool,
  step: IndirectIndexYStep,
  tgt_addr: Word,
}

impl IndirectIndexYAddressingTasks {
  pub fn new() -> Self {
    IndirectIndexYAddressingTasks {
      done: false,
      step: IndirectIndexYStep::MemoryAccess,
      tgt_addr: Word::default(),
    }
  }
}

impl Tasks for IndirectIndexYAddressingTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    if self.done {
      return self.done;
    }

    match self.step {
      IndirectIndexYStep::MemoryAccess => {
        let addr: Byte = memory[cpu.program_counter];
        self.tgt_addr = addr.into();
        cpu.increment_program_counter();
        self.step = IndirectIndexYStep::IndirectAccessLo;

        false
      }
      IndirectIndexYStep::IndirectAccessLo => {
        let addr_lo = memory[self.tgt_addr];
        cpu.addr.set_lo(addr_lo);
        self.step = IndirectIndexYStep::IndirectAccessHi;

        false
      }
      IndirectIndexYStep::IndirectAccessHi => {
        let addr_hi = memory[self.tgt_addr.wrapping_add(1)];
        cpu.addr.set_hi(addr_hi);
        self.step = IndirectIndexYStep::OffsetLo;

        false
      }
      IndirectIndexYStep::OffsetLo => {
        let [lo, hi] = cpu
          .addr
          .value()
          .expect("unexpected lack of address in OffsetLo step")
          .to_le_bytes();
        let (new_lo, carry) = lo.overflowing_add(cpu.index_register_y);
        cpu.addr.set(Word::from_le_bytes([new_lo, hi]));
        self.step = IndirectIndexYStep::OffsetHi;

        if !carry {
          self.done = true;
        }
        self.done
      }
      IndirectIndexYStep::OffsetHi => {
        let [lo, hi] = cpu
          .addr
          .value()
          .expect("unexpected lack of address in OffsetHi step")
          .to_le_bytes();
        let new_hi = hi.wrapping_add(1);
        cpu.addr.set(Word::from_le_bytes([lo, new_hi]));

        self.done = true;
        self.done
      }
    }
  }
}

enum IndexIndirectXStep {
  IndirectAccess,
  SumWithX,
  MemoryAccessLo,
  MemoryAccessHi,
}
pub struct IndexIndirectXAddressingTasks {
  done: bool,
  step: IndexIndirectXStep,
  tgt_addr: Word,
}

impl IndexIndirectXAddressingTasks {
  pub fn new() -> Self {
    IndexIndirectXAddressingTasks {
      done: false,
      step: IndexIndirectXStep::IndirectAccess,
      tgt_addr: Word::default(),
    }
  }
}

impl Tasks for IndexIndirectXAddressingTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    if self.done {
      return self.done;
    }

    match self.step {
      IndexIndirectXStep::IndirectAccess => {
        let addr: Byte = memory[cpu.program_counter];
        cpu.addr.set(addr);
        cpu.increment_program_counter();
        self.step = IndexIndirectXStep::SumWithX;

        false
      }
      IndexIndirectXStep::SumWithX => {
        let addr_output = cpu
          .addr
          .value()
          .expect("unexpected lack of address in SumWithX step");
        self.tgt_addr = addr_output.wrapping_add(cpu.index_register_x.into());
        self.step = IndexIndirectXStep::MemoryAccessLo;

        false
      }
      IndexIndirectXStep::MemoryAccessLo => {
        let addr_lo = memory[self.tgt_addr];
        cpu.addr.set_lo(addr_lo);
        self.step = IndexIndirectXStep::MemoryAccessHi;

        false
      }
      IndexIndirectXStep::MemoryAccessHi => {
        let addr_hi = memory[self.tgt_addr.wrapping_add(1)];
        cpu.addr.set_hi(addr_hi);

        self.done = true;
        self.done
      }
    }
  }
}

enum IndirectStep {
  IndirectFetchLo,
  IndirectFetchHi,
  AddrFixing,
  MemoryAccessLo,
  FixedMemoryAccessHi,
  IncorrectMemoryAccessHi,
}

pub struct IndirectAddressingTasks {
  fixed_addressing: bool,
  done: bool,
  step: IndirectStep,
  tgt_addr_lo: Byte,
  tgt_addr_hi: Byte,
}

impl IndirectAddressingTasks {
  pub fn new_fixed_addressing() -> Self {
    IndirectAddressingTasks {
      fixed_addressing: true,
      done: false,
      step: IndirectStep::IndirectFetchLo,
      tgt_addr_lo: Byte::default(),
      tgt_addr_hi: Byte::default(),
    }
  }

  pub fn new_incorrect_addressing() -> Self {
    IndirectAddressingTasks {
      fixed_addressing: false,
      done: false,
      step: IndirectStep::IndirectFetchLo,
      tgt_addr_lo: Byte::default(),
      tgt_addr_hi: Byte::default(),
    }
  }
}

impl Tasks for IndirectAddressingTasks {
  fn done(&self) -> bool {
    self.done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    if self.done {
      return self.done;
    }

    match self.step {
      IndirectStep::IndirectFetchLo => {
        self.tgt_addr_lo = memory[cpu.program_counter];
        cpu.increment_program_counter();
        self.step = IndirectStep::IndirectFetchHi;

        false
      }
      IndirectStep::IndirectFetchHi => {
        self.tgt_addr_hi = memory[cpu.program_counter];
        cpu.increment_program_counter();
        if self.fixed_addressing {
          self.step = IndirectStep::AddrFixing;
        } else {
          self.step = IndirectStep::MemoryAccessLo;
        }

        false
      }
      IndirectStep::AddrFixing => {
        self.step = IndirectStep::MemoryAccessLo;

        false
      }
      IndirectStep::MemoryAccessLo => {
        let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
        let addr_lo = memory[addr];
        cpu.addr.set_lo(addr_lo);

        if self.fixed_addressing {
          self.step = IndirectStep::FixedMemoryAccessHi;
        } else {
          self.step = IndirectStep::IncorrectMemoryAccessHi;
        }

        false
      }
      IndirectStep::FixedMemoryAccessHi => {
        let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
        let addr_hi = memory[addr + 1];
        cpu.addr.set_hi(addr_hi);

        self.done = true;
        self.done
      }
      IndirectStep::IncorrectMemoryAccessHi => {
        let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
        let should_incorrectly_jump = self.tgt_addr_lo == 0xFF;
        let mut target_addr = addr + 1;
        if should_incorrectly_jump {
          target_addr = addr & 0xFF00;
        };
        let addr_hi = memory[target_addr];
        cpu.addr.set_hi(addr_hi);

        self.done = true;
        self.done
      }
    }
  }
}

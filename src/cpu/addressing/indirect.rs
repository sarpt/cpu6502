use crate::{
  consts::{Byte, Word},
  cpu::{addressing::AddressingMode, tasks::Tasks},
  memory::Memory,
};

#[derive(Eq, PartialEq)]
enum IndirectIndexYStep {
  MemoryAccess,
  IndirectAccessLo,
  IndirectAccessHi,
  OffsetLo,
  OffsetHi,
  Done,
}

pub struct IndirectIndexYAddressingTasks {
  step: IndirectIndexYStep,
}

impl IndirectIndexYAddressingTasks {
  pub fn new() -> Self {
    IndirectIndexYAddressingTasks {
      step: IndirectIndexYStep::MemoryAccess,
    }
  }
}

impl Tasks for IndirectIndexYAddressingTasks {
  fn done(&self) -> bool {
    self.step == IndirectIndexYStep::Done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      IndirectIndexYStep::MemoryAccess => {
        cpu.addr.reset(AddressingMode::IndirectIndexY);
        let addr: Byte = memory[cpu.program_counter];
        cpu.addr.set_indirect_lo(addr);
        cpu.increment_program_counter();
        self.step = IndirectIndexYStep::IndirectAccessLo;

        false
      }
      IndirectIndexYStep::IndirectAccessLo => {
        let indirect_addr = cpu
          .addr
          .indirect()
          .expect("indirect address is unexpectedly empty in IndirectAccessLo step");
        let addr_lo = memory[indirect_addr];
        cpu.addr.set_lo(addr_lo);
        self.step = IndirectIndexYStep::IndirectAccessHi;

        false
      }
      IndirectIndexYStep::IndirectAccessHi => {
        let indirect_addr = cpu
          .addr
          .indirect()
          .expect("indirect address is unexpectedly empty in IndirectAccessHi step");
        let addr_hi = memory[indirect_addr.wrapping_add(1)];
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

        if !carry {
          cpu.addr.done = true;
          self.step = IndirectIndexYStep::Done;
          true
        } else {
          self.step = IndirectIndexYStep::OffsetHi;
          false
        }
      }
      IndirectIndexYStep::OffsetHi => {
        let [lo, hi] = cpu
          .addr
          .value()
          .expect("unexpected lack of address in OffsetHi step")
          .to_le_bytes();
        let new_hi = hi.wrapping_add(1);
        cpu.addr.set(Word::from_le_bytes([lo, new_hi]));

        cpu.addr.done = true;
        self.step = IndirectIndexYStep::Done;
        true
      }
      IndirectIndexYStep::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

#[derive(Eq, PartialEq)]
enum IndexIndirectXStep {
  IndirectAccess,
  SumWithX,
  MemoryAccessLo,
  MemoryAccessHi,
  Done,
}

pub struct IndexIndirectXAddressingTasks {
  step: IndexIndirectXStep,
  tgt_addr_lo: u8,
}

impl IndexIndirectXAddressingTasks {
  pub fn new() -> Self {
    IndexIndirectXAddressingTasks {
      step: IndexIndirectXStep::IndirectAccess,
      tgt_addr_lo: 0,
    }
  }
}

impl Tasks for IndexIndirectXAddressingTasks {
  fn done(&self) -> bool {
    self.step == IndexIndirectXStep::Done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      IndexIndirectXStep::IndirectAccess => {
        cpu.addr.reset(AddressingMode::IndexIndirectX);
        let addr: Byte = memory[cpu.program_counter];
        cpu.addr.set_indirect_lo(addr);
        cpu.increment_program_counter();
        self.step = IndexIndirectXStep::SumWithX;

        false
      }
      IndexIndirectXStep::SumWithX => {
        cpu.dummy_fetch(memory);
        let addr_output = cpu
          .addr
          .indirect()
          .expect("unexpected lack of indirect address in SumWithX step");
        self.tgt_addr_lo = addr_output.to_le_bytes()[0].wrapping_add(cpu.index_register_x);
        self.step = IndexIndirectXStep::MemoryAccessLo;

        false
      }
      IndexIndirectXStep::MemoryAccessLo => {
        let tgt_addr = [self.tgt_addr_lo, 0x0];
        let addr_lo = memory[Word::from_le_bytes(tgt_addr)];
        cpu.addr.set_lo(addr_lo);
        self.step = IndexIndirectXStep::MemoryAccessHi;

        false
      }
      IndexIndirectXStep::MemoryAccessHi => {
        let tgt_addr = Word::from_le_bytes([self.tgt_addr_lo.wrapping_add(1), 0x0]);
        let addr_hi = memory[tgt_addr];
        cpu.addr.set_hi(addr_hi);
        cpu.addr.done = true;
        self.step = IndexIndirectXStep::Done;

        true
      }
      IndexIndirectXStep::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

#[derive(Eq, PartialEq)]
enum IndirectStep {
  IndirectFetchLo,
  IndirectFetchHi,
  AddrFixing,
  MemoryAccessLo,
  FixedMemoryAccessHi,
  IncorrectMemoryAccessHi,
  Done,
}

pub struct IndirectAddressingTasks {
  fixed_addressing: bool,
  step: IndirectStep,
}

impl IndirectAddressingTasks {
  pub fn new_fixed_addressing() -> Self {
    IndirectAddressingTasks {
      fixed_addressing: true,
      step: IndirectStep::IndirectFetchLo,
    }
  }

  pub fn new_incorrect_addressing() -> Self {
    IndirectAddressingTasks {
      fixed_addressing: false,
      step: IndirectStep::IndirectFetchLo,
    }
  }
}

impl Tasks for IndirectAddressingTasks {
  fn done(&self) -> bool {
    self.step == IndirectStep::Done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      IndirectStep::IndirectFetchLo => {
        cpu.addr.reset(AddressingMode::Indirect);
        cpu.addr.set_indirect_lo(memory[cpu.program_counter]);
        cpu.increment_program_counter();
        self.step = IndirectStep::IndirectFetchHi;

        false
      }
      IndirectStep::IndirectFetchHi => {
        cpu.addr.set_indirect_hi(memory[cpu.program_counter]);
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
        let addr = cpu
          .addr
          .indirect()
          .expect("indirect address is unexpectedly empty");
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
        let addr = cpu
          .addr
          .indirect()
          .expect("indirect address is unexpectedly empty");
        let addr_hi = memory[addr + 1];
        cpu.addr.set_hi(addr_hi);
        cpu.addr.done = true;
        self.step = IndirectStep::Done;

        true
      }
      IndirectStep::IncorrectMemoryAccessHi => {
        let addr = cpu
          .addr
          .indirect()
          .expect("indirect address is unexpectedly empty");
        let should_incorrectly_jump = addr.to_le_bytes()[0] == 0xFF;
        let mut target_addr = addr + 1;
        if should_incorrectly_jump {
          target_addr = addr & 0xFF00;
        };
        let addr_hi = memory[target_addr];
        cpu.addr.set_hi(addr_hi);
        cpu.addr.done = true;
        self.step = IndirectStep::Done;

        true
      }
      IndirectStep::Done => {
        panic!("tick mustn't be called when done")
      }
    }
  }
}

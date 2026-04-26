use crate::{
  consts::{Byte, Word},
  cpu::{
    addressing::{AddressingMode, AddressingTasks, absolute::AccessVariant},
    tasks::Tasks,
  },
  memory::Memory,
};

#[derive(Eq, PartialEq)]
enum IndirectIndexYStep {
  PointerAddrFetch,
  IndirectAccessLo,
  IndirectAccessHi,
  MemAccessAndFixHi,
  Refetch,
  Done,
}

pub struct IndirectIndexYAddressingTasks {
  step: IndirectIndexYStep,
  carry: bool,
  access_variant: AccessVariant,
}

impl IndirectIndexYAddressingTasks {
  pub fn new(access_variant: AccessVariant) -> Self {
    IndirectIndexYAddressingTasks {
      step: IndirectIndexYStep::PointerAddrFetch,
      carry: false,
      access_variant,
    }
  }
}

impl AddressingTasks for IndirectIndexYAddressingTasks {
  fn fetch_during_addressing(&self) -> bool {
    true
  }
}

impl Tasks for IndirectIndexYAddressingTasks {
  fn done(&self) -> bool {
    self.step == IndirectIndexYStep::Done
  }

  fn tick(&mut self, cpu: &mut super::CPU, memory: &mut dyn Memory) -> bool {
    match self.step {
      IndirectIndexYStep::PointerAddrFetch => {
        cpu.addr.reset(AddressingMode::IndirectIndexY);
        let addr: Byte = memory[cpu.program_counter];
        cpu.addr.set_indirect_lo(addr);
        cpu.increment_program_counter();
        self.step = IndirectIndexYStep::IndirectAccessLo;

        false
      }
      IndirectIndexYStep::IndirectAccessLo => {
        let ptr_addr = cpu
          .addr
          .indirect()
          .expect("indirect address is unexpectedly empty in IndirectAccessLo step");
        let addr_lo = memory[ptr_addr];
        cpu.addr.set_lo(addr_lo);
        self.step = IndirectIndexYStep::IndirectAccessHi;

        false
      }
      IndirectIndexYStep::IndirectAccessHi => {
        let [indirect_lo, indirect_hi] = cpu
          .addr
          .indirect()
          .expect("indirect address is unexpectedly empty in IndirectAccessHi step")
          .to_le_bytes();
        let ptr_addr = Word::from_le_bytes([indirect_lo.wrapping_add(1), indirect_hi]);
        let addr_hi = memory[ptr_addr];
        cpu.addr.set_hi(addr_hi);
        let [lo, _] = cpu
          .addr
          .value()
          .expect("unexpected lack of address in OffsetLo step")
          .to_le_bytes();
        let (new_lo, carry) = lo.overflowing_add(cpu.index_register_y);
        cpu.addr.set_lo(new_lo);
        self.carry = carry;
        self.step = IndirectIndexYStep::MemAccessAndFixHi;

        false
      }
      IndirectIndexYStep::MemAccessAndFixHi => {
        let tgt_addr = cpu
          .addr
          .value()
          .expect("unexpected lack of address in OffsetLo step");
        _ = memory[tgt_addr]; // dummy fetch;

        if !self.carry {
          cpu.addr.done = true;
          self.step = IndirectIndexYStep::Done;
          return true;
        }

        let [_, hi] = tgt_addr.to_le_bytes();
        let new_hi = hi.wrapping_add(1);
        cpu.addr.set_hi(new_hi);

        match self.access_variant {
          AccessVariant::Read => {
            self.step = IndirectIndexYStep::Refetch;
            false
          }
          AccessVariant::Modify | AccessVariant::Write => {
            self.step = IndirectIndexYStep::Done;
            true
          }
        }
      }
      IndirectIndexYStep::Refetch => {
        let tgt_addr = cpu
          .addr
          .value()
          .expect("unexpected lack of address in OffsetHi step");
        _ = memory[tgt_addr]; // dummy refetch;

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

impl AddressingTasks for IndexIndirectXAddressingTasks {
  fn fetch_during_addressing(&self) -> bool {
    false
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
        let addr_output = cpu
          .addr
          .indirect()
          .expect("unexpected lack of indirect address in SumWithX step");
        _ = memory[addr_output]; // dummy read
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

impl AddressingTasks for IndirectAddressingTasks {
  fn fetch_during_addressing(&self) -> bool {
    false
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
        let (mut target_addr, _) = addr.overflowing_add(1);
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

#[cfg(test)]
mod tests {
  #[cfg(test)]
  mod indirect_index_y_addressing {
    use crate::cpu::{
      CPU,
      addressing::{absolute::AccessVariant, indirect::IndirectIndexYAddressingTasks},
      tests::{MemoryMock, run_tasks},
    };

    #[test]
    fn should_return_address_offset_by_index_register_y_which_is_stored_at_zero_page_address() {
      let mut memory = MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = Box::new(IndirectIndexYAddressingTasks::new(AccessVariant::Read));
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.addr.value(), Some(0xDD05));
    }

    #[test]
    fn should_advance_program_counter_once() {
      let mut memory = MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;

      let mut tasks = Box::new(IndirectIndexYAddressingTasks::new(AccessVariant::Read));
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.program_counter, 0x01);
    }

    #[test]
    fn should_take_four_cycles_when_not_crossing_page_boundary_during_offset_addition_for_a_read_operation_address()
     {
      let mut memory = MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0x02;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = Box::new(IndirectIndexYAddressingTasks::new(AccessVariant::Read));
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }

    #[test]
    fn should_take_five_cycles_when_crossing_page_boundary_during_offset_addition_for_a_read_operation_address()
     {
      let mut memory = MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0xFF;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = Box::new(IndirectIndexYAddressingTasks::new(AccessVariant::Read));
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 5);
    }

    #[test]
    fn should_take_four_cycles_when_crossing_page_boundary_during_offset_addition_for_a_write_operation_address()
     {
      let mut memory = MemoryMock::new(&[0x02, 0xFF, 0x03, 0xDD, 0x25]);
      let mut cpu = CPU::new_nmos();
      cpu.index_register_y = 0xFF;
      cpu.program_counter = 0x00;
      cpu.cycle = 0;

      let mut tasks = Box::new(IndirectIndexYAddressingTasks::new(AccessVariant::Write));
      run_tasks(&mut cpu, &mut *tasks, &mut memory);

      assert_eq!(cpu.cycle, 4);
    }
  }
}

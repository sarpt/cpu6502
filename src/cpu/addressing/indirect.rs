use crate::{
    consts::{Byte, Word},
    cpu::tasks::Tasks,
};

use super::{address::Address, AddressingTasks};

enum IndirectIndexYStep {
    MemoryAccess,
    IndirectAccessLo,
    IndirectAccessHi,
    OffsetLo,
    OffsetHi,
}

pub struct IndirectIndexYAddressingTasks {
    addr: Address,
    done: bool,
    step: IndirectIndexYStep,
    tgt_addr: Word,
}

impl IndirectIndexYAddressingTasks {
    pub fn new() -> Self {
        return IndirectIndexYAddressingTasks {
            addr: Address::new(),
            done: false,
            step: IndirectIndexYStep::MemoryAccess,
            tgt_addr: Word::default(),
        };
    }
}

impl Tasks for IndirectIndexYAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> bool {
        if self.done {
            return self.done;
        }

        match self.step {
            IndirectIndexYStep::MemoryAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                self.tgt_addr = addr.into();
                cpu.increment_program_counter();
                self.step = IndirectIndexYStep::IndirectAccessLo;

                return false;
            }
            IndirectIndexYStep::IndirectAccessLo => {
                let addr_lo = cpu.access_memory(self.tgt_addr);
                self.addr.set_lo(addr_lo);
                self.step = IndirectIndexYStep::IndirectAccessHi;

                return false;
            }
            IndirectIndexYStep::IndirectAccessHi => {
                let addr_hi = cpu.access_memory(self.tgt_addr.wrapping_add(1));
                self.addr.set_hi(addr_hi);
                self.step = IndirectIndexYStep::OffsetLo;

                return false;
            }
            IndirectIndexYStep::OffsetLo => {
                let [lo, hi] = self
                    .addr
                    .value()
                    .expect("unexpected lack of address in OffsetLo step")
                    .to_le_bytes();
                let (new_lo, carry) = lo.overflowing_add(cpu.index_register_y);
                self.addr.set(Word::from_le_bytes([new_lo, hi]));
                self.step = IndirectIndexYStep::OffsetHi;

                if !carry {
                    self.done = true;
                }
                return self.done;
            }
            IndirectIndexYStep::OffsetHi => {
                let [lo, hi] = self
                    .addr
                    .value()
                    .expect("unexpected lack of address in OffsetHi step")
                    .to_le_bytes();
                let new_hi = hi.wrapping_add(1);
                self.addr.set(Word::from_le_bytes([lo, new_hi]));

                self.done = true;
                return self.done;
            }
        }
    }
}

impl AddressingTasks for IndirectIndexYAddressingTasks {
    fn address(&self) -> Option<Word> {
        self.addr.value()
    }
}

enum IndexIndirectXStep {
    IndirectAccess,
    SumWithX,
    MemoryAccessLo,
    MemoryAccessHi,
}
pub struct IndexIndirectXAddressingTasks {
    addr: Address,
    done: bool,
    step: IndexIndirectXStep,
    tgt_addr: Word,
}

impl IndexIndirectXAddressingTasks {
    pub fn new() -> Self {
        return IndexIndirectXAddressingTasks {
            addr: Address::new(),
            done: false,
            step: IndexIndirectXStep::IndirectAccess,
            tgt_addr: Word::default(),
        };
    }
}

impl Tasks for IndexIndirectXAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> bool {
        if self.done {
            return self.done;
        }

        match self.step {
            IndexIndirectXStep::IndirectAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                self.addr.set(addr);
                cpu.increment_program_counter();
                self.step = IndexIndirectXStep::SumWithX;

                return false;
            }
            IndexIndirectXStep::SumWithX => {
                let addr_output = self
                    .addr
                    .value()
                    .expect("unexpected lack of address in SumWithX step");
                self.tgt_addr = addr_output.wrapping_add(cpu.index_register_x.into());
                self.step = IndexIndirectXStep::MemoryAccessLo;

                return false;
            }
            IndexIndirectXStep::MemoryAccessLo => {
                let addr_lo = cpu.access_memory(self.tgt_addr);
                self.addr.set_lo(addr_lo);
                self.step = IndexIndirectXStep::MemoryAccessHi;

                return false;
            }
            IndexIndirectXStep::MemoryAccessHi => {
                let addr_hi = cpu.access_memory(self.tgt_addr.wrapping_add(1));
                self.addr.set_hi(addr_hi);

                self.done = true;
                return self.done;
            }
        }
    }
}

impl AddressingTasks for IndexIndirectXAddressingTasks {
    fn address(&self) -> Option<Word> {
        self.addr.value()
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
    addr: Address,
    fixed_addressing: bool,
    done: bool,
    step: IndirectStep,
    tgt_addr_lo: Byte,
    tgt_addr_hi: Byte,
}

impl IndirectAddressingTasks {
    pub fn new_fixed_addressing() -> Self {
        return IndirectAddressingTasks {
            addr: Address::new(),
            fixed_addressing: true,
            done: false,
            step: IndirectStep::IndirectFetchLo,
            tgt_addr_lo: Byte::default(),
            tgt_addr_hi: Byte::default(),
        };
    }

    pub fn new_incorrect_addressing() -> Self {
        return IndirectAddressingTasks {
            addr: Address::new(),
            fixed_addressing: false,
            done: false,
            step: IndirectStep::IndirectFetchLo,
            tgt_addr_lo: Byte::default(),
            tgt_addr_hi: Byte::default(),
        };
    }
}

impl Tasks for IndirectAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> bool {
        if self.done {
            return self.done;
        }

        match self.step {
            IndirectStep::IndirectFetchLo => {
                self.tgt_addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.increment_program_counter();
                self.step = IndirectStep::IndirectFetchHi;

                return false;
            }
            IndirectStep::IndirectFetchHi => {
                self.tgt_addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.increment_program_counter();
                if self.fixed_addressing {
                    self.step = IndirectStep::AddrFixing;
                } else {
                    self.step = IndirectStep::MemoryAccessLo;
                }

                return false;
            }
            IndirectStep::AddrFixing => {
                self.step = IndirectStep::MemoryAccessLo;

                return false;
            }
            IndirectStep::MemoryAccessLo => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let addr_lo = cpu.access_memory(addr);
                self.addr.set_lo(addr_lo);

                if self.fixed_addressing {
                    self.step = IndirectStep::FixedMemoryAccessHi;
                } else {
                    self.step = IndirectStep::IncorrectMemoryAccessHi;
                }

                return false;
            }
            IndirectStep::FixedMemoryAccessHi => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let addr_hi = cpu.access_memory(addr + 1);
                self.addr.set_hi(addr_hi);

                self.done = true;
                return self.done;
            }
            IndirectStep::IncorrectMemoryAccessHi => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let should_incorrectly_jump = self.tgt_addr_lo == 0xFF;
                let mut target_addr = addr + 1;
                if should_incorrectly_jump {
                    target_addr = addr & 0xFF00;
                };
                let addr_hi = cpu.access_memory(target_addr);
                self.addr.set_hi(addr_hi);

                self.done = true;
                return self.done;
            }
        }
    }
}

impl AddressingTasks for IndirectAddressingTasks {
    fn address(&self) -> Option<Word> {
        self.addr.value()
    }
}

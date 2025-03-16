use crate::consts::{Byte, Word};

use super::tasks::Tasks;

pub struct ZeroPageAddressingTasks {
    done: bool,
}

impl ZeroPageAddressingTasks {
    pub fn new() -> Self {
        return ZeroPageAddressingTasks { done: false };
    }
}

impl Tasks for ZeroPageAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        let addr: Byte = cpu.access_memory(cpu.program_counter);
        cpu.set_address_output(addr);
        cpu.increment_program_counter();
        self.done = true;

        return (true, self.done);
    }
}

pub struct ImmediateAddressingTasks {
    done: bool,
}

impl ImmediateAddressingTasks {
    pub fn new() -> Self {
        return ImmediateAddressingTasks { done: false };
    }
}

impl Tasks for ImmediateAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        let addr = cpu.program_counter;
        cpu.set_address_output(addr);
        cpu.increment_program_counter();
        self.done = true;

        return (false, self.done);
    }
}

enum OffsetVariant {
    X,
    Y,
}

enum AbsoluteOffsetStep {
    MemoryAccessLo,
    MemoryAccessHi,
    OffsetLo,
    OffsetHi,
}

pub struct AbsoluteOffsetAddressingTasks {
    done: bool,
    step: AbsoluteOffsetStep,
    variant: OffsetVariant,
}

impl AbsoluteOffsetAddressingTasks {
    pub fn new_offset_by_x() -> Self {
        return AbsoluteOffsetAddressingTasks {
            done: false,
            step: AbsoluteOffsetStep::MemoryAccessLo,
            variant: OffsetVariant::X,
        };
    }

    pub fn new_offset_by_y() -> Self {
        return AbsoluteOffsetAddressingTasks {
            done: false,
            step: AbsoluteOffsetStep::MemoryAccessLo,
            variant: OffsetVariant::Y,
        };
    }
}

impl Tasks for AbsoluteOffsetAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            AbsoluteOffsetStep::MemoryAccessLo => {
                let addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_lo(addr_lo);
                cpu.increment_program_counter();
                self.step = AbsoluteOffsetStep::MemoryAccessHi;

                return (true, false);
            }
            AbsoluteOffsetStep::MemoryAccessHi => {
                let addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_hi(addr_hi);
                cpu.increment_program_counter();
                self.step = AbsoluteOffsetStep::OffsetLo;

                return (true, false);
            }
            AbsoluteOffsetStep::OffsetLo => {
                let offset = match self.variant {
                    OffsetVariant::X => cpu.index_register_x,
                    OffsetVariant::Y => cpu.index_register_y,
                };
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let (new_lo, carry) = lo.overflowing_add(offset);
                cpu.address_output = Word::from_le_bytes([new_lo, hi]);
                self.step = AbsoluteOffsetStep::OffsetHi;

                if !carry {
                    self.done = true;
                }
                return (true, self.done);
            }
            AbsoluteOffsetStep::OffsetHi => {
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let new_hi = hi.wrapping_add(1);
                cpu.address_output = Word::from_le_bytes([lo, new_hi]);

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

enum ZeroPageOffsetStep {
    ZeroPageAccess,
    Offset,
}

pub struct ZeroPageOffsetAddressingTasks {
    done: bool,
    step: ZeroPageOffsetStep,
    variant: OffsetVariant,
}

impl ZeroPageOffsetAddressingTasks {
    pub fn new_offset_by_x() -> Self {
        return ZeroPageOffsetAddressingTasks {
            done: false,
            step: ZeroPageOffsetStep::ZeroPageAccess,
            variant: OffsetVariant::X,
        };
    }

    pub fn new_offset_by_y() -> Self {
        return ZeroPageOffsetAddressingTasks {
            done: false,
            step: ZeroPageOffsetStep::ZeroPageAccess,
            variant: OffsetVariant::Y,
        };
    }
}

impl Tasks for ZeroPageOffsetAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            ZeroPageOffsetStep::ZeroPageAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output(addr);
                cpu.increment_program_counter();
                self.step = ZeroPageOffsetStep::Offset;

                return (true, false);
            }
            ZeroPageOffsetStep::Offset => {
                let offset: Byte = match self.variant {
                    OffsetVariant::X => cpu.index_register_x.into(),
                    OffsetVariant::Y => cpu.index_register_y.into(),
                };
                let addr_output = cpu.address_output as Byte;
                let final_address = addr_output.wrapping_add(offset);
                cpu.set_address_output(final_address);

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

enum AbsoluteStep {
    MemoryLo,
    MemoryHi,
}

pub struct AbsoluteAddressingTasks {
    done: bool,
    step: AbsoluteStep,
}

impl AbsoluteAddressingTasks {
    pub fn new() -> Self {
        return AbsoluteAddressingTasks {
            done: false,
            step: AbsoluteStep::MemoryLo,
        };
    }
}

impl Tasks for AbsoluteAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            AbsoluteStep::MemoryLo => {
                let addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_lo(addr_lo);
                cpu.increment_program_counter();
                self.step = AbsoluteStep::MemoryHi;

                return (true, false);
            }
            AbsoluteStep::MemoryHi => {
                let addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_hi(addr_hi);
                cpu.increment_program_counter();

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

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
        return IndirectIndexYAddressingTasks {
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

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            IndirectIndexYStep::MemoryAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                self.tgt_addr = addr.into();
                cpu.increment_program_counter();
                self.step = IndirectIndexYStep::IndirectAccessLo;

                return (true, false);
            }
            IndirectIndexYStep::IndirectAccessLo => {
                let addr_lo = cpu.access_memory(self.tgt_addr);
                cpu.set_address_output_lo(addr_lo);
                self.step = IndirectIndexYStep::IndirectAccessHi;

                return (true, false);
            }
            IndirectIndexYStep::IndirectAccessHi => {
                let addr_hi = cpu.access_memory(self.tgt_addr.wrapping_add(1));
                cpu.set_address_output_hi(addr_hi);
                self.step = IndirectIndexYStep::OffsetLo;

                return (true, false);
            }
            IndirectIndexYStep::OffsetLo => {
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let (new_lo, carry) = lo.overflowing_add(cpu.index_register_y);
                cpu.address_output = Word::from_le_bytes([new_lo, hi]);
                self.step = IndirectIndexYStep::OffsetHi;

                if !carry {
                    self.done = true;
                }
                return (true, self.done);
            }
            IndirectIndexYStep::OffsetHi => {
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let new_hi = hi.wrapping_add(1);
                cpu.address_output = Word::from_le_bytes([lo, new_hi]);

                self.done = true;
                return (true, self.done);
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
        return IndexIndirectXAddressingTasks {
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

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            IndexIndirectXStep::IndirectAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output(addr);
                cpu.increment_program_counter();
                self.step = IndexIndirectXStep::SumWithX;

                return (true, false);
            }
            IndexIndirectXStep::SumWithX => {
                let addr_output = cpu.address_output;
                self.tgt_addr = addr_output.wrapping_add(cpu.index_register_x.into());
                self.step = IndexIndirectXStep::MemoryAccessLo;

                return (true, false);
            }
            IndexIndirectXStep::MemoryAccessLo => {
                let addr_lo = cpu.access_memory(self.tgt_addr);
                cpu.set_address_output_lo(addr_lo);
                self.step = IndexIndirectXStep::MemoryAccessHi;

                return (true, false);
            }
            IndexIndirectXStep::MemoryAccessHi => {
                let addr_hi = cpu.access_memory(self.tgt_addr.wrapping_add(1));
                cpu.set_address_output_hi(addr_hi);

                self.done = true;
                return (true, self.done);
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
        return IndirectAddressingTasks {
            fixed_addressing: true,
            done: false,
            step: IndirectStep::IndirectFetchLo,
            tgt_addr_lo: Byte::default(),
            tgt_addr_hi: Byte::default(),
        };
    }

    pub fn new_incorrect_addressing() -> Self {
        return IndirectAddressingTasks {
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

    fn tick(&mut self, cpu: &mut super::CPU) -> (bool, bool) {
        if self.done {
            return (false, self.done);
        }

        match self.step {
            IndirectStep::IndirectFetchLo => {
                self.tgt_addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.increment_program_counter();
                self.step = IndirectStep::IndirectFetchHi;

                return (true, false);
            }
            IndirectStep::IndirectFetchHi => {
                self.tgt_addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.increment_program_counter();
                if self.fixed_addressing {
                    self.step = IndirectStep::AddrFixing;
                } else {
                    self.step = IndirectStep::MemoryAccessLo;
                }

                return (true, false);
            }
            IndirectStep::AddrFixing => {
                self.step = IndirectStep::MemoryAccessLo;

                return (true, false);
            }
            IndirectStep::MemoryAccessLo => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let addr_lo = cpu.access_memory(addr);
                cpu.set_address_output_lo(addr_lo);

                if self.fixed_addressing {
                    self.step = IndirectStep::FixedMemoryAccessHi;
                } else {
                    self.step = IndirectStep::IncorrectMemoryAccessHi;
                }

                return (true, false);
            }
            IndirectStep::FixedMemoryAccessHi => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let addr_hi = cpu.access_memory(addr + 1);
                cpu.set_address_output_hi(addr_hi);

                self.done = true;
                return (true, self.done);
            }
            IndirectStep::IncorrectMemoryAccessHi => {
                let addr = Word::from_le_bytes([self.tgt_addr_lo, self.tgt_addr_hi]);
                let should_incorrectly_jump = self.tgt_addr_lo == 0xFF;
                let mut target_addr = addr + 1;
                if should_incorrectly_jump {
                    target_addr = addr & 0xFF00;
                };
                let addr_hi = cpu.access_memory(target_addr);
                cpu.set_address_output_hi(addr_hi);

                self.done = true;
                return (true, self.done);
            }
        }
    }
}

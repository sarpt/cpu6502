use crate::{consts::Word, cpu::tasks::Tasks};

use super::OffsetVariant;

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

    fn tick(&mut self, cpu: &mut super::CPU) -> bool {
        if self.done {
            return self.done;
        }

        match self.step {
            AbsoluteOffsetStep::MemoryAccessLo => {
                let addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_lo(addr_lo);
                cpu.increment_program_counter();
                self.step = AbsoluteOffsetStep::MemoryAccessHi;

                return false;
            }
            AbsoluteOffsetStep::MemoryAccessHi => {
                let addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_hi(addr_hi);
                cpu.increment_program_counter();
                self.step = AbsoluteOffsetStep::OffsetLo;

                return false;
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
                return self.done;
            }
            AbsoluteOffsetStep::OffsetHi => {
                let [lo, hi] = cpu.address_output.to_le_bytes();
                let new_hi = hi.wrapping_add(1);
                cpu.address_output = Word::from_le_bytes([lo, new_hi]);

                self.done = true;
                return self.done;
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

    fn tick(&mut self, cpu: &mut super::CPU) -> bool {
        if self.done {
            return self.done;
        }

        match self.step {
            AbsoluteStep::MemoryLo => {
                let addr_lo = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_lo(addr_lo);
                cpu.increment_program_counter();
                self.step = AbsoluteStep::MemoryHi;

                return false;
            }
            AbsoluteStep::MemoryHi => {
                let addr_hi = cpu.access_memory(cpu.program_counter);
                cpu.set_address_output_hi(addr_hi);
                cpu.increment_program_counter();

                self.done = true;
                return self.done;
            }
        }
    }
}

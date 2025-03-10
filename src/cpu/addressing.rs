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

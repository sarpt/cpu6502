use crate::{consts::Byte, cpu::tasks::Tasks};

use super::OffsetVariant;

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

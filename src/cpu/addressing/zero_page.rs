use crate::{
    consts::{Byte, Word},
    cpu::tasks::Tasks,
};

use super::{address::Address, AddressingTasks, OffsetVariant};

pub struct ZeroPageAddressingTasks {
    done: bool,
    addr: Address,
}

impl ZeroPageAddressingTasks {
    pub fn new() -> Self {
        return ZeroPageAddressingTasks {
            done: false,
            addr: Address::new(),
        };
    }
}

impl Tasks for ZeroPageAddressingTasks {
    fn done(&self) -> bool {
        return self.done;
    }

    fn tick(&mut self, cpu: &mut super::CPU) -> bool {
        if self.done {
            return self.done;
        }

        let addr: Byte = cpu.access_memory(cpu.program_counter);
        self.addr.set(addr);
        cpu.increment_program_counter();
        self.done = true;

        return self.done;
    }
}

impl AddressingTasks for ZeroPageAddressingTasks {
    fn address(&self) -> Option<Word> {
        self.addr.value()
    }
}

enum ZeroPageOffsetStep {
    ZeroPageAccess,
    Offset,
}

pub struct ZeroPageOffsetAddressingTasks {
    addr: Address,
    done: bool,
    step: ZeroPageOffsetStep,
    variant: OffsetVariant,
}

impl ZeroPageOffsetAddressingTasks {
    pub fn new_offset_by_x() -> Self {
        return ZeroPageOffsetAddressingTasks {
            addr: Address::new(),
            done: false,
            step: ZeroPageOffsetStep::ZeroPageAccess,
            variant: OffsetVariant::X,
        };
    }

    pub fn new_offset_by_y() -> Self {
        return ZeroPageOffsetAddressingTasks {
            addr: Address::new(),
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

    fn tick(&mut self, cpu: &mut super::CPU) -> bool {
        if self.done {
            return self.done;
        }

        match self.step {
            ZeroPageOffsetStep::ZeroPageAccess => {
                let addr: Byte = cpu.access_memory(cpu.program_counter);
                self.addr.set(addr);
                cpu.increment_program_counter();
                self.step = ZeroPageOffsetStep::Offset;

                return false;
            }
            ZeroPageOffsetStep::Offset => {
                let offset: Byte = match self.variant {
                    OffsetVariant::X => cpu.index_register_x.into(),
                    OffsetVariant::Y => cpu.index_register_y.into(),
                };
                let addr_output = self
                    .addr
                    .value()
                    .expect("unexpected lack of address at Offset step")
                    as Byte;
                let final_address = addr_output.wrapping_add(offset);
                self.addr.set(final_address);

                self.done = true;
                return self.done;
            }
        }
    }
}

impl AddressingTasks for ZeroPageOffsetAddressingTasks {
    fn address(&self) -> Option<crate::consts::Word> {
        self.addr.value()
    }
}

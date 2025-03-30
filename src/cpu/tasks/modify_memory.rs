use crate::{consts::Byte, cpu::CPU};

use super::Tasks;

enum ModificationVariant {
    Inc,
    Dec,
}

#[derive(PartialEq, PartialOrd)]
enum ModifyMemoryStep {
    Addressing,
    MemoryAccess,
    ValueModification,
    MemoryAndStatusWrite,
    Done,
}

pub struct ModifyMemoryTasks {
    variant: ModificationVariant,
    addr_tasks: Box<dyn Tasks>,
    step: ModifyMemoryStep,
    value: Byte,
}

impl ModifyMemoryTasks {
    pub fn new_inc(addr_tasks: Box<dyn Tasks>) -> Self {
        return ModifyMemoryTasks {
            variant: ModificationVariant::Inc,
            addr_tasks,
            step: ModifyMemoryStep::Addressing,
            value: Byte::default(),
        };
    }

    pub fn new_dec(addr_tasks: Box<dyn Tasks>) -> Self {
        return ModifyMemoryTasks {
            variant: ModificationVariant::Dec,
            addr_tasks,
            step: ModifyMemoryStep::Addressing,
            value: Byte::default(),
        };
    }
}

impl Tasks for ModifyMemoryTasks {
    fn done(&self) -> bool {
        self.step == ModifyMemoryStep::Done
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        match self.step {
            ModifyMemoryStep::Addressing => {
                let done = self.addr_tasks.tick(cpu);
                if done {
                    self.step = ModifyMemoryStep::MemoryAccess
                }

                return false;
            }
            ModifyMemoryStep::MemoryAccess => {
                self.value = cpu.access_memory(cpu.address_output);

                self.step = ModifyMemoryStep::ValueModification;
                return false;
            }
            ModifyMemoryStep::ValueModification => {
                match self.variant {
                    ModificationVariant::Inc => self.value = self.value.wrapping_add(1),
                    ModificationVariant::Dec => self.value = self.value.wrapping_sub(1),
                }

                self.step = ModifyMemoryStep::MemoryAndStatusWrite;
                return false;
            }
            ModifyMemoryStep::MemoryAndStatusWrite => {
                cpu.put_into_memory(cpu.address_output, self.value);
                cpu.set_status_of_value(self.value);

                self.step = ModifyMemoryStep::Done;
                return true;
            }
            ModifyMemoryStep::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

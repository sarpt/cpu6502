use crate::{consts::Byte, cpu::CPU};

use super::Tasks;

enum ModificationVariant {
    Inc,
    Dec,
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
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
    previous_value: Byte,
    value: Byte,
}

impl ModifyMemoryTasks {
    pub fn new_inc(addr_tasks: Box<dyn Tasks>) -> Self {
        return ModifyMemoryTasks {
            variant: ModificationVariant::Inc,
            addr_tasks,
            step: ModifyMemoryStep::Addressing,
            previous_value: Byte::default(),
            value: Byte::default(),
        };
    }

    pub fn new_dec(addr_tasks: Box<dyn Tasks>) -> Self {
        return ModifyMemoryTasks {
            variant: ModificationVariant::Dec,
            addr_tasks,
            step: ModifyMemoryStep::Addressing,
            previous_value: Byte::default(),
            value: Byte::default(),
        };
    }

    pub fn new_shift_left(addr_tasks: Box<dyn Tasks>) -> Self {
        return ModifyMemoryTasks {
            variant: ModificationVariant::ShiftLeft,
            addr_tasks,
            step: ModifyMemoryStep::Addressing,
            previous_value: Byte::default(),
            value: Byte::default(),
        };
    }

    pub fn new_shift_right(addr_tasks: Box<dyn Tasks>) -> Self {
        return ModifyMemoryTasks {
            variant: ModificationVariant::ShiftRight,
            addr_tasks,
            step: ModifyMemoryStep::Addressing,
            previous_value: Byte::default(),
            value: Byte::default(),
        };
    }

    pub fn new_rotate_left(addr_tasks: Box<dyn Tasks>) -> Self {
        return ModifyMemoryTasks {
            variant: ModificationVariant::RotateLeft,
            addr_tasks,
            step: ModifyMemoryStep::Addressing,
            previous_value: Byte::default(),
            value: Byte::default(),
        };
    }

    pub fn new_rotate_right(addr_tasks: Box<dyn Tasks>) -> Self {
        return ModifyMemoryTasks {
            variant: ModificationVariant::RotateRight,
            addr_tasks,
            step: ModifyMemoryStep::Addressing,
            previous_value: Byte::default(),
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
                self.previous_value = self.value;

                match self.variant {
                    ModificationVariant::Inc => self.value = self.value.wrapping_add(1),
                    ModificationVariant::Dec => self.value = self.value.wrapping_sub(1),
                    ModificationVariant::ShiftLeft => {
                        self.value = self.value << 1;
                    }
                    ModificationVariant::ShiftRight => {
                        self.value = self.value >> 1;
                    }
                    ModificationVariant::RotateLeft => {
                        let mod_value = self.value << 1;
                        if !cpu.processor_status.get_carry_flag() {
                            self.value = mod_value;
                        } else {
                            self.value = mod_value | 0b00000001;
                        }
                    }
                    ModificationVariant::RotateRight => {
                        let mod_value = self.value >> 1;
                        if !cpu.processor_status.get_carry_flag() {
                            self.value = mod_value;
                        } else {
                            self.value = mod_value | 0b10000000;
                        }
                    }
                }

                self.step = ModifyMemoryStep::MemoryAndStatusWrite;
                return false;
            }
            ModifyMemoryStep::MemoryAndStatusWrite => {
                cpu.put_into_memory(cpu.address_output, self.value);
                cpu.set_status_of_value(self.value);

                match self.variant {
                    ModificationVariant::ShiftLeft | ModificationVariant::RotateLeft => {
                        cpu.processor_status
                            .change_carry_flag(self.previous_value & 0b10000000 > 0);
                    }
                    ModificationVariant::ShiftRight | ModificationVariant::RotateRight => {
                        cpu.processor_status
                            .change_carry_flag(self.previous_value & 0b00000001 > 0);
                    }
                    _ => {}
                };

                self.step = ModifyMemoryStep::Done;
                return true;
            }
            ModifyMemoryStep::Done => {
                panic!("tick mustn't be called when done")
            }
        }
    }
}

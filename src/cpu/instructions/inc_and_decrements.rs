use std::rc::Rc;

use crate::{
    consts::Byte,
    cpu::{
        addressing::get_addressing_tasks, tasks::GenericTasks, AddressingMode, Registers, Tasks,
        CPU,
    },
};

fn decrement_cb(value: &u8) -> u8 {
    return value.wrapping_sub(1);
}

fn increment_cb(value: &u8) -> u8 {
    return value.wrapping_add(1);
}

fn decrement_memory(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);
    return Box::new(ModifyMemoryTasks::new_dec(addr_tasks));
}

fn decrement_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
    match register {
        Registers::IndexX | Registers::IndexY => {
            let mut tasks = GenericTasks::new();
            tasks.push(Rc::new(move |cpu: &mut CPU| {
                cpu.decrement_register(register);
            }));
            return Box::new(tasks);
        }
        _ => panic!("decrement_register used with incorrect register"),
    }
}

pub fn dec_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return decrement_memory(cpu, AddressingMode::ZeroPage);
}

pub fn dec_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return decrement_memory(cpu, AddressingMode::ZeroPageX);
}

pub fn dec_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return decrement_memory(cpu, AddressingMode::Absolute);
}

pub fn dec_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return decrement_memory(cpu, AddressingMode::AbsoluteX);
}

pub fn dex_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return decrement_register(cpu, Registers::IndexX);
}

pub fn dey_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return decrement_register(cpu, Registers::IndexY);
}

fn increment_memory(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);
    return Box::new(ModifyMemoryTasks::new_inc(addr_tasks));
}

fn increment_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
    match register {
        Registers::IndexX | Registers::IndexY => {
            let mut tasks = GenericTasks::new();
            tasks.push(Rc::new(move |cpu: &mut CPU| {
                cpu.increment_register(register);
            }));
            return Box::new(tasks);
        }
        _ => panic!("increment_register used with incorrect register"),
    }
}

pub fn inc_zp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return increment_memory(cpu, AddressingMode::ZeroPage);
}

pub fn inc_zpx(cpu: &mut CPU) -> Box<dyn Tasks> {
    return increment_memory(cpu, AddressingMode::ZeroPageX);
}

pub fn inc_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return increment_memory(cpu, AddressingMode::Absolute);
}

pub fn inc_ax(cpu: &mut CPU) -> Box<dyn Tasks> {
    return increment_memory(cpu, AddressingMode::AbsoluteX);
}

pub fn inx_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return increment_register(cpu, Registers::IndexX);
}

pub fn iny_im(cpu: &mut CPU) -> Box<dyn Tasks> {
    return increment_register(cpu, Registers::IndexY);
}

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

struct ModifyMemoryTasks {
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
                    ModificationVariant::Inc => self.value = increment_cb(&self.value),
                    ModificationVariant::Dec => self.value = decrement_cb(&self.value),
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

#[cfg(test)]
mod tests;

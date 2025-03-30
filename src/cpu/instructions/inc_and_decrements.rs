use std::rc::Rc;

use crate::cpu::{
    addressing::get_addressing_tasks,
    tasks::{modify_memory::ModifyMemoryTasks, GenericTasks},
    AddressingMode, Registers, Tasks, CPU,
};

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

#[cfg(test)]
mod tests;

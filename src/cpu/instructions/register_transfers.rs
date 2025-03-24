use std::rc::Rc;

use crate::cpu::{tasks::GenericTasks, Registers, Tasks, CPU};

pub fn tax(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::Accumulator, Registers::IndexX);
    }));

    return Box::new(tasks);
}

pub fn txa(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::IndexX, Registers::Accumulator);
    }));

    return Box::new(tasks);
}

pub fn tay(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::Accumulator, Registers::IndexY);
    }));

    return Box::new(tasks);
}

pub fn tya(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::IndexY, Registers::Accumulator);
    }));

    return Box::new(tasks);
}

#[cfg(test)]
mod tests;

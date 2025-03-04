use std::rc::Rc;

use crate::cpu::{Registers, TaskCycleVariant, Tasks, CPU};

pub fn tax(_cpu: &mut CPU) -> Tasks {
    let mut tasks: Tasks = Vec::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::Accumulator, Registers::IndexX);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

pub fn txa(_cpu: &mut CPU) -> Tasks {
    let mut tasks: Tasks = Vec::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::IndexX, Registers::Accumulator);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

pub fn tay(_cpu: &mut CPU) -> Tasks {
    let mut tasks: Tasks = Vec::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::Accumulator, Registers::IndexY);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

pub fn tya(_cpu: &mut CPU) -> Tasks {
    let mut tasks: Tasks = Vec::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::IndexY, Registers::Accumulator);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

#[cfg(test)]
mod tests;

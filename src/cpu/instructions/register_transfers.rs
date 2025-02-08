use std::rc::Rc;

use crate::cpu::{Registers, ScheduledTask, TaskCycleVariant, CPU};

pub fn tax(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::Accumulator, Registers::IndexX);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn txa(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::IndexX, Registers::Accumulator);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn tay(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::Accumulator, Registers::IndexY);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn tya(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::IndexY, Registers::Accumulator);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

#[cfg(test)]
mod tests;

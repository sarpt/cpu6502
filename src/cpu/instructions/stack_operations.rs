use std::rc::Rc;

use crate::cpu::{Registers, ScheduledTask, TaskCycleVariant, CPU};

fn push_register(cpu: &mut CPU, register: Registers) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(|cpu| {
        cpu.dummy_fetch();

        return TaskCycleVariant::Full;
    }));

    cycles.push(Rc::new(move |cpu| {
        let val = cpu.get_register(register);
        cpu.push_byte_to_stack(val);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn pha(cpu: &mut CPU) {
    push_register(cpu, Registers::Accumulator);
}

pub fn php(cpu: &mut CPU) {
    push_register(cpu, Registers::ProcessorStatus);
}

fn pull_register(cpu: &mut CPU, register: Registers) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(|cpu| {
        cpu.dummy_fetch();

        return TaskCycleVariant::Full;
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    cycles.push(Rc::new(|_| TaskCycleVariant::Full));

    cycles.push(Rc::new(move |cpu| {
        let value = cpu.pop_byte_from_stack();
        cpu.set_register(register, value);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn pla(cpu: &mut CPU) {
    pull_register(cpu, Registers::Accumulator);
}

pub fn plp(cpu: &mut CPU) {
    pull_register(cpu, Registers::ProcessorStatus);
}

pub fn tsx(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::StackPointer, Registers::IndexX);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

pub fn txs(cpu: &mut CPU) {
    let mut cycles: Vec<ScheduledTask> = Vec::new();
    cycles.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::IndexX, Registers::StackPointer);

        return TaskCycleVariant::Full;
    }));

    cpu.schedule_instruction(cycles);
}

#[cfg(test)]
mod tests;

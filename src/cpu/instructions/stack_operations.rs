use std::rc::Rc;

use crate::cpu::{Registers, TaskCycleVariant, Tasks, CPU};

fn push_register(_cpu: &mut CPU, register: Registers) -> Tasks {
    let mut tasks: Tasks = Vec::new();
    tasks.push(Rc::new(|cpu| {
        cpu.dummy_fetch();

        return TaskCycleVariant::Full;
    }));

    tasks.push(Rc::new(move |cpu| {
        let val = cpu.get_register(register);
        cpu.push_byte_to_stack(val);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

pub fn pha(cpu: &mut CPU) -> Tasks {
    return push_register(cpu, Registers::Accumulator);
}

pub fn php(cpu: &mut CPU) -> Tasks {
    return push_register(cpu, Registers::ProcessorStatus);
}

fn pull_register(_cpu: &mut CPU, register: Registers) -> Tasks {
    let mut tasks: Tasks = Vec::new();
    tasks.push(Rc::new(|cpu| {
        cpu.dummy_fetch();

        return TaskCycleVariant::Full;
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    tasks.push(Rc::new(|_| TaskCycleVariant::Full));

    tasks.push(Rc::new(move |cpu| {
        let value = cpu.pop_byte_from_stack();
        cpu.set_register(register, value);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

pub fn pla(cpu: &mut CPU) -> Tasks {
    return pull_register(cpu, Registers::Accumulator);
}

pub fn plp(cpu: &mut CPU) -> Tasks {
    return pull_register(cpu, Registers::ProcessorStatus);
}

pub fn tsx(_cpu: &mut CPU) -> Tasks {
    let mut tasks: Tasks = Vec::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::StackPointer, Registers::IndexX);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

pub fn txs(_cpu: &mut CPU) -> Tasks {
    let mut tasks: Tasks = Vec::new();
    tasks.push(Rc::new(|cpu| {
        cpu.transfer_registers(Registers::IndexX, Registers::StackPointer);

        return TaskCycleVariant::Full;
    }));

    return tasks;
}

#[cfg(test)]
mod tests;

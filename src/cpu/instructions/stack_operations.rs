use std::rc::Rc;

use crate::cpu::{
    tasks::{transfer_register::TransferRegistersTasks, GenericTasks},
    Registers, Tasks, CPU,
};

fn push_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.dummy_fetch();
    }));

    tasks.push(Rc::new(move |cpu| {
        let val = cpu.get_register(register);
        cpu.push_byte_to_stack(val);
    }));

    return Box::new(tasks);
}

pub fn pha(cpu: &mut CPU) -> Box<dyn Tasks> {
    return push_register(cpu, Registers::Accumulator);
}

pub fn php(cpu: &mut CPU) -> Box<dyn Tasks> {
    return push_register(cpu, Registers::ProcessorStatus);
}

fn pull_register(_cpu: &mut CPU, register: Registers) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.dummy_fetch();
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    tasks.push(Rc::new(|_| {}));

    tasks.push(Rc::new(move |cpu| {
        let value = cpu.pop_byte_from_stack();
        cpu.set_register(register, value);
    }));

    return Box::new(tasks);
}

pub fn pla(cpu: &mut CPU) -> Box<dyn Tasks> {
    return pull_register(cpu, Registers::Accumulator);
}

pub fn plp(cpu: &mut CPU) -> Box<dyn Tasks> {
    return pull_register(cpu, Registers::ProcessorStatus);
}

pub fn tsx(_cpu: &mut CPU) -> Box<dyn Tasks> {
    return Box::new(TransferRegistersTasks::new(
        Registers::StackPointer,
        Registers::IndexX,
    ));
}

pub fn txs(_cpu: &mut CPU) -> Box<dyn Tasks> {
    return Box::new(TransferRegistersTasks::new(
        Registers::IndexX,
        Registers::StackPointer,
    ));
}

#[cfg(test)]
mod tests;

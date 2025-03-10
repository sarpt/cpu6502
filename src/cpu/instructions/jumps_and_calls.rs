use std::rc::Rc;

use crate::cpu::{tasks::GenericTasks, AddressingMode, TaskCycleVariant, Tasks, CPU};

pub fn jsr_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    let addr_tasks = cpu.get_address(AddressingMode::Absolute);
    let mut tasks = GenericTasks::new_dependent(addr_tasks);

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let [_, ret_program_counter_hi] = cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
        cpu.push_byte_to_stack(ret_program_counter_hi);

        return TaskCycleVariant::Full;
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let [ret_program_counter_lo, _] = cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
        cpu.push_byte_to_stack(ret_program_counter_lo);

        return TaskCycleVariant::Full;
    }));

    tasks.push(Rc::new(|cpu| {
        cpu.program_counter = cpu.address_output;

        return TaskCycleVariant::Full;
    }));

    return Box::new(tasks);
}

pub fn rts(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.dummy_fetch();

        return TaskCycleVariant::Full;
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    tasks.push(Rc::new(|_| TaskCycleVariant::Full));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let lo = cpu.pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);

        return TaskCycleVariant::Full;
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let hi = cpu.pop_byte_from_stack();
        cpu.set_program_counter_hi(hi);

        return TaskCycleVariant::Full;
    }));

    tasks.push(Rc::new(|cpu| {
        cpu.increment_program_counter();

        return TaskCycleVariant::Full;
    }));

    return Box::new(tasks);
}

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = cpu.get_address(addr_mode);
    let mut tasks = GenericTasks::new_dependent(addr_tasks);

    tasks.push(Rc::new(|cpu| {
        cpu.program_counter = cpu.address_output;

        return TaskCycleVariant::Partial;
    }));

    return Box::new(tasks);
}

pub fn jmp_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return jmp(cpu, AddressingMode::Absolute);
}

pub fn jmp_in(cpu: &mut CPU) -> Box<dyn Tasks> {
    return jmp(cpu, AddressingMode::Indirect);
}

#[cfg(test)]
mod tests;

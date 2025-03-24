use std::rc::Rc;

use crate::{
    consts::BRK_INTERRUPT_VECTOR,
    cpu::{tasks::GenericTasks, ChipVariant, Tasks, CPU},
};

pub fn nop(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.increment_program_counter();
    }));

    return Box::new(tasks);
}

pub fn brk(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.access_memory(cpu.program_counter); // fetch and discard
        cpu.increment_program_counter();
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.get_program_counter_hi());
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.get_program_counter_lo());
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.push_byte_to_stack(cpu.processor_status.into());
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let lo = cpu.access_memory(BRK_INTERRUPT_VECTOR);
        cpu.set_program_counter_lo(lo);
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let hi = cpu.access_memory(BRK_INTERRUPT_VECTOR + 1);
        cpu.set_program_counter_hi(hi);

        cpu.processor_status.change_break_flag(true);
        if cpu.chip_variant != ChipVariant::NMOS {
            cpu.processor_status.change_decimal_mode_flag(false);
        }
    }));

    return Box::new(tasks);
}

pub fn rti(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.dummy_fetch();
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    tasks.push(Rc::new(|_: &mut CPU| {}));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        cpu.processor_status = cpu.pop_byte_from_stack().into();
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let lo = cpu.pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let hi = cpu.pop_byte_from_stack();
        cpu.set_program_counter_hi(hi);
    }));

    return Box::new(tasks);
}

#[cfg(test)]
mod tests;

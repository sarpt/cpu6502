use std::rc::Rc;

use crate::cpu::{
    addressing::get_addressing_tasks, tasks::GenericTasks, AddressingMode, Tasks, CPU,
};

pub fn jsr_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, AddressingMode::Absolute);
    let mut tasks = GenericTasks::new_dependent(addr_tasks);

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let [_, ret_program_counter_hi] = cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
        cpu.push_byte_to_stack(ret_program_counter_hi);
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let [ret_program_counter_lo, _] = cpu.program_counter.clone().wrapping_sub(1).to_le_bytes();
        cpu.push_byte_to_stack(ret_program_counter_lo);
    }));

    tasks.push(Rc::new(|cpu| {
        cpu.program_counter = cpu.address_output;
    }));

    return Box::new(tasks);
}

pub fn rts(_cpu: &mut CPU) -> Box<dyn Tasks> {
    let mut tasks = GenericTasks::new();
    tasks.push(Rc::new(|cpu| {
        cpu.dummy_fetch();
    }));

    // dummy tick, simulate separate stack pointer decrement
    // second cycle involves decrement of the stack pointer but poping byte from stack in third cycle does it in a single fn call
    // TODO: dont create dummy cycles, instead of decrementing and poping values in one call separate them into respective cycles
    tasks.push(Rc::new(|_| {}));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let lo = cpu.pop_byte_from_stack();
        cpu.set_program_counter_lo(lo);
    }));

    tasks.push(Rc::new(|cpu: &mut CPU| {
        let hi = cpu.pop_byte_from_stack();
        cpu.set_program_counter_hi(hi);
    }));

    tasks.push(Rc::new(|cpu| {
        cpu.increment_program_counter();
    }));

    return Box::new(tasks);
}

pub struct JmpTasks {
    addressing_tasks: Box<dyn Tasks>,
}

impl JmpTasks {
    fn new(addressing_tasks: Box<dyn Tasks>) -> Self {
        return JmpTasks { addressing_tasks };
    }
}

impl Tasks for JmpTasks {
    fn done(&self) -> bool {
        self.addressing_tasks.done()
    }

    fn tick(&mut self, cpu: &mut CPU) -> bool {
        if self.addressing_tasks.done() {
            return true;
        }

        let done = self.addressing_tasks.tick(cpu);

        if done {
            cpu.program_counter = cpu.address_output;
        }

        return done;
    }
}

fn jmp(cpu: &mut CPU, addr_mode: AddressingMode) -> Box<dyn Tasks> {
    let addr_tasks = get_addressing_tasks(&cpu, addr_mode);

    return Box::new(JmpTasks::new(addr_tasks));
}

pub fn jmp_a(cpu: &mut CPU) -> Box<dyn Tasks> {
    return jmp(cpu, AddressingMode::Absolute);
}

pub fn jmp_in(cpu: &mut CPU) -> Box<dyn Tasks> {
    return jmp(cpu, AddressingMode::Indirect);
}

#[cfg(test)]
mod tests;
